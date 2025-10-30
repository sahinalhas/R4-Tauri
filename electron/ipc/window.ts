import { ipcMain, BrowserWindow } from 'electron';
import log from '../logger';
import { getStoreValue } from '../store';

export function setupWindowHandlers(mainWindow: BrowserWindow): void {
  ipcMain.handle('window:minimize', () => {
    try {
      mainWindow.minimize();
      log.debug('Window minimized');
    } catch (error) {
      log.error('Window minimize failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:maximize', () => {
    try {
      if (mainWindow.isMaximized()) {
        mainWindow.unmaximize();
        log.debug('Window unmaximized');
      } else {
        mainWindow.maximize();
        log.debug('Window maximized');
      }
    } catch (error) {
      log.error('Window maximize/unmaximize failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:close', () => {
    try {
      const minimizeToTray = getStoreValue('minimizeToTray') ?? true;
      
      if (minimizeToTray) {
        mainWindow.hide();
        log.debug('Window hidden (minimize to tray)');
      } else {
        mainWindow.close();
        log.debug('Window closed');
      }
    } catch (error) {
      log.error('Window close failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:isMaximized', () => {
    return mainWindow.isMaximized();
  });

  ipcMain.handle('window:isMinimized', () => {
    return mainWindow.isMinimized();
  });

  ipcMain.handle('window:isFullScreen', () => {
    return mainWindow.isFullScreen();
  });

  ipcMain.handle('window:toggleFullScreen', () => {
    try {
      const isFullScreen = mainWindow.isFullScreen();
      mainWindow.setFullScreen(!isFullScreen);
      log.debug(`Full screen toggled: ${!isFullScreen}`);
      return !isFullScreen;
    } catch (error) {
      log.error('Toggle full screen failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:show', () => {
    try {
      mainWindow.show();
      if (mainWindow.isMinimized()) {
        mainWindow.restore();
      }
      mainWindow.focus();
      log.debug('Window shown and focused');
    } catch (error) {
      log.error('Window show failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:getBounds', () => {
    return mainWindow.getBounds();
  });

  ipcMain.handle('window:setBounds', (event, bounds: { x?: number; y?: number; width?: number; height?: number }) => {
    try {
      mainWindow.setBounds(bounds);
      log.debug('Window bounds set:', bounds);
    } catch (error) {
      log.error('Set window bounds failed:', error);
      throw error;
    }
  });

  ipcMain.handle('window:center', () => {
    try {
      mainWindow.center();
      log.debug('Window centered');
    } catch (error) {
      log.error('Window center failed:', error);
      throw error;
    }
  });

  mainWindow.on('maximize', () => {
    mainWindow.webContents.send('window:maximized', true);
  });

  mainWindow.on('unmaximize', () => {
    mainWindow.webContents.send('window:maximized', false);
  });

  mainWindow.on('enter-full-screen', () => {
    mainWindow.webContents.send('window:fullscreen', true);
  });

  mainWindow.on('leave-full-screen', () => {
    mainWindow.webContents.send('window:fullscreen', false);
  });

  mainWindow.on('blur', () => {
    mainWindow.webContents.send('window:focused', false);
  });

  mainWindow.on('focus', () => {
    mainWindow.webContents.send('window:focused', true);
  });
}
