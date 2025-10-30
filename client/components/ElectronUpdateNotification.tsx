/**
 * Electron Update Notification Component
 * 
 * Displays update notifications for Electron app
 * Shows when updates are available or downloaded
 */

import { useState } from 'react';
import { toast } from 'sonner';
import { useElectronUpdateEvents } from '../hooks/useElectron';
import { isElectron } from '../lib/utils/electron';

export function ElectronUpdateNotification() {
  const [updateInfo, setUpdateInfo] = useState<{ version: string; releaseDate?: string } | null>(null);
  const [downloadProgress, setDownloadProgress] = useState<number>(0);
  const [updateReady, setUpdateReady] = useState(false);

  useElectronUpdateEvents({
    onUpdateAvailable: (info) => {
      setUpdateInfo(info);
      toast.info('Güncelleme Mevcut', {
        description: `Yeni sürüm ${info.version} indiriliyor...`,
      });
    },
    
    onUpdateDownloaded: (info) => {
      setUpdateReady(true);
      setUpdateInfo(info);
      toast.success('Güncelleme Hazır', {
        description: `Sürüm ${info.version} indirildi. Yeniden başlatmak için tıklayın.`,
        action: {
          label: 'Yeniden Başlat',
          onClick: () => {
            if (window.electronAPI) {
              window.electronAPI.close();
            }
          },
        },
        duration: 10000,
      });
    },
    
    onUpdateProgress: (progress) => {
      setDownloadProgress(progress.percent);
    },
    
    onUpdateError: (error) => {
      toast.error('Güncelleme Hatası', {
        description: error.message,
      });
      setUpdateInfo(null);
      setDownloadProgress(0);
    },
  });

  if (!isElectron()) {
    return null;
  }

  return null;
}
