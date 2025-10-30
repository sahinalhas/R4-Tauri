/**
 * Electron Environment Detection and Utilities
 * 
 * Bu modül Electron ortamını tespit eder ve 
 * Electron-spesifik özelliklere erişim sağlar
 */

/**
 * Uygulamanın Electron içinde çalışıp çalışmadığını kontrol eder
 */
export function isElectron(): boolean {
  return typeof window !== 'undefined' && window.electronAPI !== undefined;
}

/**
 * Electron production modunda mı kontrol eder
 */
export function isElectronProduction(): boolean {
  if (!isElectron()) return false;
  return import.meta.env.PROD;
}

/**
 * Electron development modunda mı kontrol eder
 */
export function isElectronDev(): boolean {
  if (!isElectron()) return false;
  return import.meta.env.DEV;
}

/**
 * Backend API base URL'ini döndürür
 * - Electron: Child process'ten dinamik olarak alır
 * - Web: Environment variable kullanır
 */
export async function getBackendUrl(): Promise<string> {
  if (isElectron() && window.electronAPI) {
    try {
      const port = await window.electronAPI.getServerPort();
      return `http://localhost:${port}`;
    } catch (error) {
      console.error('Failed to get Electron backend port:', error);
      return 'http://localhost:3000';
    }
  }
  
  return import.meta.env.VITE_API_URL || 'http://localhost:3000';
}

/**
 * Backend API base URL'ini senkron döndürür (cache kullanır)
 */
let cachedBackendUrl: string | null = null;

export function getBackendUrlSync(): string {
  if (cachedBackendUrl) return cachedBackendUrl;
  
  if (isElectron()) {
    return 'http://localhost:3000';
  }
  
  return import.meta.env.VITE_API_URL || 'http://localhost:3000';
}

/**
 * Backend URL cache'ini günceller
 */
export async function updateBackendUrlCache(): Promise<void> {
  if (isElectron() && window.electronAPI) {
    try {
      const port = await window.electronAPI.getServerPort();
      cachedBackendUrl = `http://localhost:${port}`;
    } catch (error) {
      console.error('Failed to update backend URL cache:', error);
      cachedBackendUrl = 'http://localhost:3000';
    }
  } else {
    cachedBackendUrl = import.meta.env.VITE_API_URL || 'http://localhost:3000';
  }
}

/**
 * Uygulama platformunu döndürür
 */
export function getPlatform(): 'electron' | 'web' {
  return isElectron() ? 'electron' : 'web';
}

/**
 * Electron API'ye erişim kontrolü yapar
 */
export function getElectronAPI() {
  if (!isElectron()) {
    console.warn('Electron API is not available in web mode');
    return null;
  }
  return window.electronAPI;
}

/**
 * Electron version bilgisini döndürür
 */
export async function getElectronVersion(): Promise<{
  electron: string;
  chrome: string;
  node: string;
} | null> {
  const api = getElectronAPI();
  if (!api || !api.system) return null;
  
  try {
    return await api.system.getVersions();
  } catch (error) {
    console.error('Failed to get Electron versions:', error);
    return null;
  }
}

/**
 * Electron özelliklerinin kullanılabilirliğini kontrol eder
 */
export function hasElectronFeature(feature: string): boolean {
  const api = getElectronAPI();
  if (!api) return false;
  return feature in api;
}
