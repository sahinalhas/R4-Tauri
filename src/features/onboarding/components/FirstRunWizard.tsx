import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { Progress } from '@/components/ui/progress';
import { Loader2, CheckCircle2, AlertCircle, Database, User, Sparkles, FileCheck } from 'lucide-react';
import { toast } from 'sonner';

type WizardStep = 'welcome' | 'database' | 'admin' | 'ai-provider' | 'complete';

interface MigrationReport {
  success: boolean;
  students_migrated: number;
  counseling_sessions_migrated: number;
  academic_records_migrated: number;
  behavior_records_migrated: number;
  documents_migrated: number;
  settings_migrated: number;
  errors: string[];
  warnings: string[];
}

export function FirstRunWizard() {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<WizardStep>('welcome');
  const [loading, setLoading] = useState(false);

  const [dbChoice, setDbChoice] = useState<'new' | 'migrate'>('new');
  const [electronDbPath, setElectronDbPath] = useState('');
  const [migrationReport, setMigrationReport] = useState<MigrationReport | null>(null);

  const [adminEmail, setAdminEmail] = useState('rehber@okul.edu.tr');
  const [adminPassword, setAdminPassword] = useState('');
  const [adminName, setAdminName] = useState('');
  const [adminSurname, setAdminSurname] = useState('');

  const [aiProvider, setAiProvider] = useState<'gemini' | 'openai' | 'ollama' | 'skip'>('gemini');
  const [apiKey, setApiKey] = useState('');

  const steps: WizardStep[] = ['welcome', 'database', 'admin', 'ai-provider', 'complete'];
  const currentStepIndex = steps.indexOf(currentStep);
  const progress = ((currentStepIndex + 1) / steps.length) * 100;

  const detectElectronDb = async () => {
    try {
      const path = await invoke<string | null>('detect_electron_database');
      if (path) {
        setElectronDbPath(path);
        toast.success('Eski veritabanı bulundu!', {
          description: path
        });
      } else {
        toast.info('Eski veritabanı bulunamadı', {
          description: 'Yeni veritabanı oluşturabilirsiniz'
        });
      }
    } catch (error) {
      console.error('Error detecting Electron database:', error);
    }
  };

  const validateElectronDb = async () => {
    if (!electronDbPath) return false;
    try {
      const isValid = await invoke<boolean>('validate_electron_database', { dbPath: electronDbPath });
      return isValid;
    } catch (error) {
      console.error('Error validating database:', error);
      return false;
    }
  };

  const migrateFromElectron = async () => {
    setLoading(true);
    try {
      const isValid = await validateElectronDb();
      if (!isValid) {
        toast.error('Geçersiz veritabanı', {
          description: 'Seçilen dosya geçerli bir Rehber360 veritabanı değil'
        });
        setLoading(false);
        return;
      }

      const report = await invoke<MigrationReport>('migrate_from_electron', { 
        oldDbPath: electronDbPath 
      });

      setMigrationReport(report);

      if (report.success) {
        toast.success('Veri aktarımı başarılı!', {
          description: `${report.students_migrated} öğrenci, ${report.counseling_sessions_migrated} görüşme aktarıldı`
        });
        handleNext();
      } else {
        toast.error('Veri aktarımı başarısız', {
          description: report.errors.join(', ')
        });
      }
    } catch (error) {
      console.error('Migration error:', error);
      toast.error('Veri aktarımı sırasında hata oluştu');
    } finally {
      setLoading(false);
    }
  };

  const createAdmin = async () => {
    setLoading(true);
    try {
      await invoke('create_initial_admin', {
        email: adminEmail,
        password: adminPassword,
        name: adminName,
        surname: adminSurname,
      });
      toast.success('Yönetici hesabı oluşturuldu');
      handleNext();
    } catch (error) {
      console.error('Admin creation error:', error);
      toast.error('Hesap oluşturma hatası');
    } finally {
      setLoading(false);
    }
  };

  const saveAiSettings = async () => {
    setLoading(true);
    try {
      if (aiProvider !== 'skip') {
        await invoke('update_ai_provider', {
          provider: aiProvider,
          apiKey: apiKey || undefined,
        });
        toast.success('AI ayarları kaydedildi');
      }
      handleNext();
    } catch (error) {
      console.error('AI settings error:', error);
      toast.error('AI ayarları kaydedilemedi');
    } finally {
      setLoading(false);
    }
  };

  const handleNext = () => {
    const nextIndex = currentStepIndex + 1;
    if (nextIndex < steps.length) {
      setCurrentStep(steps[nextIndex]);
    }
  };

  const handleBack = () => {
    const prevIndex = currentStepIndex - 1;
    if (prevIndex >= 0) {
      setCurrentStep(steps[prevIndex]);
    }
  };

  const handleComplete = () => {
    localStorage.setItem('rehber360_first_run_completed', 'true');
    toast.success('Kurulum tamamlandı! Hoş geldiniz! 🎉');
    navigate('/dashboard');
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
      <Card className="w-full max-w-2xl shadow-2xl">
        <CardHeader>
          <div className="flex items-center justify-between mb-4">
            <CardTitle className="text-2xl">Rehber360 Kurulum</CardTitle>
            <div className="text-sm text-muted-foreground">
              Adım {currentStepIndex + 1} / {steps.length}
            </div>
          </div>
          <Progress value={progress} className="h-2" />
        </CardHeader>

        <CardContent className="space-y-6">
          {currentStep === 'welcome' && (
            <div className="text-center space-y-6 py-8">
              <div className="w-20 h-20 bg-blue-500 rounded-full flex items-center justify-center mx-auto">
                <Sparkles className="w-10 h-10 text-white" />
              </div>
              <div>
                <h2 className="text-3xl font-bold mb-2">Rehber360'a Hoş Geldiniz!</h2>
                <p className="text-muted-foreground text-lg">
                  Modern, AI destekli öğrenci rehberlik sistemi
                </p>
              </div>
              <ul className="text-left space-y-2 max-w-md mx-auto">
                <li className="flex items-center gap-2">
                  <CheckCircle2 className="w-5 h-5 text-green-500" />
                  <span>AI destekli profil analizi</span>
                </li>
                <li className="flex items-center gap-2">
                  <CheckCircle2 className="w-5 h-5 text-green-500" />
                  <span>Kapsamlı öğrenci takibi</span>
                </li>
                <li className="flex items-center gap-2">
                  <CheckCircle2 className="w-5 h-5 text-green-500" />
                  <span>Detaylı raporlama</span>
                </li>
                <li className="flex items-center gap-2">
                  <CheckCircle2 className="w-5 h-5 text-green-500" />
                  <span>%93 daha küçük, %50 daha hızlı</span>
                </li>
              </ul>
              <Button size="lg" onClick={handleNext}>
                Kuruluma Başla
              </Button>
            </div>
          )}

          {currentStep === 'database' && (
            <div className="space-y-6">
              <div className="flex items-center gap-3">
                <Database className="w-8 h-8 text-blue-500" />
                <div>
                  <h3 className="text-xl font-semibold">Veritabanı Kurulumu</h3>
                  <p className="text-sm text-muted-foreground">
                    Yeni veritabanı oluşturun veya eski verilerinizi aktarın
                  </p>
                </div>
              </div>

              <RadioGroup value={dbChoice} onValueChange={(v) => setDbChoice(v as 'new' | 'migrate')}>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="new" id="new" />
                  <Label htmlFor="new" className="cursor-pointer flex-1">
                    <div className="font-medium">Yeni Veritabanı</div>
                    <div className="text-sm text-muted-foreground">Temiz bir başlangıç yapın</div>
                  </Label>
                </div>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="migrate" id="migrate" />
                  <Label htmlFor="migrate" className="cursor-pointer flex-1">
                    <div className="font-medium">Eski Verilerimi Aktar (Electron)</div>
                    <div className="text-sm text-muted-foreground">Mevcut verilerinizi taşıyın</div>
                  </Label>
                </div>
              </RadioGroup>

              {dbChoice === 'migrate' && (
                <div className="space-y-4 p-4 border rounded-lg bg-muted/50">
                  <div className="space-y-2">
                    <Label htmlFor="db-path">Eski Veritabanı Yolu</Label>
                    <div className="flex gap-2">
                      <Input
                        id="db-path"
                        value={electronDbPath}
                        onChange={(e) => setElectronDbPath(e.target.value)}
                        placeholder="C:\Users\...\rehber360-electron\database.db"
                      />
                      <Button variant="outline" onClick={detectElectronDb}>
                        Otomatik Bul
                      </Button>
                    </div>
                  </div>

                  {migrationReport && (
                    <div className="space-y-2 p-3 bg-background rounded-lg">
                      <div className="font-medium flex items-center gap-2">
                        {migrationReport.success ? (
                          <CheckCircle2 className="w-5 h-5 text-green-500" />
                        ) : (
                          <AlertCircle className="w-5 h-5 text-red-500" />
                        )}
                        Aktarım Raporu
                      </div>
                      <div className="text-sm space-y-1">
                        <div>Öğrenci: {migrationReport.students_migrated}</div>
                        <div>Görüşme: {migrationReport.counseling_sessions_migrated}</div>
                        <div>Akademik Kayıt: {migrationReport.academic_records_migrated}</div>
                        <div>Davranış Kaydı: {migrationReport.behavior_records_migrated}</div>
                        <div>Belge: {migrationReport.documents_migrated}</div>
                      </div>
                    </div>
                  )}

                  <Button 
                    onClick={migrateFromElectron} 
                    disabled={!electronDbPath || loading}
                    className="w-full"
                  >
                    {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                    Verileri Aktar
                  </Button>
                </div>
              )}

              <div className="flex gap-2">
                <Button variant="outline" onClick={handleBack}>Geri</Button>
                <Button 
                  onClick={handleNext} 
                  disabled={dbChoice === 'migrate' && !migrationReport?.success}
                  className="flex-1"
                >
                  Devam Et
                </Button>
              </div>
            </div>
          )}

          {currentStep === 'admin' && (
            <div className="space-y-6">
              <div className="flex items-center gap-3">
                <User className="w-8 h-8 text-blue-500" />
                <div>
                  <h3 className="text-xl font-semibold">Yönetici Hesabı</h3>
                  <p className="text-sm text-muted-foreground">
                    İlk yönetici hesabınızı oluşturun
                  </p>
                </div>
              </div>

              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="name">Ad</Label>
                    <Input
                      id="name"
                      value={adminName}
                      onChange={(e) => setAdminName(e.target.value)}
                      placeholder="Ahmet"
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="surname">Soyad</Label>
                    <Input
                      id="surname"
                      value={adminSurname}
                      onChange={(e) => setAdminSurname(e.target.value)}
                      placeholder="Yılmaz"
                    />
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="email">E-posta</Label>
                  <Input
                    id="email"
                    type="email"
                    value={adminEmail}
                    onChange={(e) => setAdminEmail(e.target.value)}
                    placeholder="rehber@okul.edu.tr"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="password">Şifre</Label>
                  <Input
                    id="password"
                    type="password"
                    value={adminPassword}
                    onChange={(e) => setAdminPassword(e.target.value)}
                    placeholder="Güçlü bir şifre belirleyin"
                  />
                  <p className="text-xs text-muted-foreground">
                    En az 8 karakter, büyük/küçük harf ve sayı içermelidir
                  </p>
                </div>
              </div>

              <div className="flex gap-2">
                <Button variant="outline" onClick={handleBack}>Geri</Button>
                <Button 
                  onClick={createAdmin}
                  disabled={!adminEmail || !adminPassword || adminPassword.length < 8 || !adminName || !adminSurname || loading}
                  className="flex-1"
                >
                  {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                  Hesap Oluştur
                </Button>
              </div>
            </div>
          )}

          {currentStep === 'ai-provider' && (
            <div className="space-y-6">
              <div className="flex items-center gap-3">
                <Sparkles className="w-8 h-8 text-blue-500" />
                <div>
                  <h3 className="text-xl font-semibold">AI Sağlayıcı</h3>
                  <p className="text-sm text-muted-foreground">
                    AI destekli analiz için bir sağlayıcı seçin (sonra değiştirilebilir)
                  </p>
                </div>
              </div>

              <RadioGroup value={aiProvider} onValueChange={(v) => setAiProvider(v as any)}>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="gemini" id="gemini" />
                  <Label htmlFor="gemini" className="cursor-pointer flex-1">
                    <div className="font-medium">Google Gemini (Önerilen)</div>
                    <div className="text-sm text-muted-foreground">Ücretsiz kota yüksek, hızlı</div>
                  </Label>
                </div>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="openai" id="openai" />
                  <Label htmlFor="openai" className="cursor-pointer flex-1">
                    <div className="font-medium">OpenAI (GPT-4)</div>
                    <div className="text-sm text-muted-foreground">En güçlü, ücretli</div>
                  </Label>
                </div>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="ollama" id="ollama" />
                  <Label htmlFor="ollama" className="cursor-pointer flex-1">
                    <div className="font-medium">Ollama (Lokal)</div>
                    <div className="text-sm text-muted-foreground">%100 gizli, kurulum gerektirir</div>
                  </Label>
                </div>
                <div className="flex items-center space-x-2 p-4 border rounded-lg hover:bg-accent cursor-pointer">
                  <RadioGroupItem value="skip" id="skip" />
                  <Label htmlFor="skip" className="cursor-pointer flex-1">
                    <div className="font-medium">Şimdi Atlansın</div>
                    <div className="text-sm text-muted-foreground">Daha sonra ayarlayın</div>
                  </Label>
                </div>
              </RadioGroup>

              {aiProvider !== 'skip' && aiProvider !== 'ollama' && (
                <div className="space-y-2">
                  <Label htmlFor="api-key">API Anahtarı</Label>
                  <Input
                    id="api-key"
                    type="password"
                    value={apiKey}
                    onChange={(e) => setApiKey(e.target.value)}
                    placeholder={aiProvider === 'gemini' ? 'AIza...' : 'sk-...'}
                  />
                  <p className="text-xs text-muted-foreground">
                    {aiProvider === 'gemini' ? (
                      <a href="https://ai.google.dev" target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">
                        ai.google.dev
                      </a>
                    ) : (
                      <a href="https://platform.openai.com" target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">
                        platform.openai.com
                      </a>
                    )}
                    {' '}adresinden ücretsiz API anahtarı alabilirsiniz
                  </p>
                </div>
              )}

              {aiProvider === 'ollama' && (
                <div className="p-4 border rounded-lg bg-blue-50 dark:bg-blue-950">
                  <p className="text-sm">
                    <strong>Ollama kurulumu:</strong><br />
                    1. <a href="https://ollama.ai" target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">ollama.ai</a> adresinden indirin<br />
                    2. Terminal: <code className="bg-muted px-1 rounded">ollama pull llama3.1</code><br />
                    3. Rehber360 otomatik olarak bağlanacak
                  </p>
                </div>
              )}

              <div className="flex gap-2">
                <Button variant="outline" onClick={handleBack}>Geri</Button>
                <Button 
                  onClick={saveAiSettings}
                  disabled={loading || (aiProvider !== 'skip' && aiProvider !== 'ollama' && !apiKey)}
                  className="flex-1"
                >
                  {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                  Devam Et
                </Button>
              </div>
            </div>
          )}

          {currentStep === 'complete' && (
            <div className="text-center space-y-6 py-8">
              <div className="w-20 h-20 bg-green-500 rounded-full flex items-center justify-center mx-auto">
                <FileCheck className="w-10 h-10 text-white" />
              </div>
              <div>
                <h2 className="text-3xl font-bold mb-2">Kurulum Tamamlandı!</h2>
                <p className="text-muted-foreground text-lg">
                  Rehber360 kullanıma hazır
                </p>
              </div>
              <div className="space-y-2 text-left max-w-md mx-auto text-sm">
                <p>✅ Veritabanı hazır</p>
                <p>✅ Yönetici hesabı oluşturuldu</p>
                <p>✅ AI ayarları yapılandırıldı</p>
              </div>
              <Button size="lg" onClick={handleComplete}>
                Rehber360'ı Başlat
              </Button>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
