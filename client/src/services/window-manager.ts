/**
 * Window Management Service
 * 
 * Provides utilities for managing the application window
 * in Tauri desktop mode (minimize, maximize, fullscreen, etc.)
 */

import { getCurrentWindow } from '@tauri-apps/api/window';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize, LogicalPosition } from '@tauri-apps/api/dpi';

/**
 * Check if running in Tauri desktop mode
 */
const isTauriApp = (): boolean => {
  return typeof window !== 'undefined' && '__TAURI__' in window;
};

/**
 * Get the current window instance
 */
const getWindow = () => {
  if (!isTauriApp()) return null;
  
  try {
    return getCurrentWebviewWindow();
  } catch {
    // Fallback to legacy API if new one fails
    return getCurrentWindow();
  }
};

/**
 * Window Manager class with utility methods
 */
export class WindowManager {
  /**
   * Minimize the window to taskbar
   */
  static async minimize(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.minimize();
    }
  }

  /**
   * Maximize the window
   */
  static async maximize(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.maximize();
    }
  }

  /**
   * Unmaximize the window
   */
  static async unmaximize(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.unmaximize();
    }
  }

  /**
   * Toggle maximize state
   */
  static async toggleMaximize(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.toggleMaximize();
    }
  }

  /**
   * Show the window
   */
  static async show(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.show();
    }
  }

  /**
   * Hide the window (minimize to tray)
   */
  static async hide(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.hide();
    }
  }

  /**
   * Close the window (will trigger minimize-to-tray if configured)
   */
  static async close(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.close();
    }
  }

  /**
   * Set window fullscreen mode
   */
  static async setFullscreen(fullscreen: boolean): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.setFullscreen(fullscreen);
    }
  }

  /**
   * Toggle fullscreen mode
   */
  static async toggleFullscreen(): Promise<void> {
    const window = getWindow();
    if (window) {
      const isFullscreen = await window.isFullscreen();
      await window.setFullscreen(!isFullscreen);
    }
  }

  /**
   * Check if window is maximized
   */
  static async isMaximized(): Promise<boolean> {
    const window = getWindow();
    if (window) {
      return await window.isMaximized();
    }
    return false;
  }

  /**
   * Check if window is fullscreen
   */
  static async isFullscreen(): Promise<boolean> {
    const window = getWindow();
    if (window) {
      return await window.isFullscreen();
    }
    return false;
  }

  /**
   * Set window title
   */
  static async setTitle(title: string): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.setTitle(title);
    }
  }

  /**
   * Focus the window
   */
  static async focus(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.setFocus();
    }
  }

  /**
   * Center the window on screen
   */
  static async center(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.center();
    }
  }

  /**
   * Set window size
   */
  static async setSize(width: number, height: number): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.setSize(new LogicalSize(width, height));
    }
  }

  /**
   * Get window size
   */
  static async getSize(): Promise<{ width: number; height: number } | null> {
    const window = getWindow();
    if (window) {
      const size = await window.innerSize();
      return { width: size.width, height: size.height };
    }
    return null;
  }

  /**
   * Set window position
   */
  static async setPosition(x: number, y: number): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.setPosition(new LogicalPosition(x, y));
    }
  }

  /**
   * Request user attention (flash taskbar icon)
   */
  static async requestAttention(): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.requestUserAttention(1); // 1 = Critical
    }
  }

  /**
   * Listen to window events
   */
  static async onCloseRequested(callback: () => void): Promise<void> {
    const window = getWindow();
    if (window) {
      await window.onCloseRequested((event) => {
        callback();
      });
    }
  }

  /**
   * Check if running in desktop mode
   */
  static isDesktopMode(): boolean {
    return isTauriApp();
  }
}

/**
 * React hook for window manager
 */
export function useWindowManager() {
  return WindowManager;
}

/**
 * Keyboard shortcuts for window management
 */
export const setupWindowShortcuts = () => {
  if (!isTauriApp()) return;

  // F11 - Toggle fullscreen
  document.addEventListener('keydown', async (e) => {
    if (e.key === 'F11') {
      e.preventDefault();
      await WindowManager.toggleFullscreen();
    }
  });

  // Ctrl+M / Cmd+M - Minimize
  document.addEventListener('keydown', async (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'm') {
      e.preventDefault();
      await WindowManager.minimize();
    }
  });
};
