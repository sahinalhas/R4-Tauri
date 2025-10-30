import { app, BrowserWindow, ipcMain, session } from 'electron';
import path from 'path';
import { spawn, ChildProcess } from 'child_process';
import isDev from 'electron-is-dev';
import log from './logger';
import store, { getStoreValue, setStoreValue } from './store';
import { setupApplicationMenu } from './menu';
import { createSystemTray, destroyTray } from './tray';
import { setupAutoUpdater } from './updater';
import { setupDatabaseHandlers } from './ipc/database';
import { setupFileHandlers } from './ipc/file';
import { setupWindowHandlers } from './ipc/window';
import { setupNotificationHandlers } from './ipc/notifications';

let mainWindow: BrowserWindow | null = null;
let serverProcess: ChildProcess | null = null;
let serverPort: number = 3000;
let isQuitting = false;

function findAvailablePort(): number {
  return 3000 + Math.floor(Math.random() * 6000);
}

async function startBackend(): Promise<void> {
  return new Promise((resolve, reject) => {
    serverPort = findAvailablePort();
    
    const serverScript = isDev
      ? path.join(__dirname, '../../dist/server/node-build.mjs')
      : path.join(process.resourcesPath, 'dist/server/node-build.mjs');

    log.info(`Starting backend server on port ${serverPort}`);
    log.info(`Server script path: ${serverScript}`);

    serverProcess = spawn('node', [serverScript], {
      env: {
        ...process.env,
        PORT: serverPort.toString(),
        NODE_ENV: isDev ? 'development' : 'production',
      },
      stdio: ['ignore', 'pipe', 'pipe'],
    });

    serverProcess.stdout?.on('data', (data) => {
      const output = data.toString();
      log.info(`[Backend] ${output.trim()}`);
      
      if (output.includes('running on port') || output.includes('Server started')) {
        log.info('Backend server started successfully');
        resolve();
      }
    });

    serverProcess.stderr?.on('data', (data) => {
      log.error(`[Backend Error] ${data.toString().trim()}`);
    });

    serverProcess.on('error', (error) => {
      log.error('Backend process error:', error);
      reject(error);
    });

    serverProcess.on('exit', (code) => {
      log.info(`Backend process exited with code ${code}`);
      serverProcess = null;
    });

    setTimeout(() => {
      if (serverProcess && !serverProcess.killed) {
        log.info('Backend server initialization timeout reached, assuming started');
        resolve();
      } else {
        reject(new Error('Backend server failed to start'));
      }
    }, 5000);
  });
}

function stopBackend(): void {
  if (serverProcess && !serverProcess.killed) {
    log.info('Stopping backend server...');
    serverProcess.kill('SIGTERM');

    setTimeout(() => {
      if (serverProcess && !serverProcess.killed) {
        log.warn('Force killing backend server');
        serverProcess.kill('SIGKILL');
      }
    }, 5000);
  }
}

function setupIpcHandlers(): void {
  ipcMain.handle('app:getVersion', () => {
    return app.getVersion();
  });

  ipcMain.handle('app:getPlatform', () => {
    return process.platform;
  });

  ipcMain.handle('server:getPort', () => {
    return serverPort;
  });

  ipcMain.handle('app:getPath', (event, name: string) => {
    return app.getPath(name as any);
  });

  ipcMain.handle('update:quitAndInstall', () => {
    try {
      const { quitAndInstall } = require('./updater');
      quitAndInstall();
      return true;
    } catch (error) {
      log.error('Failed to quit and install:', error);
      return false;
    }
  });

  setupDatabaseHandlers();
  setupFileHandlers();
  
  if (mainWindow) {
    setupWindowHandlers(mainWindow);
    setupNotificationHandlers(mainWindow);
  }

  log.info('IPC handlers registered successfully');
}

function setupSecurityPolicy(): void {
  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    callback({
      responseHeaders: {
        ...details.responseHeaders,
        'Content-Security-Policy': [
          isDev
            ? "default-src 'self' 'unsafe-inline' 'unsafe-eval' http://localhost:* ws://localhost:*"
            : "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:;",
        ],
      },
    });
  });

  log.info('Security policy configured');
}

function saveWindowState(): void {
  if (mainWindow) {
    const bounds = mainWindow.getBounds();
    const isMaximized = mainWindow.isMaximized();
    
    setStoreValue('windowBounds', {
      ...bounds,
      isMaximized,
    });
    
    log.debug('Window state saved');
  }
}

function restoreWindowState(): { x: number; y: number; width: number; height: number } {
  const savedBounds = getStoreValue('windowBounds');
  return {
    x: savedBounds.x || 0,
    y: savedBounds.y || 0,
    width: savedBounds.width || 1280,
    height: savedBounds.height || 800,
  };
}

async function createWindow(): Promise<void> {
  const bounds = restoreWindowState();
  const isMaximized = getStoreValue('windowBounds').isMaximized || false;

  mainWindow = new BrowserWindow({
    ...bounds,
    minWidth: 1024,
    minHeight: 768,
    frame: true,
    backgroundColor: '#ffffff',
    show: false,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
      webSecurity: true,
      devTools: isDev,
    },
  });

  if (isMaximized) {
    mainWindow.maximize();
  }

  const startURL = isDev
    ? 'http://localhost:5000'
    : `file://${path.join(__dirname, '../spa/index.html')}`;

  mainWindow.loadURL(startURL);

  mainWindow.once('ready-to-show', () => {
    mainWindow?.show();
    log.info('Window shown');
  });

  if (isDev) {
    mainWindow.webContents.openDevTools();
  }

  mainWindow.on('close', (event) => {
    const minimizeToTray = getStoreValue('minimizeToTray');
    
    if (minimizeToTray && !isQuitting) {
      event.preventDefault();
      mainWindow?.hide();
      log.debug('Window hidden (minimize to tray)');
    } else {
      saveWindowState();
    }
  });

  mainWindow.on('closed', () => {
    mainWindow = null;
    log.info('Window closed');
  });

  mainWindow.on('resize', () => {
    if (!mainWindow?.isMaximized()) {
      saveWindowState();
    }
  });

  mainWindow.on('move', () => {
    if (!mainWindow?.isMaximized()) {
      saveWindowState();
    }
  });

  setupApplicationMenu(mainWindow, isDev);
  createSystemTray(mainWindow);
  setupAutoUpdater(mainWindow);

  log.info('Rehber360 Electron App window created');
}

app.on('before-quit', () => {
  isQuitting = true;
  saveWindowState();
  stopBackend();
  destroyTray();
  log.info('Application is quitting');
});

app.whenReady().then(async () => {
  log.info('Electron app is ready');
  
  setupSecurityPolicy();
  setupIpcHandlers();

  try {
    if (!isDev) {
      await startBackend();
    }
  } catch (error) {
    log.error('Failed to start backend:', error);
  }

  await createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    } else if (mainWindow) {
      mainWindow.show();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

process.on('uncaughtException', (error) => {
  log.error('Uncaught exception:', error);
});

process.on('unhandledRejection', (reason) => {
  log.error('Unhandled rejection:', reason);
});
