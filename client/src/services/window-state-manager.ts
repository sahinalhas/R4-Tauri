/**
 * Window State Persistence Service
 * 
 * Automatically saves and restores window position, size, and state
 * using Tauri's store plugin for persistent storage.
 */

import { Store } from '@tauri-apps/plugin-store';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize, LogicalPosition } from '@tauri-apps/api/dpi';

interface WindowState {
  x: number;
  y: number;
  width: number;
  height: number;
  maximized: boolean;
  fullscreen: boolean;
}

const STORE_PATH = 'window-state.json';
const WINDOW_STATE_KEY = 'window-state';

/**
 * Check if running in Tauri desktop mode
 */
const isTauriApp = (): boolean => {
  return typeof window !== 'undefined' && '__TAURI__' in window;
};

/**
 * Window State Manager class
 */
export class WindowStateManager {
  private static store: Store | null = null;
  private static saveTimeout: NodeJS.Timeout | null = null;

  /**
   * Initialize the store
   */
  private static async getStore(): Promise<Store | null> {
    if (!isTauriApp()) return null;
    
    if (!this.store) {
      this.store = await Store.load(STORE_PATH);
    }
    return this.store;
  }

  /**
   * Get current window state
   */
  private static async getCurrentState(): Promise<WindowState | null> {
    if (!isTauriApp()) return null;

    const window = getCurrentWebviewWindow();
    if (!window) return null;

    try {
      const position = await window.outerPosition();
      const size = await window.outerSize();
      const maximized = await window.isMaximized();
      const fullscreen = await window.isFullscreen();

      return {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
        maximized,
        fullscreen,
      };
    } catch (error) {
      console.error('Failed to get window state:', error);
      return null;
    }
  }

  /**
   * Save current window state to store
   */
  static async saveState(): Promise<void> {
    const store = await this.getStore();
    if (!store) return;

    const state = await this.getCurrentState();
    if (!state) return;

    try {
      await store.set(WINDOW_STATE_KEY, state);
      await store.save();
      console.log('Window state saved:', state);
    } catch (error) {
      console.error('Failed to save window state:', error);
    }
  }

  /**
   * Save window state with debouncing (prevents excessive saves during resize/move)
   */
  static debouncedSaveState(): void {
    if (this.saveTimeout) {
      clearTimeout(this.saveTimeout);
    }

    this.saveTimeout = setTimeout(() => {
      this.saveState();
    }, 500); // Save 500ms after last change
  }

  /**
   * Restore saved window state
   */
  static async restoreState(): Promise<void> {
    if (!isTauriApp()) return;

    const store = await this.getStore();
    if (!store) return;

    const window = getCurrentWebviewWindow();
    if (!window) return;

    try {
      const state = await store.get<WindowState>(WINDOW_STATE_KEY);
      if (!state) {
        console.log('No saved window state found');
        return;
      }

      console.log('Restoring window state:', state);

      // Restore position and size
      await window.setPosition(new LogicalPosition(state.x, state.y));
      await window.setSize(new LogicalSize(state.width, state.height));

      // Restore maximized state
      if (state.maximized) {
        await window.maximize();
      }

      // Restore fullscreen state
      if (state.fullscreen) {
        await window.setFullscreen(true);
      }

      console.log('Window state restored successfully');
    } catch (error) {
      console.error('Failed to restore window state:', error);
    }
  }

  /**
   * Setup automatic window state persistence
   * Call this once on app startup
   */
  static async setupAutoPersistence(): Promise<void> {
    if (!isTauriApp()) return;

    // Restore state on startup
    await this.restoreState();

    const window = getCurrentWebviewWindow();
    if (!window) return;

    // Listen to window events and save state
    await window.listen('tauri://resize', () => {
      this.debouncedSaveState();
    });

    await window.listen('tauri://move', () => {
      this.debouncedSaveState();
    });

    // Save state before window closes
    await window.onCloseRequested(() => {
      this.saveState();
    });

    console.log('Window state auto-persistence setup complete');
  }

  /**
   * Clear saved window state
   */
  static async clearState(): Promise<void> {
    const store = await this.getStore();
    if (!store) return;

    try {
      await store.delete(WINDOW_STATE_KEY);
      await store.save();
      console.log('Window state cleared');
    } catch (error) {
      console.error('Failed to clear window state:', error);
    }
  }

  /**
   * Reset window to default state
   */
  static async resetToDefault(): Promise<void> {
    if (!isTauriApp()) return;

    const window = getCurrentWebviewWindow();
    if (!window) return;

    try {
      // Set default size and position
      await window.setSize(new LogicalSize(1400, 900));
      await window.center();

      // Clear saved state
      await this.clearState();

      console.log('Window reset to default state');
    } catch (error) {
      console.error('Failed to reset window state:', error);
    }
  }
}

/**
 * Initialize window state persistence on app startup
 * Call this in your app's main initialization
 */
export async function initializeWindowStatePersistence(): Promise<void> {
  await WindowStateManager.setupAutoPersistence();
}
