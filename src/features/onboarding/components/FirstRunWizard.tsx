import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';

type WizardStep = 'welcome' | 'database' | 'admin' | 'complete';

interface MigrationReport {
  success: boolean;
  students_migrated: number;
  counseling_sessions_migrated: number;
  academic_records_migrated: number;
  errors: string[];
}

export function FirstRunWizard() {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<WizardStep>('welcome');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const [dbChoice, setDbChoice] = useState<'new' | 'migrate'>('new');
  const [electronDbPath, setElectronDbPath] = useState('');
  const [migrationReport, setMigrationReport] = useState<MigrationReport | null>(null);

  const [adminEmail, setAdminEmail] = useState('rehber@okul.edu.tr');
  const [adminPassword, setAdminPassword] = useState('');
  const [adminName, setAdminName] = useState('');
  const [adminSurname, setAdminSurname] = useState('');

  const validatePassword = (pwd: string): string | null => {
    if (pwd.length < 8) return 'En az 8 karakter gerekli';
    if (!/[A-Z]/.test(pwd)) return 'En az bir büyük harf gerekli';
    if (!/[a-z]/.test(pwd)) return 'En az bir küçük harf gerekli';
    if (!/[0-9]/.test(pwd)) return 'En az bir rakam gerekli';
    return null;
  };

  const passwordError = adminPassword ? validatePassword(adminPassword) : null;
  const isAdminFormValid = adminName.trim() && adminSurname.trim() && adminEmail.trim() && adminPassword && !passwordError;

  const detectElectronDb = async () => {
    try {
      const path = await invoke<string | null>('detect_electron_database');
      if (path) {
        setElectronDbPath(path);
        setError('');
      } else {
        setError('Eski veritabanı bulunamadı');
      }
    } catch (err) {
      setError('Eski veritabanı bulunamadı');
    }
  };

  const migrateFromElectron = async () => {
    setLoading(true);
    setError('');
    try {
      const report = await invoke<MigrationReport>('migrate_from_electron', { 
        oldDbPath: electronDbPath 
      });
      setMigrationReport(report);
      if (report.success) {
        setError('');
        setCurrentStep('complete');
      } else {
        setError(report.errors.join(', '));
      }
    } catch (err) {
      setError('Veri aktarımı sırasında hata oluştu: ' + String(err));
    } finally {
      setLoading(false);
    }
  };

  const createAdmin = async () => {
    setLoading(true);
    setError('');
    try {
      await invoke('create_initial_admin', {
        email: adminEmail,
        password: adminPassword,
        name: adminName,
        surname: adminSurname,
      });
      setError('');
      setCurrentStep('complete');
    } catch (err) {
      setError('Hesap oluşturma hatası: ' + String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleComplete = () => {
    localStorage.setItem('rehber360_first_run_completed', 'true');
    navigate('/dashboard');
  };

  return (
    <div style={{
      minHeight: '100vh',
      background: 'linear-gradient(to bottom right, #eff6ff, #eef2ff)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '2rem'
    }}>
      <div style={{
        width: '100%',
        maxWidth: '600px',
        background: 'white',
        borderRadius: '12px',
        boxShadow: '0 20px 25px -5px rgba(0,0,0,0.1)',
        padding: '2rem'
      }}>
        <h1 style={{ fontSize: '2rem', fontWeight: 'bold', marginBottom: '1rem' }}>
          Rehber360 Kurulum
        </h1>
        
        {error && (
          <div style={{ 
            background: '#fee2e2', 
            color: '#991b1b', 
            padding: '1rem', 
            borderRadius: '8px',
            marginBottom: '1rem'
          }}>
            {error}
          </div>
        )}

        {currentStep === 'welcome' && (
          <div style={{ textAlign: 'center', padding: '2rem' }}>
            <h2 style={{ fontSize: '1.5rem', marginBottom: '1rem' }}>Hoş Geldiniz!</h2>
            <p style={{ marginBottom: '2rem', color: '#6b7280' }}>
              Modern, AI destekli öğrenci rehberlik sistemi
            </p>
            <ul style={{ textAlign: 'left', marginBottom: '2rem', listStyle: 'none', padding: 0 }}>
              <li style={{ marginBottom: '0.5rem' }}>✓ AI destekli profil analizi</li>
              <li style={{ marginBottom: '0.5rem' }}>✓ Kapsamlı öğrenci takibi</li>
              <li style={{ marginBottom: '0.5rem' }}>✓ Detaylı raporlama</li>
              <li style={{ marginBottom: '0.5rem' }}>✓ %93 daha küçük, %50 daha hızlı</li>
            </ul>
            <button
              onClick={() => setCurrentStep('database')}
              style={{
                background: '#3b82f6',
                color: 'white',
                padding: '0.75rem 2rem',
                borderRadius: '8px',
                border: 'none',
                cursor: 'pointer',
                fontSize: '1rem',
                fontWeight: '600'
              }}
            >
              Kuruluma Başla
            </button>
          </div>
        )}

        {currentStep === 'database' && (
          <div>
            <h2 style={{ fontSize: '1.5rem', marginBottom: '1rem' }}>Veritabanı Kurulumu</h2>
            
            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'flex', alignItems: 'center', padding: '1rem', border: '1px solid #e5e7eb', borderRadius: '8px', marginBottom: '0.5rem', cursor: 'pointer' }}>
                <input
                  type="radio"
                  checked={dbChoice === 'new'}
                  onChange={() => setDbChoice('new')}
                  style={{ marginRight: '0.5rem' }}
                />
                <div>
                  <div style={{ fontWeight: '600' }}>Yeni Veritabanı</div>
                  <div style={{ fontSize: '0.875rem', color: '#6b7280' }}>Temiz bir başlangıç yapın</div>
                </div>
              </label>

              <label style={{ display: 'flex', alignItems: 'center', padding: '1rem', border: '1px solid #e5e7eb', borderRadius: '8px', cursor: 'pointer' }}>
                <input
                  type="radio"
                  checked={dbChoice === 'migrate'}
                  onChange={() => setDbChoice('migrate')}
                  style={{ marginRight: '0.5rem' }}
                />
                <div>
                  <div style={{ fontWeight: '600' }}>Eski Verilerimi Aktar</div>
                  <div style={{ fontSize: '0.875rem', color: '#6b7280' }}>Electron'dan veri taşıyın</div>
                </div>
              </label>
            </div>

            {dbChoice === 'migrate' && (
              <div style={{ marginTop: '1rem', padding: '1rem', background: '#f9fafb', borderRadius: '8px' }}>
                <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600' }}>
                  Eski Veritabanı Yolu:
                </label>
                <input
                  type="text"
                  value={electronDbPath}
                  onChange={(e) => setElectronDbPath(e.target.value)}
                  placeholder="C:\Users\...\rehber360-electron\database.db"
                  style={{
                    width: '100%',
                    padding: '0.5rem',
                    border: '1px solid #d1d5db',
                    borderRadius: '6px',
                    marginBottom: '0.5rem'
                  }}
                />
                <button
                  onClick={detectElectronDb}
                  style={{
                    background: '#6b7280',
                    color: 'white',
                    padding: '0.5rem 1rem',
                    borderRadius: '6px',
                    border: 'none',
                    cursor: 'pointer',
                    marginRight: '0.5rem',
                    marginBottom: '0.5rem'
                  }}
                >
                  Otomatik Bul
                </button>
                <button
                  onClick={migrateFromElectron}
                  disabled={!electronDbPath || loading}
                  style={{
                    background: electronDbPath && !loading ? '#3b82f6' : '#d1d5db',
                    color: 'white',
                    padding: '0.5rem 1rem',
                    borderRadius: '6px',
                    border: 'none',
                    cursor: electronDbPath && !loading ? 'pointer' : 'not-allowed'
                  }}
                >
                  {loading ? 'Aktarılıyor...' : 'Verileri Aktar'}
                </button>
                
                {migrationReport && (
                  <div style={{ marginTop: '1rem', padding: '1rem', background: 'white', borderRadius: '6px' }}>
                    <div style={{ fontWeight: '600', marginBottom: '0.5rem', color: migrationReport.success ? '#059669' : '#dc2626' }}>
                      {migrationReport.success ? '✓ Aktarım Başarılı' : '✗ Aktarım Başarısız'}
                    </div>
                    <div style={{ fontSize: '0.875rem' }}>
                      <div>Öğrenci: {migrationReport.students_migrated}</div>
                      <div>Görüşme: {migrationReport.counseling_sessions_migrated}</div>
                      <div>Akademik Kayıt: {migrationReport.academic_records_migrated}</div>
                    </div>
                  </div>
                )}
              </div>
            )}

            <div style={{ marginTop: '1.5rem', display: 'flex', gap: '0.5rem' }}>
              <button
                onClick={() => setCurrentStep('welcome')}
                style={{
                  background: '#f3f4f6',
                  color: '#374151',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '8px',
                  border: 'none',
                  cursor: 'pointer',
                  fontWeight: '600'
                }}
              >
                Geri
              </button>
              <button
                onClick={() => {
                  if (dbChoice === 'migrate' && migrationReport?.success) {
                    setCurrentStep('complete');
                  } else if (dbChoice === 'new') {
                    setCurrentStep('admin');
                  }
                }}
                disabled={dbChoice === 'migrate' && !migrationReport?.success}
                style={{
                  flex: 1,
                  background: (dbChoice === 'migrate' && !migrationReport?.success) ? '#d1d5db' : '#3b82f6',
                  color: 'white',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '8px',
                  border: 'none',
                  cursor: (dbChoice === 'migrate' && !migrationReport?.success) ? 'not-allowed' : 'pointer',
                  fontWeight: '600'
                }}
              >
                Devam Et
              </button>
            </div>
          </div>
        )}

        {currentStep === 'admin' && (
          <div>
            <h2 style={{ fontSize: '1.5rem', marginBottom: '1rem' }}>Yönetici Hesabı</h2>
            <p style={{ marginBottom: '1rem', color: '#6b7280', fontSize: '0.875rem' }}>
              İlk yönetici hesabınızı oluşturun
            </p>

            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600' }}>Ad:</label>
              <input
                type="text"
                value={adminName}
                onChange={(e) => setAdminName(e.target.value)}
                placeholder="Ahmet"
                style={{
                  width: '100%',
                  padding: '0.5rem',
                  border: '1px solid #d1d5db',
                  borderRadius: '6px'
                }}
              />
            </div>

            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600' }}>Soyad:</label>
              <input
                type="text"
                value={adminSurname}
                onChange={(e) => setAdminSurname(e.target.value)}
                placeholder="Yılmaz"
                style={{
                  width: '100%',
                  padding: '0.5rem',
                  border: '1px solid #d1d5db',
                  borderRadius: '6px'
                }}
              />
            </div>

            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600' }}>E-posta:</label>
              <input
                type="email"
                value={adminEmail}
                onChange={(e) => setAdminEmail(e.target.value)}
                placeholder="rehber@okul.edu.tr"
                style={{
                  width: '100%',
                  padding: '0.5rem',
                  border: '1px solid #d1d5db',
                  borderRadius: '6px'
                }}
              />
            </div>

            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '600' }}>Şifre:</label>
              <input
                type="password"
                value={adminPassword}
                onChange={(e) => setAdminPassword(e.target.value)}
                placeholder="En az 8 karakter"
                style={{
                  width: '100%',
                  padding: '0.5rem',
                  border: '1px solid #d1d5db',
                  borderRadius: '6px'
                }}
              />
              {passwordError && adminPassword && (
                <p style={{ fontSize: '0.75rem', color: '#dc2626', marginTop: '0.25rem' }}>
                  {passwordError}
                </p>
              )}
              {!passwordError && adminPassword && (
                <p style={{ fontSize: '0.75rem', color: '#059669', marginTop: '0.25rem' }}>
                  ✓ Güçlü şifre
                </p>
              )}
              {!adminPassword && (
                <p style={{ fontSize: '0.75rem', color: '#6b7280', marginTop: '0.25rem' }}>
                  En az 8 karakter, büyük/küçük harf ve sayı içermelidir
                </p>
              )}
            </div>

            <div style={{ marginTop: '1.5rem', display: 'flex', gap: '0.5rem' }}>
              <button
                onClick={() => setCurrentStep('database')}
                style={{
                  background: '#f3f4f6',
                  color: '#374151',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '8px',
                  border: 'none',
                  cursor: 'pointer',
                  fontWeight: '600'
                }}
              >
                Geri
              </button>
              <button
                onClick={createAdmin}
                disabled={!isAdminFormValid || loading}
                style={{
                  flex: 1,
                  background: (!isAdminFormValid || loading) ? '#d1d5db' : '#3b82f6',
                  color: 'white',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '8px',
                  border: 'none',
                  cursor: (!isAdminFormValid || loading) ? 'not-allowed' : 'pointer',
                  fontWeight: '600'
                }}
              >
                {loading ? 'Oluşturuluyor...' : 'Hesap Oluştur'}
              </button>
            </div>
          </div>
        )}

        {currentStep === 'complete' && (
          <div style={{ textAlign: 'center', padding: '2rem' }}>
            <div style={{
              width: '80px',
              height: '80px',
              background: '#10b981',
              borderRadius: '50%',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              margin: '0 auto 1rem'
            }}>
              <span style={{ fontSize: '3rem', color: 'white' }}>✓</span>
            </div>
            <h2 style={{ fontSize: '1.5rem', marginBottom: '1rem' }}>Kurulum Tamamlandı!</h2>
            <p style={{ marginBottom: '0.5rem', color: '#6b7280' }}>
              Rehber360 kullanıma hazır
            </p>
            <div style={{ margin: '1.5rem auto', textAlign: 'left', maxWidth: '400px', padding: '1rem', background: '#f0fdf4', borderRadius: '8px' }}>
              <p style={{ fontSize: '0.875rem', marginBottom: '0.5rem', color: '#059669', fontWeight: '600' }}>✓ Kurulum Başarılı</p>
              <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>• Veritabanı hazır</p>
              {migrationReport && migrationReport.success && (
                <>
                  <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>
                    • {migrationReport.students_migrated} öğrenci aktarıldı
                  </p>
                  <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>
                    • {migrationReport.counseling_sessions_migrated} görüşme aktarıldı
                  </p>
                  <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>
                    • {migrationReport.academic_records_migrated} akademik kayıt aktarıldı
                  </p>
                  {migrationReport.errors.length > 0 && (
                    <p style={{ fontSize: '0.75rem', marginTop: '0.5rem', color: '#dc2626' }}>
                      Uyarı: Bazı veriler aktarılamadı
                    </p>
                  )}
                </>
              )}
              {!migrationReport && (
                <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>• Yönetici hesabı oluşturuldu</p>
              )}
            </div>
            <button
              onClick={handleComplete}
              style={{
                background: '#3b82f6',
                color: 'white',
                padding: '0.75rem 2rem',
                borderRadius: '8px',
                border: 'none',
                cursor: 'pointer',
                fontSize: '1rem',
                fontWeight: '600'
              }}
            >
              Rehber360'ı Başlat
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
