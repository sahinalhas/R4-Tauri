import { Tray, Menu, BrowserWindow, app, nativeImage, NativeImage, MenuItemConstructorOptions } from 'electron';
import path from 'path';
import log from './logger';

let tray: Tray | null = null;

export function createSystemTray(mainWindow: BrowserWindow): Tray {
  const iconPath = path.join(__dirname, '../../build/tray.png');
  
  let trayIcon: NativeImage;
  try {
    trayIcon = nativeImage.createFromPath(iconPath);
    if (trayIcon.isEmpty()) {
      log.warn('Tray icon not found, using empty icon');
      trayIcon = nativeImage.createEmpty();
    }
  } catch (error) {
    log.error('Failed to load tray icon:', error);
    trayIcon = nativeImage.createEmpty();
  }

  tray = new Tray(trayIcon);
  tray.setToolTip('Rehber360 - Öğrenci Rehberlik Sistemi');

  const contextMenu = createTrayMenu(mainWindow);
  tray.setContextMenu(contextMenu);

  tray.on('click', () => {
    if (mainWindow.isVisible()) {
      mainWindow.hide();
      log.debug('Tray: Window hidden');
    } else {
      showWindow(mainWindow);
      log.debug('Tray: Window shown');
    }
  });

  tray.on('double-click', () => {
    showWindow(mainWindow);
    log.debug('Tray: Window shown (double-click)');
  });

  log.info('System tray created successfully');
  return tray;
}

function createTrayMenu(mainWindow: BrowserWindow): Menu {
  const template: MenuItemConstructorOptions[] = [
    {
      label: '📊 Rehber360',
      enabled: false,
    },
    {
      type: 'separator',
    },
    {
      label: 'Göster',
      click: () => {
        showWindow(mainWindow);
        log.debug('Tray Menu: Show window');
      },
    },
    {
      type: 'separator',
    },
    {
      label: 'Hızlı Erişim',
      submenu: [
        {
          label: 'Ana Sayfa',
          click: () => {
            showWindow(mainWindow);
            mainWindow.webContents.send('menu:navigate', '/');
            log.debug('Tray Menu: Navigate to Home');
          },
        },
        {
          label: 'Öğrenciler',
          click: () => {
            showWindow(mainWindow);
            mainWindow.webContents.send('menu:navigate', '/students');
            log.debug('Tray Menu: Navigate to Students');
          },
        },
        {
          label: 'Sınavlar',
          click: () => {
            showWindow(mainWindow);
            mainWindow.webContents.send('menu:navigate', '/exams');
            log.debug('Tray Menu: Navigate to Exams');
          },
        },
        {
          label: 'Raporlar',
          click: () => {
            showWindow(mainWindow);
            mainWindow.webContents.send('menu:navigate', '/reports');
            log.debug('Tray Menu: Navigate to Reports');
          },
        },
      ],
    },
    {
      type: 'separator',
    },
    {
      label: 'Ayarlar',
      click: () => {
        showWindow(mainWindow);
        mainWindow.webContents.send('menu:settings');
        log.debug('Tray Menu: Settings');
      },
    },
    {
      type: 'separator',
    },
    {
      label: 'Çıkış',
      click: () => {
        app.quit();
      },
    },
  ];

  return Menu.buildFromTemplate(template);
}

function showWindow(mainWindow: BrowserWindow): void {
  if (mainWindow.isMinimized()) {
    mainWindow.restore();
  }
  mainWindow.show();
  mainWindow.focus();
}

export function updateTrayMenu(mainWindow: BrowserWindow): void {
  if (tray) {
    const contextMenu = createTrayMenu(mainWindow);
    tray.setContextMenu(contextMenu);
    log.debug('Tray menu updated');
  }
}

export function destroyTray(): void {
  if (tray) {
    tray.destroy();
    tray = null;
    log.info('System tray destroyed');
  }
}

export function getTray(): Tray | null {
  return tray;
}

export function displayBalloon(title: string, content: string): void {
  if (tray) {
    tray.displayBalloon({
      title,
      content,
      icon: nativeImage.createEmpty(),
    });
    log.debug(`Tray balloon: ${title}`);
  }
}
