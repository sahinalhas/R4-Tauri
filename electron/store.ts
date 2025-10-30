import Store from 'electron-store';

export interface WindowBounds {
  width: number;
  height: number;
  x: number;
  y: number;
  isMaximized?: boolean;
}

export interface NotificationSettings {
  riskStudents: boolean;
  missingData: boolean;
  dailyReports: boolean;
  systemUpdates: boolean;
}

export interface BackupSettings {
  autoBackup: boolean;
  backupFrequency: 'daily' | 'weekly' | 'monthly';
  backupTime: string;
  maxBackups: number;
}

export interface AIProviderSettings {
  provider: 'openai' | 'ollama' | 'gemini';
  model: string;
  apiKey?: string;
}

export interface StoreSchema {
  windowBounds: WindowBounds;
  theme: 'light' | 'dark' | 'system';
  language: 'tr';
  notifications: NotificationSettings;
  lastOpenedPages: string[];
  backup: BackupSettings;
  aiProvider: AIProviderSettings;
  minimizeToTray: boolean;
  startMinimized: boolean;
  autoStart: boolean;
}

const storeInstance = new Store<StoreSchema>({
  defaults: {
    windowBounds: {
      width: 1280,
      height: 800,
      x: 0,
      y: 0,
      isMaximized: false,
    },
    theme: 'light',
    language: 'tr',
    notifications: {
      riskStudents: true,
      missingData: true,
      dailyReports: false,
      systemUpdates: true,
    },
    lastOpenedPages: [],
    backup: {
      autoBackup: true,
      backupFrequency: 'daily',
      backupTime: '02:00',
      maxBackups: 7,
    },
    aiProvider: {
      provider: 'ollama',
      model: 'llama3',
    },
    minimizeToTray: true,
    startMinimized: false,
    autoStart: false,
  },
});

export const getStoreValue = <K extends keyof StoreSchema>(key: K): StoreSchema[K] => {
  return (storeInstance as any).get(key) as StoreSchema[K];
};

export const setStoreValue = <K extends keyof StoreSchema>(key: K, value: StoreSchema[K]): void => {
  (storeInstance as any).set(key, value);
};

export default storeInstance;
