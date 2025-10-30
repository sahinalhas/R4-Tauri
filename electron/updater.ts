import { BrowserWindow } from 'electron';
import log from './logger';

export function setupAutoUpdater(mainWindow: BrowserWindow): void {
  try {
    const { autoUpdater } = require('electron-updater');
    
    autoUpdater.logger = log;
    (autoUpdater.logger as any).transports.file.level = 'info';

    autoUpdater.on('checking-for-update', () => {
      log.info('Checking for updates...');
    });

    autoUpdater.on('update-available', (info: any) => {
      log.info('Update available:', info.version);
      mainWindow.webContents.send('update:available', {
        version: info.version,
        releaseDate: info.releaseDate,
      });
    });

    autoUpdater.on('update-not-available', (info: any) => {
      log.info('Update not available, current version:', info.version);
    });

    autoUpdater.on('error', (error: Error) => {
      log.error('Auto-updater error:', error);
      mainWindow.webContents.send('update:error', {
        message: error.message,
      });
    });

    autoUpdater.on('download-progress', (progressObj: any) => {
      const message = `Download speed: ${progressObj.bytesPerSecond} - Downloaded ${progressObj.percent}%`;
      log.info(message);
      mainWindow.webContents.send('update:downloadProgress', {
        percent: progressObj.percent,
        transferred: progressObj.transferred,
        total: progressObj.total,
      });
    });

    autoUpdater.on('update-downloaded', (info: any) => {
      log.info('Update downloaded, version:', info.version);
      mainWindow.webContents.send('update:downloaded', {
        version: info.version,
      });
    });

    setTimeout(() => {
      autoUpdater.checkForUpdates();
    }, 5000);

    setInterval(() => {
      autoUpdater.checkForUpdates();
    }, 2 * 60 * 60 * 1000);

    log.info('Auto-updater initialized successfully');
  } catch (error) {
    log.warn('electron-updater not available, auto-update disabled');
  }
}

export function quitAndInstall(): void {
  try {
    const { autoUpdater } = require('electron-updater');
    autoUpdater.quitAndInstall(false, true);
    log.info('Installing update and quitting...');
  } catch (error) {
    log.error('Failed to quit and install:', error);
  }
}
