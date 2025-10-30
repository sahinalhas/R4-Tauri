/// <reference types="vite/client" />

import { IpcRendererEvent } from 'electron';

interface NotificationOptions {
  title: string;
  body: string;
  type?: 'info' | 'warning' | 'error' | 'success';
  urgency?: 'low' | 'normal' | 'critical';
  silent?: boolean;
}

interface FileSelectOptions {
  title?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
  properties?: Array<'openFile' | 'openDirectory' | 'multiSelections'>;
}

interface FileSaveOptions {
  title?: string;
  defaultPath?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
}

interface WindowBounds {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

interface UpdateInfo {
  version: string;
  releaseDate?: string;
}

interface UpdateProgress {
  percent: number;
  transferred: number;
  total: number;
}

interface BackupInfo {
  name: string;
  path: string;
  size: number;
  date: Date;
}

interface NotificationSettings {
  riskStudents: boolean;
  missingData: boolean;
  dailyReports: boolean;
  systemUpdates: boolean;
}

interface ElectronAPI {
  getAppVersion: () => Promise<string>;
  getPlatform: () => Promise<string>;
  getAppPath: (name: string) => Promise<string>;

  minimize: () => Promise<void>;
  maximize: () => Promise<void>;
  close: () => Promise<void>;
  isMaximized: () => Promise<boolean>;
  isMinimized: () => Promise<boolean>;
  isFullScreen: () => Promise<boolean>;
  toggleFullScreen: () => Promise<boolean>;
  show: () => Promise<void>;
  getBounds: () => Promise<WindowBounds>;
  setBounds: (bounds: WindowBounds) => Promise<void>;
  center: () => Promise<void>;

  selectFile: (options?: FileSelectOptions) => Promise<string | null>;
  selectMultiple: (options?: FileSelectOptions) => Promise<string[]>;
  selectDirectory: (title?: string) => Promise<string | null>;
  saveFile: (data: any, options?: FileSaveOptions) => Promise<string | null>;
  readFile: (filePath: string) => Promise<Buffer>;
  openExternal: (url: string) => Promise<void>;
  openPath: (filePath: string) => Promise<void>;
  showItemInFolder: (filePath: string) => Promise<void>;

  dbGetPath: () => Promise<string>;
  dbBackup: () => Promise<string>;
  dbRestore: (backupPath: string) => Promise<boolean>;
  dbSelectBackupFile: () => Promise<string | null>;
  dbListBackups: () => Promise<BackupInfo[]>;

  showNotification: (options: NotificationOptions) => Promise<boolean>;
  isNotificationSupported: () => Promise<boolean>;
  getNotificationSettings: () => Promise<NotificationSettings>;
  updateNotificationSettings: (settings: Partial<NotificationSettings>) => Promise<NotificationSettings>;
  showRiskAlert: (studentName: string, riskLevel: string) => Promise<boolean>;
  showMissingDataAlert: (message: string) => Promise<boolean>;
  showDailyReportNotification: (message: string) => Promise<boolean>;

  getServerPort: () => Promise<number>;

  onMenuNavigate: (callback: (event: IpcRendererEvent, route: string) => void) => () => void;
  onMenuNewStudent: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuImportData: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuExportData: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuSettings: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuDbBackup: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuDbRestore: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuExportExcel: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuExportPDF: (callback: (event: IpcRendererEvent) => void) => () => void;
  onMenuAbout: (callback: (event: IpcRendererEvent) => void) => () => void;

  onUpdateAvailable: (callback: (event: IpcRendererEvent, info: UpdateInfo) => void) => () => void;
  onUpdateDownloaded: (callback: (event: IpcRendererEvent, info: UpdateInfo) => void) => () => void;
  onUpdateDownloadProgress: (callback: (event: IpcRendererEvent, progress: UpdateProgress) => void) => () => void;
  onUpdateError: (callback: (event: IpcRendererEvent, error: { message: string }) => void) => () => void;

  onWindowMaximized: (callback: (event: IpcRendererEvent, isMaximized: boolean) => void) => () => void;
  onWindowFullscreen: (callback: (event: IpcRendererEvent, isFullscreen: boolean) => void) => () => void;
  onWindowFocused: (callback: (event: IpcRendererEvent, isFocused: boolean) => void) => () => void;

  onNotificationClicked: (callback: (event: IpcRendererEvent, data: { title: string; type?: string }) => void) => () => void;
  
  system: {
    getVersions: () => Promise<{ electron: string; chrome: string; node: string }>;
  };
}

declare global {
  interface Window {
    electronAPI?: ElectronAPI;
  }
}
