/**
 * Desktop Integration Index
 * 
 * Central export point for all Tauri desktop integration services.
 * Import this module to access all desktop features.
 */

export { 
  sendNativeNotification, 
  requestNotificationPermission,
  sendTemplatedNotification,
  NotificationTemplates,
  type NotificationOptions 
} from './native-notifications';

export { 
  WindowManager,
  useWindowManager,
  setupWindowShortcuts 
} from './window-manager';

export {
  checkForUpdates,
  downloadAndInstall,
  checkUpdatesOnStartup,
  setupPeriodicUpdateChecks,
  type UpdateInfo
} from './auto-updater';

export {
  WindowStateManager,
  initializeWindowStatePersistence
} from './window-state-manager';

/**
 * Check if running in Tauri desktop mode
 */
function isTauriApp(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

/**
 * Initialize all desktop features
 * Call this once on app startup in desktop mode
 */
export async function initializeDesktopIntegration(): Promise<void> {
  if (!isTauriApp()) {
    console.log('Not running in Tauri desktop mode, skipping desktop integration');
    return;
  }

  console.log('Initializing Tauri desktop integration...');

  // Import and setup window keyboard shortcuts
  const { setupWindowShortcuts: setupShortcuts } = await import('./window-manager');
  setupShortcuts();

  // Import and setup window state persistence
  const { initializeWindowStatePersistence: initWindowState } = await import('./window-state-manager');
  await initWindowState();

  // Import and setup periodic update checks
  const { setupPeriodicUpdateChecks: setupUpdates } = await import('./auto-updater');
  setupUpdates();

  console.log('Desktop integration initialized successfully');
}
