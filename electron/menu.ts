import { Menu, BrowserWindow, app, shell, MenuItem, MenuItemConstructorOptions } from 'electron';
import log from './logger';

export function createApplicationMenu(mainWindow: BrowserWindow, isDev: boolean): Menu {
  const template: MenuItemConstructorOptions[] = [
    {
      label: 'Dosya',
      submenu: [
        {
          label: 'Yeni Öğrenci',
          accelerator: 'CmdOrCtrl+N',
          click: () => {
            mainWindow.webContents.send('menu:newStudent');
            log.debug('Menu: New Student clicked');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Veri İçe Aktar',
          accelerator: 'CmdOrCtrl+I',
          click: () => {
            mainWindow.webContents.send('menu:importData');
            log.debug('Menu: Import Data clicked');
          },
        },
        {
          label: 'Veri Dışa Aktar',
          accelerator: 'CmdOrCtrl+E',
          click: () => {
            mainWindow.webContents.send('menu:exportData');
            log.debug('Menu: Export Data clicked');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Ayarlar',
          accelerator: 'CmdOrCtrl+,',
          click: () => {
            mainWindow.webContents.send('menu:settings');
            log.debug('Menu: Settings clicked');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Çıkış',
          accelerator: 'CmdOrCtrl+Q',
          click: () => {
            app.quit();
          },
        },
      ],
    },
    {
      label: 'Düzenle',
      submenu: [
        {
          label: 'Geri Al',
          accelerator: 'CmdOrCtrl+Z',
          role: 'undo',
        },
        {
          label: 'İleri Al',
          accelerator: 'CmdOrCtrl+Shift+Z',
          role: 'redo',
        },
        {
          type: 'separator',
        },
        {
          label: 'Kes',
          accelerator: 'CmdOrCtrl+X',
          role: 'cut',
        },
        {
          label: 'Kopyala',
          accelerator: 'CmdOrCtrl+C',
          role: 'copy',
        },
        {
          label: 'Yapıştır',
          accelerator: 'CmdOrCtrl+V',
          role: 'paste',
        },
        {
          type: 'separator',
        },
        {
          label: 'Tümünü Seç',
          accelerator: 'CmdOrCtrl+A',
          role: 'selectAll',
        },
      ],
    },
    {
      label: 'Görünüm',
      submenu: [
        {
          label: 'Ana Sayfa',
          accelerator: 'CmdOrCtrl+H',
          click: () => {
            mainWindow.webContents.send('menu:navigate', '/');
            log.debug('Menu: Navigate to Home');
          },
        },
        {
          label: 'Öğrenciler',
          accelerator: 'CmdOrCtrl+1',
          click: () => {
            mainWindow.webContents.send('menu:navigate', '/students');
            log.debug('Menu: Navigate to Students');
          },
        },
        {
          label: 'Sınavlar',
          accelerator: 'CmdOrCtrl+2',
          click: () => {
            mainWindow.webContents.send('menu:navigate', '/exams');
            log.debug('Menu: Navigate to Exams');
          },
        },
        {
          label: 'Anketler',
          accelerator: 'CmdOrCtrl+3',
          click: () => {
            mainWindow.webContents.send('menu:navigate', '/surveys');
            log.debug('Menu: Navigate to Surveys');
          },
        },
        {
          label: 'Raporlar',
          accelerator: 'CmdOrCtrl+4',
          click: () => {
            mainWindow.webContents.send('menu:navigate', '/reports');
            log.debug('Menu: Navigate to Reports');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Tam Ekran',
          accelerator: 'F11',
          click: () => {
            const isFullScreen = mainWindow.isFullScreen();
            mainWindow.setFullScreen(!isFullScreen);
            log.debug(`Menu: Full screen toggled - ${!isFullScreen}`);
          },
        },
        {
          label: 'Yenile',
          accelerator: 'CmdOrCtrl+R',
          click: () => {
            mainWindow.webContents.reload();
            log.debug('Menu: Page reloaded');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Yakınlaştır',
          accelerator: 'CmdOrCtrl+Plus',
          click: () => {
            const currentZoom = mainWindow.webContents.getZoomLevel();
            mainWindow.webContents.setZoomLevel(currentZoom + 0.5);
            log.debug(`Menu: Zoom in - level ${currentZoom + 0.5}`);
          },
        },
        {
          label: 'Uzaklaştır',
          accelerator: 'CmdOrCtrl+-',
          click: () => {
            const currentZoom = mainWindow.webContents.getZoomLevel();
            mainWindow.webContents.setZoomLevel(currentZoom - 0.5);
            log.debug(`Menu: Zoom out - level ${currentZoom - 0.5}`);
          },
        },
        {
          label: 'Sıfırla',
          accelerator: 'CmdOrCtrl+0',
          click: () => {
            mainWindow.webContents.setZoomLevel(0);
            log.debug('Menu: Zoom reset');
          },
        },
      ],
    },
    {
      label: 'Veritabanı',
      submenu: [
        {
          label: 'Yedek Al',
          accelerator: 'CmdOrCtrl+B',
          click: () => {
            mainWindow.webContents.send('menu:dbBackup');
            log.debug('Menu: Database Backup clicked');
          },
        },
        {
          label: 'Yedek Geri Yükle',
          click: () => {
            mainWindow.webContents.send('menu:dbRestore');
            log.debug('Menu: Database Restore clicked');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Excel Export',
          click: () => {
            mainWindow.webContents.send('menu:exportExcel');
            log.debug('Menu: Export Excel clicked');
          },
        },
        {
          label: 'PDF Export',
          accelerator: 'CmdOrCtrl+P',
          click: () => {
            mainWindow.webContents.send('menu:exportPDF');
            log.debug('Menu: Export PDF clicked');
          },
        },
      ],
    },
    {
      label: 'Yardım',
      submenu: [
        {
          label: 'Dokümantasyon',
          click: () => {
            shell.openExternal('https://rehber360.com/docs');
            log.debug('Menu: Documentation opened');
          },
        },
        {
          label: 'Sürüm Notları',
          click: () => {
            shell.openExternal('https://rehber360.com/changelog');
            log.debug('Menu: Changelog opened');
          },
        },
        {
          type: 'separator',
        },
        {
          label: 'Hakkında',
          click: () => {
            mainWindow.webContents.send('menu:about');
            log.debug('Menu: About clicked');
          },
        },
      ],
    },
  ];

  if (isDev) {
    const devMenuIndex = template.findIndex((item) => item.label === 'Görünüm');
    if (devMenuIndex !== -1 && Array.isArray(template[devMenuIndex].submenu)) {
      (template[devMenuIndex].submenu as MenuItemConstructorOptions[]).push(
        {
          type: 'separator',
        },
        {
          label: 'Geliştirici Araçları',
          accelerator: 'CmdOrCtrl+Shift+I',
          click: () => {
            mainWindow.webContents.toggleDevTools();
            log.debug('Menu: DevTools toggled');
          },
        }
      );
    }
  }

  const menu = Menu.buildFromTemplate(template);
  return menu;
}

export function setupApplicationMenu(mainWindow: BrowserWindow, isDev: boolean): void {
  const menu = createApplicationMenu(mainWindow, isDev);
  Menu.setApplicationMenu(menu);
  log.info('Application menu set up successfully');
}
