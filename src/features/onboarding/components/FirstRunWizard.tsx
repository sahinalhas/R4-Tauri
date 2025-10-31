import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';

type WizardStep = 'welcome' | 'admin' | 'complete';

export function FirstRunWizard() {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<WizardStep>('welcome');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

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
              onClick={() => setCurrentStep('admin')}
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
              <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>• Yönetici hesabı oluşturuldu</p>
              <p style={{ fontSize: '0.875rem', marginBottom: '0.25rem' }}>• Tauri desktop uygulaması hazır</p>
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
