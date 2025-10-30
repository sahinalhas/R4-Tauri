import { contextBridge, ipcRenderer, IpcRendererEvent } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  getAppVersion: () => ipcRenderer.invoke('app:getVersion'),
  getPlatform: () => ipcRenderer.invoke('app:getPlatform'),
  getAppPath: (name: string) => ipcRenderer.invoke('app:getPath', name),

  minimize: () => ipcRenderer.invoke('window:minimize'),
  maximize: () => ipcRenderer.invoke('window:maximize'),
  close: () => ipcRenderer.invoke('window:close'),
  isMaximized: () => ipcRenderer.invoke('window:isMaximized'),
  isMinimized: () => ipcRenderer.invoke('window:isMinimized'),
  isFullScreen: () => ipcRenderer.invoke('window:isFullScreen'),
  toggleFullScreen: () => ipcRenderer.invoke('window:toggleFullScreen'),
  show: () => ipcRenderer.invoke('window:show'),
  getBounds: () => ipcRenderer.invoke('window:getBounds'),
  setBounds: (bounds: { x?: number; y?: number; width?: number; height?: number }) =>
    ipcRenderer.invoke('window:setBounds', bounds),
  center: () => ipcRenderer.invoke('window:center'),

  selectFile: (options?: any) => ipcRenderer.invoke('file:select', options),
  selectMultiple: (options?: any) => ipcRenderer.invoke('file:selectMultiple', options),
  selectDirectory: (title?: string) => ipcRenderer.invoke('file:selectDirectory', title),
  saveFile: (data: any, options?: any) => ipcRenderer.invoke('file:save', data, options),
  readFile: (filePath: string) => ipcRenderer.invoke('file:read', filePath),
  openExternal: (url: string) => ipcRenderer.invoke('shell:openExternal', url),
  openPath: (filePath: string) => ipcRenderer.invoke('shell:openPath', filePath),
  showItemInFolder: (filePath: string) => ipcRenderer.invoke('shell:showItemInFolder', filePath),

  dbGetPath: () => ipcRenderer.invoke('db:getPath'),
  dbBackup: () => ipcRenderer.invoke('db:backup'),
  dbRestore: (backupPath: string) => ipcRenderer.invoke('db:restore', backupPath),
  dbSelectBackupFile: () => ipcRenderer.invoke('db:selectBackupFile'),
  dbListBackups: () => ipcRenderer.invoke('db:listBackups'),

  showNotification: (options: {
    title: string;
    body: string;
    type?: 'info' | 'warning' | 'error' | 'success';
    urgency?: 'low' | 'normal' | 'critical';
    silent?: boolean;
  }) => ipcRenderer.invoke('notification:show', options),
  isNotificationSupported: () => ipcRenderer.invoke('notification:isSupported'),
  getNotificationSettings: () => ipcRenderer.invoke('notification:getSettings'),
  updateNotificationSettings: (settings: any) =>
    ipcRenderer.invoke('notification:updateSettings', settings),
  showRiskAlert: (studentName: string, riskLevel: string) =>
    ipcRenderer.invoke('notification:showRiskAlert', studentName, riskLevel),
  showMissingDataAlert: (message: string) =>
    ipcRenderer.invoke('notification:showMissingData', message),
  showDailyReportNotification: (message: string) =>
    ipcRenderer.invoke('notification:showDailyReport', message),

  getServerPort: () => ipcRenderer.invoke('server:getPort'),

  onMenuNavigate: (callback: (event: IpcRendererEvent, route: string) => void) => {
    ipcRenderer.on('menu:navigate', callback);
    return () => ipcRenderer.removeListener('menu:navigate', callback);
  },
  onMenuNewStudent: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:newStudent', callback);
    return () => ipcRenderer.removeListener('menu:newStudent', callback);
  },
  onMenuImportData: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:importData', callback);
    return () => ipcRenderer.removeListener('menu:importData', callback);
  },
  onMenuExportData: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:exportData', callback);
    return () => ipcRenderer.removeListener('menu:exportData', callback);
  },
  onMenuSettings: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:settings', callback);
    return () => ipcRenderer.removeListener('menu:settings', callback);
  },
  onMenuDbBackup: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:dbBackup', callback);
    return () => ipcRenderer.removeListener('menu:dbBackup', callback);
  },
  onMenuDbRestore: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:dbRestore', callback);
    return () => ipcRenderer.removeListener('menu:dbRestore', callback);
  },
  onMenuExportExcel: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:exportExcel', callback);
    return () => ipcRenderer.removeListener('menu:exportExcel', callback);
  },
  onMenuExportPDF: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:exportPDF', callback);
    return () => ipcRenderer.removeListener('menu:exportPDF', callback);
  },
  onMenuAbout: (callback: (event: IpcRendererEvent) => void) => {
    ipcRenderer.on('menu:about', callback);
    return () => ipcRenderer.removeListener('menu:about', callback);
  },

  onUpdateAvailable: (callback: (event: IpcRendererEvent, info: any) => void) => {
    ipcRenderer.on('update:available', callback);
    return () => ipcRenderer.removeListener('update:available', callback);
  },
  onUpdateDownloaded: (callback: (event: IpcRendererEvent, info: any) => void) => {
    ipcRenderer.on('update:downloaded', callback);
    return () => ipcRenderer.removeListener('update:downloaded', callback);
  },
  onUpdateDownloadProgress: (callback: (event: IpcRendererEvent, progress: any) => void) => {
    ipcRenderer.on('update:downloadProgress', callback);
    return () => ipcRenderer.removeListener('update:downloadProgress', callback);
  },
  onUpdateError: (callback: (event: IpcRendererEvent, error: any) => void) => {
    ipcRenderer.on('update:error', callback);
    return () => ipcRenderer.removeListener('update:error', callback);
  },

  onWindowMaximized: (callback: (event: IpcRendererEvent, isMaximized: boolean) => void) => {
    ipcRenderer.on('window:maximized', callback);
    return () => ipcRenderer.removeListener('window:maximized', callback);
  },
  onWindowFullscreen: (callback: (event: IpcRendererEvent, isFullscreen: boolean) => void) => {
    ipcRenderer.on('window:fullscreen', callback);
    return () => ipcRenderer.removeListener('window:fullscreen', callback);
  },
  onWindowFocused: (callback: (event: IpcRendererEvent, isFocused: boolean) => void) => {
    ipcRenderer.on('window:focused', callback);
    return () => ipcRenderer.removeListener('window:focused', callback);
  },

  onNotificationClicked: (callback: (event: IpcRendererEvent, data: any) => void) => {
    ipcRenderer.on('notification:clicked', callback);
    return () => ipcRenderer.removeListener('notification:clicked', callback);
  },
});
