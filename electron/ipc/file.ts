import { ipcMain, dialog, shell } from 'electron';
import fs from 'fs';
import path from 'path';
import log from '../logger';

export interface FileSelectOptions {
  title?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
  properties?: Array<'openFile' | 'openDirectory' | 'multiSelections'>;
}

export interface FileSaveOptions {
  title?: string;
  defaultPath?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
}

export function setupFileHandlers(): void {
  ipcMain.handle('file:select', async (event, options: FileSelectOptions) => {
    try {
      const result = await dialog.showOpenDialog({
        title: options.title || 'Dosya Seç',
        properties: options.properties || ['openFile'],
        filters: options.filters || [
          { name: 'Excel Files', extensions: ['xlsx', 'xls'] },
          { name: 'All Files', extensions: ['*'] },
        ],
      });

      if (!result.canceled && result.filePaths.length > 0) {
        return result.filePaths[0];
      }
      return null;
    } catch (error) {
      log.error('File selection failed:', error);
      throw error;
    }
  });

  ipcMain.handle('file:selectMultiple', async (event, options: FileSelectOptions) => {
    try {
      const result = await dialog.showOpenDialog({
        title: options.title || 'Dosyalar Seç',
        properties: ['openFile', 'multiSelections'],
        filters: options.filters || [{ name: 'All Files', extensions: ['*'] }],
      });

      if (!result.canceled) {
        return result.filePaths;
      }
      return [];
    } catch (error) {
      log.error('Multiple file selection failed:', error);
      throw error;
    }
  });

  ipcMain.handle('file:selectDirectory', async (event, title?: string) => {
    try {
      const result = await dialog.showOpenDialog({
        title: title || 'Klasör Seç',
        properties: ['openDirectory'],
      });

      if (!result.canceled && result.filePaths.length > 0) {
        return result.filePaths[0];
      }
      return null;
    } catch (error) {
      log.error('Directory selection failed:', error);
      throw error;
    }
  });

  ipcMain.handle('file:save', async (event, data: any, options: FileSaveOptions) => {
    try {
      const result = await dialog.showSaveDialog({
        title: options.title || 'Dosyayı Kaydet',
        defaultPath: options.defaultPath,
        filters: options.filters || [
          { name: 'PDF Files', extensions: ['pdf'] },
          { name: 'Excel Files', extensions: ['xlsx'] },
          { name: 'CSV Files', extensions: ['csv'] },
          { name: 'All Files', extensions: ['*'] },
        ],
      });

      if (!result.canceled && result.filePath) {
        let dataToWrite: Buffer;

        if (Buffer.isBuffer(data)) {
          dataToWrite = data;
        } else if (typeof data === 'string') {
          dataToWrite = Buffer.from(data);
        } else if (data instanceof Uint8Array) {
          dataToWrite = Buffer.from(data);
        } else {
          dataToWrite = Buffer.from(JSON.stringify(data));
        }

        fs.writeFileSync(result.filePath, dataToWrite);
        log.info(`File saved: ${result.filePath}`);
        return result.filePath;
      }
      return null;
    } catch (error) {
      log.error('File save failed:', error);
      throw error;
    }
  });

  ipcMain.handle('file:read', async (event, filePath: string) => {
    try {
      if (!fs.existsSync(filePath)) {
        throw new Error('File not found');
      }

      const data = fs.readFileSync(filePath);
      return data;
    } catch (error) {
      log.error('File read failed:', error);
      throw error;
    }
  });

  ipcMain.handle('shell:openExternal', async (event, url: string) => {
    try {
      await shell.openExternal(url);
      log.info(`Opened external URL: ${url}`);
    } catch (error) {
      log.error('Open external URL failed:', error);
      throw error;
    }
  });

  ipcMain.handle('shell:openPath', async (event, filePath: string) => {
    try {
      const result = await shell.openPath(filePath);
      if (result) {
        log.error(`Failed to open path: ${result}`);
        throw new Error(result);
      }
      log.info(`Opened path: ${filePath}`);
    } catch (error) {
      log.error('Open path failed:', error);
      throw error;
    }
  });

  ipcMain.handle('shell:showItemInFolder', async (event, filePath: string) => {
    try {
      shell.showItemInFolder(filePath);
      log.info(`Showed item in folder: ${filePath}`);
    } catch (error) {
      log.error('Show item in folder failed:', error);
      throw error;
    }
  });
}
