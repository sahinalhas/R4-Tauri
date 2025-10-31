/**
 * Unified Notification Utility
 * 
 * Provides toast notifications using Sonner
 */

import { toast } from 'sonner';

export interface NotificationOptions {
  title: string;
  message: string;
  type?: 'info' | 'success' | 'warning' | 'error';
  duration?: number;
}

/**
 * Show notification using toast
 */
export async function showNotification(options: NotificationOptions): Promise<void> {
  const { title, message, type = 'info' } = options;

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
  toast.warning('Risk Öğrenci Uyarısı', {
    description: `${studentName} - ${riskLevel} risk seviyesi tespit edildi`,
  });
}

/**
 * Show missing data alert
 */
export async function showMissingDataAlert(message: string): Promise<void> {
  toast.warning('Eksik Veri Uyarısı', {
    description: message,
  });
}

/**
 * Show daily report notification
 */
export async function showDailyReportNotification(message: string): Promise<void> {
  toast.info('Günlük Rapor Hazır', {
    description: message,
  });
}
