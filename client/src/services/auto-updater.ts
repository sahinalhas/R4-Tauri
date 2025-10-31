/**
 * Auto-Updater Service
 * 
 * Manages application updates using Tauri's updater plugin.
 * Checks for updates, downloads, and prompts user to install.
 */

import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask } from '@tauri-apps/plugin-dialog';

export interface UpdateInfo {
  version: string;
  currentVersion: string;
  date?: string;
  body?: string;
}

/**
 * Check if running in Tauri desktop mode
 */
const isTauriApp = (): boolean => {
  return typeof window !== 'undefined' && '__TAURI__' in window;
};

/**
 * Check if updater is configured and enabled
 * Note: Updater requires signing key setup before activation
 * See docs/AUTO_UPDATER_SETUP.md for configuration steps
 */
const isUpdaterConfigured = (): boolean => {
  // Updater is currently DISABLED until signing keys are set up
  // To enable: follow steps in AUTO_UPDATER_SETUP.md
  return false;
};

/**
 * Check for available updates
 */
export async function checkForUpdates(): Promise<UpdateInfo | null> {
  if (!isTauriApp()) {
    console.log('Auto-updater only available in desktop mode');
    return null;
  }

  if (!isUpdaterConfigured()) {
    console.warn('Auto-updater is not configured. See docs/AUTO_UPDATER_SETUP.md for setup instructions.');
    return null;
  }

  try {
    const update = await check();
    
    if (update?.available) {
      return {
        version: update.version,
        currentVersion: update.currentVersion,
        date: update.date,
        body: update.body,
      };
    }
    
    return null;
  } catch (error) {
    console.error('Failed to check for updates:', error);
    return null;
  }
}

/**
 * Download and install update with user confirmation
 */
export async function downloadAndInstall(onProgress?: (progress: number) => void): Promise<boolean> {
  if (!isTauriApp()) {
    console.log('Auto-updater only available in desktop mode');
    return false;
  }

  if (!isUpdaterConfigured()) {
    console.warn('Auto-updater is not configured. See docs/AUTO_UPDATER_SETUP.md for setup instructions.');
    return false;
  }

  try {
    const update = await check();
    
    if (!update?.available) {
      console.log('No updates available');
      return false;
    }

    // Ask user for confirmation
    const shouldUpdate = await ask(
      `Yeni sürüm ${update.version} mevcut. Güncellemek ister misiniz?\n\n${update.body || ''}`,
      {
        title: 'Güncelleme Mevcut',
        kind: 'info',
        okLabel: 'Güncelle',
        cancelLabel: 'Daha Sonra',
      }
    );

    if (!shouldUpdate) {
      return false;
    }

    // Download and install update
    let downloaded = 0;
    let contentLength = 0;
    
    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          contentLength = event.data.contentLength ?? 0;
          console.log(`Starting download of ${contentLength} bytes`);
          break;
        case 'Progress':
          downloaded += event.data.chunkLength;
          const progress = contentLength > 0 ? (downloaded / contentLength) * 100 : 0;
          console.log(`Downloaded ${downloaded} of ${contentLength} (${progress.toFixed(1)}%)`);
          onProgress?.(progress);
          break;
        case 'Finished':
          console.log('Download finished');
          break;
      }
    });

    // Ask to restart
    const shouldRestart = await ask(
      'Güncelleme başarıyla indirildi. Uygulamayı şimdi yeniden başlatmak ister misiniz?',
      {
        title: 'Yeniden Başlat',
        kind: 'info',
        okLabel: 'Yeniden Başlat',
        cancelLabel: 'Daha Sonra',
      }
    );

    if (shouldRestart) {
      await relaunch();
    }

    return true;
  } catch (error) {
    console.error('Failed to download and install update:', error);
    return false;
  }
}

/**
 * Check for updates on app startup
 */
export async function checkUpdatesOnStartup(): Promise<void> {
  if (!isTauriApp() || !isUpdaterConfigured()) {
    if (isTauriApp()) {
      console.log('Auto-updater not configured. Set up signing keys to enable automatic updates.');
    }
    return;
  }

  try {
    const updateInfo = await checkForUpdates();
    
    if (updateInfo) {
      console.log(`Update available: ${updateInfo.version}`);
      
      // Show non-blocking notification
      const shouldUpdate = await ask(
        `Yeni sürüm ${updateInfo.version} mevcut. Güncellemek ister misiniz?`,
        {
          title: 'Güncelleme Mevcut',
          kind: 'info',
          okLabel: 'Evet',
          cancelLabel: 'Hayır',
        }
      );

      if (shouldUpdate) {
        await downloadAndInstall();
      }
    }
  } catch (error) {
    console.error('Startup update check failed:', error);
  }
}

/**
 * Setup periodic update checks (every 6 hours)
 * Only active if updater is properly configured
 */
export function setupPeriodicUpdateChecks(): void {
  if (!isTauriApp() || !isUpdaterConfigured()) {
    if (isTauriApp()) {
      console.log('Periodic update checks disabled until updater is configured.');
    }
    return;
  }

  // Check on startup
  checkUpdatesOnStartup();

  // Check every 6 hours
  setInterval(() => {
    checkForUpdates().then((updateInfo) => {
      if (updateInfo) {
        console.log('Periodic check found update:', updateInfo.version);
      }
    });
  }, 6 * 60 * 60 * 1000); // 6 hours
}
