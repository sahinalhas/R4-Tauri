import { ipcMain, app, dialog } from 'electron';
import path from 'path';
import fs from 'fs';
import log from '../logger';

const getDbPath = (): string => {
  return path.join(app.getPath('userData'), 'database.db');
};

const getBackupPath = (): string => {
  return path.join(app.getPath('userData'), 'backups');
};

export function setupDatabaseHandlers(): void {
  ipcMain.handle('db:getPath', () => {
    return getDbPath();
  });

  ipcMain.handle('db:backup', async () => {
    try {
      const dbPath = getDbPath();
      const backupDir = getBackupPath();

      if (!fs.existsSync(backupDir)) {
        fs.mkdirSync(backupDir, { recursive: true });
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const backupFileName = `database-backup-${timestamp}.db`;
      const backupFilePath = path.join(backupDir, backupFileName);

      fs.copyFileSync(dbPath, backupFilePath);

      log.info(`Database backup created: ${backupFilePath}`);
      return backupFilePath;
    } catch (error) {
      log.error('Database backup failed:', error);
      throw error;
    }
  });

  ipcMain.handle('db:restore', async (event, backupPath: string) => {
    try {
      if (!fs.existsSync(backupPath)) {
        throw new Error('Backup file not found');
      }

      const dbPath = getDbPath();
      const tempBackup = `${dbPath}.temp`;

      fs.copyFileSync(dbPath, tempBackup);

      try {
        fs.copyFileSync(backupPath, dbPath);
        log.info(`Database restored from: ${backupPath}`);
        
        fs.unlinkSync(tempBackup);
        return true;
      } catch (error) {
        fs.copyFileSync(tempBackup, dbPath);
        fs.unlinkSync(tempBackup);
        throw error;
      }
    } catch (error) {
      log.error('Database restore failed:', error);
      throw error;
    }
  });

  ipcMain.handle('db:selectBackupFile', async () => {
    const result = await dialog.showOpenDialog({
      title: 'Veritabanı Yedeğini Seç',
      properties: ['openFile'],
      filters: [
        { name: 'Database Files', extensions: ['db'] },
        { name: 'All Files', extensions: ['*'] },
      ],
    });

    if (!result.canceled && result.filePaths.length > 0) {
      return result.filePaths[0];
    }
    return null;
  });

  ipcMain.handle('db:listBackups', async () => {
    try {
      const backupDir = getBackupPath();

      if (!fs.existsSync(backupDir)) {
        return [];
      }

      const files = fs.readdirSync(backupDir);
      const backups = files
        .filter((file) => file.endsWith('.db'))
        .map((file) => {
          const filePath = path.join(backupDir, file);
          const stats = fs.statSync(filePath);
          return {
            name: file,
            path: filePath,
            size: stats.size,
            date: stats.mtime,
          };
        })
        .sort((a, b) => b.date.getTime() - a.date.getTime());

      return backups;
    } catch (error) {
      log.error('Failed to list backups:', error);
      return [];
    }
  });
}
