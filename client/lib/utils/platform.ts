/**
 * Platform Detection Utility
 * 
 * Verifies that the app is running in Tauri desktop environment.
 * This is a desktop-only application - web mode is not supported.
 */

/**
 * Check if Tauri desktop environment is available
 * @returns true if Tauri API is detected
 */
export function isDesktopMode(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }
  
  // Check if Tauri API is available
  return '__TAURI__' in window || '__TAURI_INTERNALS__' in window;
}

/**
 * Get platform debug info
 */
export function getPlatformInfo() {
  const hasTauri = isDesktopMode();
  const info: Record<string, unknown> = {
    tauri: hasTauri,
    userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
  };
  
  if (hasTauri && typeof window !== 'undefined' && '__TAURI__' in window) {
    info.tauriVersion = (window as any).__TAURI__?.version || 'unknown';
  }
  
  return info;
}

/**
 * Log platform detection result (for debugging)
 */
export function logPlatformInfo() {
  const info = getPlatformInfo();
  console.log('[Platform Detection]', info);
}
