/**
 * Native Desktop Notifications Service
 * 
 * Provides a unified interface for sending native OS notifications
 * using Tauri's notification plugin when running in desktop mode,
 * with fallback to web notifications API.
 */

import { isPermissionGranted, requestPermission, sendNotification as tauriSendNotification } from '@tauri-apps/plugin-notification';

export interface NotificationOptions {
  title: string;
  body: string;
  icon?: string;
  sound?: string;
  largeBody?: string;
  actionTypeId?: string;
}

/**
 * Check if the app is running in Tauri desktop mode
 */
const isTauriApp = (): boolean => {
  return typeof window !== 'undefined' && '__TAURI__' in window;
};

/**
 * Request notification permission (desktop or web)
 */
export async function requestNotificationPermission(): Promise<boolean> {
  if (isTauriApp()) {
    // Tauri desktop - use plugin
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    return permissionGranted;
  } else {
    // Web - use Web Notifications API
    if (!('Notification' in window)) {
      console.warn('Browser does not support notifications');
      return false;
    }
    
    if (Notification.permission === 'granted') {
      return true;
    }
    
    if (Notification.permission !== 'denied') {
      const permission = await Notification.requestPermission();
      return permission === 'granted';
    }
    
    return false;
  }
}

/**
 * Send a native notification
 */
export async function sendNativeNotification(options: NotificationOptions): Promise<void> {
  const hasPermission = await requestNotificationPermission();
  
  if (!hasPermission) {
    console.warn('Notification permission not granted');
    return;
  }
  
  if (isTauriApp()) {
    // Tauri desktop - use plugin
    await tauriSendNotification({
      title: options.title,
      body: options.body,
      icon: options.icon,
      sound: options.sound,
      largeBody: options.largeBody,
    });
  } else {
    // Web - use Web Notifications API
    new Notification(options.title, {
      body: options.body,
      icon: options.icon || '/logo.png',
      badge: '/logo.png',
      tag: options.actionTypeId,
    });
  }
}

/**
 * Pre-defined notification templates for common scenarios
 */
export const NotificationTemplates = {
  studentAdded: (studentName: string) => ({
    title: 'Yeni Öğrenci Eklendi',
    body: `${studentName} sisteme başarıyla eklendi.`,
    icon: 'student',
  }),
  
  sessionReminder: (sessionType: string, time: string) => ({
    title: 'Görüşme Hatırlatması',
    body: `${sessionType} görüşmesi ${time} tarihinde planlandı.`,
    icon: 'calendar',
  }),
  
  aiSuggestion: (studentName: string, suggestionType: string) => ({
    title: 'Yeni AI Önerisi',
    body: `${studentName} için ${suggestionType} önerisi oluşturuldu.`,
    icon: 'ai',
  }),
  
  riskAlert: (studentName: string, riskLevel: string) => ({
    title: 'Risk Uyarısı',
    body: `${studentName} için ${riskLevel} seviyesinde risk tespit edildi.`,
    icon: 'warning',
    sound: 'default',
  }),
  
  taskDue: (taskTitle: string) => ({
    title: 'Görev Hatırlatması',
    body: `"${taskTitle}" görevinin süresi dolmak üzere.`,
    icon: 'task',
  }),
  
  updateAvailable: (version: string) => ({
    title: 'Güncelleme Mevcut',
    body: `Rehber360 ${version} sürümü yüklemeye hazır.`,
    icon: 'update',
  }),
};

/**
 * Send a notification using a pre-defined template
 */
export async function sendTemplatedNotification(
  template: keyof typeof NotificationTemplates,
  ...args: any[]
): Promise<void> {
  const templateFn = NotificationTemplates[template] as (...args: any[]) => NotificationOptions;
  const options = templateFn(...args);
  await sendNativeNotification(options);
}
