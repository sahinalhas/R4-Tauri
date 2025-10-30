const fs = require('fs');
const path = require('path');

console.log('🔍 Rehber360 - Build Validation\n');

let errorCount = 0;
let warningCount = 0;

// Validation helper
function checkFile(filePath, required = true) {
  const fullPath = path.join(process.cwd(), filePath);
  if (fs.existsSync(fullPath)) {
    const stats = fs.statSync(fullPath);
    const size = (stats.size / 1024).toFixed(2);
    console.log(`✅ ${filePath} (${size} KB)`);
    return true;
  } else {
    if (required) {
      console.log(`❌ ${filePath} - EKSIK!`);
      errorCount++;
    } else {
      console.log(`⚠️  ${filePath} - Bulunamadı (opsiyonel)`);
      warningCount++;
    }
    return false;
  }
}

function checkDirectory(dirPath, required = true) {
  const fullPath = path.join(process.cwd(), dirPath);
  if (fs.existsSync(fullPath) && fs.statSync(fullPath).isDirectory()) {
    const files = fs.readdirSync(fullPath);
    console.log(`✅ ${dirPath}/ (${files.length} dosya)`);
    return true;
  } else {
    if (required) {
      console.log(`❌ ${dirPath}/ - EKSIK!`);
      errorCount++;
    } else {
      console.log(`⚠️  ${dirPath}/ - Bulunamadı (opsiyonel)`);
      warningCount++;
    }
    return false;
  }
}

// 1. Electron Build Dosyaları
console.log('📦 Electron Build Dosyaları:');
checkFile('dist/electron/main.js');
checkFile('dist/electron/preload.js');
checkFile('dist/electron/logger.js');
checkFile('dist/electron/store.js');
checkFile('dist/electron/menu.js');
checkFile('dist/electron/tray.js');
checkFile('dist/electron/updater.js');

console.log('\n📦 Electron IPC Handlers:');
checkFile('dist/electron/ipc/database.js');
checkFile('dist/electron/ipc/file.js');
checkFile('dist/electron/ipc/window.js');
checkFile('dist/electron/ipc/notifications.js');

// 2. Icon Dosyaları
console.log('\n🎨 Icon Dosyaları:');
checkFile('build/icon.png');
checkFile('build/icon-256.png');
checkFile('build/tray.png');
checkFile('build/tray@2x.png');
checkFile('build/installerHeader.png');
checkFile('build/installerSidebar.png');
checkFile('build/generate-icons.cjs');

// 3. Konfigürasyon Dosyaları
console.log('\n⚙️  Konfigürasyon Dosyaları:');
checkFile('electron-builder.json');
checkFile('tsconfig.electron.json');
checkFile('package.json');

// 4. TypeScript Tip Tanımları
console.log('\n📝 TypeScript Dosyaları:');
checkFile('electron/main.ts');
checkFile('electron/preload.ts');
checkFile('electron/types/electron.d.ts');

// 5. Frontend Entegrasyon
console.log('\n🔌 Frontend Entegrasyon:');
checkFile('client/lib/utils/electron.ts');
checkFile('client/hooks/useElectron.ts');
checkFile('client/components/ElectronTitleBar.tsx');
checkFile('client/components/ElectronUpdateNotification.tsx');

// 6. Dokümantasyon
console.log('\n📚 Dokümantasyon:');
checkFile('electron.md');
checkFile('docs/ELECTRON-BUILD.md');

// 7. Opsiyonel - Release klasörü
console.log('\n📦 Release Klasörü (Opsiyonel):');
checkDirectory('release', false);
checkFile('release/Rehber360-Setup-1.0.0.exe', false);
checkFile('release/Rehber360-Portable-1.0.0.exe', false);

// Sonuç
console.log('\n' + '='.repeat(50));
if (errorCount === 0 && warningCount === 0) {
  console.log('✅ Tüm validation kontrollerinden geçti!');
  process.exit(0);
} else {
  console.log(`⚠️  ${errorCount} hata, ${warningCount} uyarı bulundu.`);
  if (errorCount > 0) {
    console.log('\n❌ Kritik dosyalar eksik! Build işlemini tamamlayın.');
    process.exit(1);
  } else {
    console.log('\n✅ Temel dosyalar mevcut. Uyarılar opsiyonel dosyalar.');
    process.exit(0);
  }
}
