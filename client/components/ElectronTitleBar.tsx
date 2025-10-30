import { useState, useEffect } from 'react';
import { Minus, Square, X } from 'lucide-react';
import { Button } from './atoms/Button';
import { useElectronWindow } from '../hooks/useElectron';
import { isElectron } from '../lib/utils/electron';

export function ElectronTitleBar() {
  const { minimize, maximize, close, isMaximized: checkMaximized } = useElectronWindow();
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    if (!isElectron()) return;

    checkMaximized().then(setIsMaximized);

    const api = window.electronAPI;
    if (!api) return;

    const cleanup = api.onWindowMaximized((_, maximized) => {
      setIsMaximized(maximized);
    });

    return cleanup;
  }, [checkMaximized]);

  if (!isElectron()) {
    return null;
  }

  return (
    <div className="flex items-center justify-between h-8 bg-background border-b border-border px-4 select-none" style={{ WebkitAppRegion: 'drag' } as React.CSSProperties}>
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-1">
          <div className="w-3 h-3 rounded-full bg-primary/20" />
          <span className="text-sm font-medium text-foreground/80">Rehber360</span>
        </div>
      </div>

      <div className="flex items-center gap-1" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
        <Button
          variant="ghost"
          size="icon"
          className="h-7 w-7 hover:bg-muted"
          onClick={() => minimize()}
          aria-label="Minimize"
        >
          <Minus className="h-4 w-4" />
        </Button>
        
        <Button
          variant="ghost"
          size="icon"
          className="h-7 w-7 hover:bg-muted"
          onClick={() => maximize()}
          aria-label={isMaximized ? 'Restore' : 'Maximize'}
        >
          <Square className="h-4 w-4" />
        </Button>
        
        <Button
          variant="ghost"
          size="icon"
          className="h-7 w-7 hover:bg-destructive hover:text-destructive-foreground"
          onClick={() => close()}
          aria-label="Close"
        >
          <X className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
