/**
 * Unified Notification Utility
 * 
 * Provides consistent notification interface across Electron and Web platforms
 * - Electron: Uses native OS notifications
 * - Web: Uses toast notifications (Sonner)
 */

import { toast } from 'sonner';
import { isElectron, getElectronAPI } from './electron';

export interface NotificationOptions {
  title: string;
  message: string;
  type?: 'info' | 'success' | 'warning' | 'error';
  duration?: number;
}

/**
 * Show notification using platform-appropriate method
 */
export async function showNotification(options: NotificationOptions): Promise<void> {
  const { title, message, type = 'info' } = options;

  if (isElectron()) {
    const api = getElectronAPI();
    if (api) {
      try {
        await api.showNotification({
          title,
          body: message,
          type,
        });
        return;
      } catch (error) {
        console.error('Failed to show Electron notification:', error);
      }
    }
  }

  switch (type) {
    case 'success':
      toast.success(title, { description: message });
      break;
    case 'error':
      toast.error(title, { description: message });
      break;
    case 'warning':
      toast.warning(title, { description: message });
      break;
    case 'info':
    default:
      toast.info(title, { description: message });
      break;
  }
}

/**
 * Show risk alert notification
 */
export async function showRiskAlert(studentName: string, riskLevel: string): Promise<void> {
  if (isElectron()) {
    const api = getElectronAPI();
    if (api) {
      try {
        await api.showRiskAlert(studentName, riskLevel);
        return;
      } catch (error) {
        console.error('Failed to show Electron risk alert:', error);
      }
    }
  }

  toast.warning('Risk Öğrenci Uyarısı', {
    description: `${studentName} - ${riskLevel} risk seviyesi tespit edildi`,
  });
}

/**
 * Show missing data alert
 */
export async function showMissingDataAlert(message: string): Promise<void> {
  if (isElectron()) {
    const api = getElectronAPI();
    if (api) {
      try {
        await api.showMissingDataAlert(message);
        return;
      } catch (error) {
        console.error('Failed to show Electron missing data alert:', error);
      }
    }
  }

  toast.warning('Eksik Veri Uyarısı', {
    description: message,
  });
}

/**
 * Show daily report notification
 */
export async function showDailyReportNotification(message: string): Promise<void> {
  if (isElectron()) {
    const api = getElectronAPI();
    if (api) {
      try {
        await api.showDailyReportNotification(message);
        return;
      } catch (error) {
        console.error('Failed to show Electron daily report:', error);
      }
    }
  }

  toast.info('Günlük Rapor Hazır', {
    description: message,
  });
}
