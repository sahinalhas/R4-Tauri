/**
 * Electron API Hook
 * 
 * React hook for Electron IPC communication
 * Provides type-safe access to Electron features
 */

import { useEffect, useCallback, useMemo } from 'react';
import { isElectron, getElectronAPI } from '../lib/utils/electron';

/**
 * Main Electron hook
 * Provides access to all Electron features
 */
export function useElectron() {
  const electronAvailable = useMemo(() => isElectron(), []);
  const api = useMemo(() => getElectronAPI(), []);

  return {
    isElectron: electronAvailable,
    api,
  };
}

/**
 * Hook for file operations
 */
export function useElectronFile() {
  const { isElectron: available, api } = useElectron();

  const selectFile = useCallback(
    async (options?: { title?: string; filters?: Array<{ name: string; extensions: string[] }> }) => {
      if (!available || !api) return null;
      return api.selectFile(options);
    },
    [available, api]
  );

  const selectMultipleFiles = useCallback(
    async (options?: { title?: string; filters?: Array<{ name: string; extensions: string[] }> }) => {
      if (!available || !api) return [];
      return api.selectMultiple(options);
    },
    [available, api]
  );

  const selectDirectory = useCallback(
    async (title?: string) => {
      if (!available || !api) return null;
      return api.selectDirectory(title);
    },
    [available, api]
  );

  const saveFile = useCallback(
    async (data: any, options?: { title?: string; defaultPath?: string; filters?: Array<{ name: string; extensions: string[] }> }) => {
      if (!available || !api) return null;
      return api.saveFile(data, options);
    },
    [available, api]
  );

  const openPath = useCallback(
    async (filePath: string) => {
      if (!available || !api) return;
      return api.openPath(filePath);
    },
    [available, api]
  );

  const showItemInFolder = useCallback(
    async (filePath: string) => {
      if (!available || !api) return;
      return api.showItemInFolder(filePath);
    },
    [available, api]
  );

  return {
    isAvailable: available,
    selectFile,
    selectMultipleFiles,
    selectDirectory,
    saveFile,
    openPath,
    showItemInFolder,
  };
}

/**
 * Hook for database operations
 */
export function useElectronDatabase() {
  const { isElectron: available, api } = useElectron();

  const backup = useCallback(async () => {
    if (!available || !api) return null;
    return api.dbBackup();
  }, [available, api]);

  const restore = useCallback(
    async (backupPath: string) => {
      if (!available || !api) return false;
      return api.dbRestore(backupPath);
    },
    [available, api]
  );

  const selectBackupFile = useCallback(async () => {
    if (!available || !api) return null;
    return api.dbSelectBackupFile();
  }, [available, api]);

  const listBackups = useCallback(async () => {
    if (!available || !api) return [];
    return api.dbListBackups();
  }, [available, api]);

  const getPath = useCallback(async () => {
    if (!available || !api) return null;
    return api.dbGetPath();
  }, [available, api]);

  return {
    isAvailable: available,
    backup,
    restore,
    selectBackupFile,
    listBackups,
    getPath,
  };
}

/**
 * Hook for notifications
 */
export function useElectronNotification() {
  const { isElectron: available, api } = useElectron();

  const showNotification = useCallback(
    async (options: { title: string; body: string; type?: 'info' | 'warning' | 'error' | 'success' }) => {
      if (!available || !api) return false;
      return api.showNotification(options);
    },
    [available, api]
  );

  const showRiskAlert = useCallback(
    async (studentName: string, riskLevel: string) => {
      if (!available || !api) return false;
      return api.showRiskAlert(studentName, riskLevel);
    },
    [available, api]
  );

  const showMissingDataAlert = useCallback(
    async (message: string) => {
      if (!available || !api) return false;
      return api.showMissingDataAlert(message);
    },
    [available, api]
  );

  const showDailyReportNotification = useCallback(
    async (message: string) => {
      if (!available || !api) return false;
      return api.showDailyReportNotification(message);
    },
    [available, api]
  );

  return {
    isAvailable: available,
    showNotification,
    showRiskAlert,
    showMissingDataAlert,
    showDailyReportNotification,
  };
}

/**
 * Hook for window controls
 */
