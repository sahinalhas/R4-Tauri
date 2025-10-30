import { ipcMain, Notification, BrowserWindow } from 'electron';
import log from '../logger';
import { getStoreValue, setStoreValue, NotificationSettings } from '../store';

export interface NotificationOptions {
  title: string;
  body: string;
  type?: 'info' | 'warning' | 'error' | 'success';
  urgency?: 'low' | 'normal' | 'critical';
  silent?: boolean;
  actions?: Array<{ type: string; text: string }>;
}

function createAndShowNotification(
  mainWindow: BrowserWindow,
  options: NotificationOptions
): boolean {
  try {
    if (!Notification.isSupported()) {
      log.warn('Notifications are not supported on this platform');
      return false;
    }

    const notification = new Notification({
      title: options.title,
      body: options.body,
      silent: options.silent ?? false,
      urgency: options.urgency ?? 'normal',
      timeoutType: 'default',
    });

    notification.on('click', () => {
      if (mainWindow.isMinimized()) {
        mainWindow.restore();
      }
      mainWindow.show();
      mainWindow.focus();

      mainWindow.webContents.send('notification:clicked', {
        title: options.title,
        type: options.type,
      });

      log.debug(`Notification clicked: ${options.title}`);
    });

    notification.on('close', () => {
      log.debug(`Notification closed: ${options.title}`);
    });

    notification.show();
    log.info(`Notification shown: ${options.title}`);
    return true;
  } catch (error) {
    log.error('Show notification failed:', error);
    return false;
  }
}

export function setupNotificationHandlers(mainWindow: BrowserWindow): void {
  ipcMain.handle('notification:show', async (event, options: NotificationOptions) => {
    return createAndShowNotification(mainWindow, options);
  });

  ipcMain.handle('notification:isSupported', () => {
    return Notification.isSupported();
  });

  ipcMain.handle('notification:getSettings', () => {
    return getStoreValue('notifications');
  });

  ipcMain.handle('notification:updateSettings', (event, settings: Partial<NotificationSettings>) => {
    try {
      const currentSettings = getStoreValue('notifications');
      const updatedSettings = { ...currentSettings, ...settings };
      setStoreValue('notifications', updatedSettings);
      log.info('Notification settings updated');
      return updatedSettings;
    } catch (error) {
      log.error('Update notification settings failed:', error);
      throw error;
    }
  });

  ipcMain.handle('notification:showRiskAlert', async (event, studentName: string, riskLevel: string) => {
    const settings = getStoreValue('notifications');
    
    if (!settings.riskStudents) {
      return false;
    }

    return createAndShowNotification(mainWindow, {
      title: 'Risk Öğrenci Uyarısı',
      body: `${studentName} - ${riskLevel} risk seviyesi tespit edildi`,
      type: 'warning',
      urgency: 'critical',
    });
  });

  ipcMain.handle('notification:showMissingData', async (event, message: string) => {
    const settings = getStoreValue('notifications');
    
    if (!settings.missingData) {
      return false;
    }

    return createAndShowNotification(mainWindow, {
      title: 'Eksik Veri Uyarısı',
      body: message,
      type: 'warning',
      urgency: 'normal',
    });
  });

  ipcMain.handle('notification:showDailyReport', async (event, message: string) => {
    const settings = getStoreValue('notifications');
    
    if (!settings.dailyReports) {
      return false;
    }

    return createAndShowNotification(mainWindow, {
      title: 'Günlük Rapor Hazır',
      body: message,
      type: 'info',
      urgency: 'low',
    });
  });
}
