import "./global.css";

import { createRoot } from "react-dom/client";
import App from "./App";
import { apiClient } from "./lib/api/core/client";
import { isElectron } from "./lib/utils/electron";

async function initializeApp() {
  if (isElectron()) {
    await apiClient.initialize();
    console.log('[Electron] API client initialized');
  }
  
  createRoot(document.getElementById("root")!).render(<App />);
}

initializeApp().catch(error => {
  console.error('Failed to initialize app:', error);
  createRoot(document.getElementById("root")!).render(<App />);
});