export function useElectronWindow() {
  const { isElectron: available, api } = useElectron();

  const minimize = useCallback(async () => {
    if (!available || !api) return;
    return api.minimize();
  }, [available, api]);

  const maximize = useCallback(async () => {
    if (!available || !api) return;
    return api.maximize();
  }, [available, api]);

  const close = useCallback(async () => {
    if (!available || !api) return;
    return api.close();
  }, [available, api]);

  const isMaximized = useCallback(async () => {
    if (!available || !api) return false;
    return api.isMaximized();
  }, [available, api]);

  const toggleFullScreen = useCallback(async () => {
    if (!available || !api) return false;
    return api.toggleFullScreen();
  }, [available, api]);

  return {
    isAvailable: available,
    minimize,
    maximize,
    close,
    isMaximized,
    toggleFullScreen,
  };
}

/**
 * Hook for menu events
 */
export function useElectronMenuEvents(callbacks: {
  onNavigate?: (route: string) => void;
  onNewStudent?: () => void;
  onImportData?: () => void;
  onExportData?: () => void;
  onSettings?: () => void;
  onDbBackup?: () => void;
  onDbRestore?: () => void;
  onExportExcel?: () => void;
  onExportPDF?: () => void;
  onAbout?: () => void;
}) {
  const { isElectron: available, api } = useElectron();

  useEffect(() => {
    if (!available || !api) return;

    const cleanups: Array<() => void> = [];

    if (callbacks.onNavigate) {
      const cleanup = api.onMenuNavigate((_, route) => callbacks.onNavigate!(route));
      cleanups.push(cleanup);
    }

    if (callbacks.onNewStudent) {
      const cleanup = api.onMenuNewStudent(() => callbacks.onNewStudent!());
      cleanups.push(cleanup);
    }

    if (callbacks.onImportData) {
      const cleanup = api.onMenuImportData(() => callbacks.onImportData!());
      cleanups.push(cleanup);
    }

    if (callbacks.onExportData) {
      const cleanup = api.onMenuExportData(() => callbacks.onExportData!());
      cleanups.push(cleanup);
    }

    if (callbacks.onSettings) {
      const cleanup = api.onMenuSettings(() => callbacks.onSettings!());
      cleanups.push(cleanup);
    }

    if (callbacks.onDbBackup) {
      const cleanup = api.onMenuDbBackup(() => callbacks.onDbBackup!());
      cleanups.push(cleanup);
    }

    if (callbacks.onDbRestore) {
      const cleanup = api.onMenuDbRestore(() => callbacks.onDbRestore!());
      cleanups.push(cleanup);
    }

    if (callbacks.onExportExcel) {
      const cleanup = api.onMenuExportExcel(() => callbacks.onExportExcel!());
      cleanups.push(cleanup);
    }

    if (callbacks.onExportPDF) {
      const cleanup = api.onMenuExportPDF(() => callbacks.onExportPDF!());
      cleanups.push(cleanup);
    }

    if (callbacks.onAbout) {
      const cleanup = api.onMenuAbout(() => callbacks.onAbout!());
      cleanups.push(cleanup);
    }

    return () => {
      cleanups.forEach(cleanup => cleanup());
    };
  }, [available, api, callbacks]);
}

/**
 * Hook for update events
 */
export function useElectronUpdateEvents(callbacks: {
  onUpdateAvailable?: (info: { version: string; releaseDate?: string }) => void;
  onUpdateDownloaded?: (info: { version: string; releaseDate?: string }) => void;
  onUpdateProgress?: (progress: { percent: number; transferred: number; total: number }) => void;
  onUpdateError?: (error: { message: string }) => void;
}) {
  const { isElectron: available, api } = useElectron();

  useEffect(() => {
    if (!available || !api) return;

    const cleanups: Array<() => void> = [];

    if (callbacks.onUpdateAvailable) {
      const cleanup = api.onUpdateAvailable((_, info) => callbacks.onUpdateAvailable!(info));
      cleanups.push(cleanup);
    }

    if (callbacks.onUpdateDownloaded) {
      const cleanup = api.onUpdateDownloaded((_, info) => callbacks.onUpdateDownloaded!(info));
      cleanups.push(cleanup);
    }

    if (callbacks.onUpdateProgress) {
      const cleanup = api.onUpdateDownloadProgress((_, progress) => callbacks.onUpdateProgress!(progress));
      cleanups.push(cleanup);
    }

    if (callbacks.onUpdateError) {
      const cleanup = api.onUpdateError((_, error) => callbacks.onUpdateError!(error));
      cleanups.push(cleanup);
    }

    return () => {
      cleanups.forEach(cleanup => cleanup());
    };
  }, [available, api, callbacks]);
}
