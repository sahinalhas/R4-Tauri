import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";
import { fileURLToPath } from "url";

// Tauri Desktop Application - Vite Configuration
// https://vitejs.dev/config/

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig(({ mode }) => ({
  server: {
    host: "localhost",
    port: 5000,
    strictPort: true,
    fs: {
      allow: [".", "./client", "./shared"],
    },
  },
  clearScreen: false,
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    outDir: "dist/spa",
    minify: mode === 'production' ? 'esbuild' : false,
    target: 'es2021',
    cssCodeSplit: true,
    cssMinify: mode === 'production',
    assetsInlineLimit: 4096,
    rollupOptions: {
      output: {
        manualChunks: (id: string) => {
          if (id.includes('node_modules')) {
            if (id.includes('react') || id.includes('react-dom')) {
              return 'vendor-react-core';
            }
            if (id.includes('react-router')) {
              return 'vendor-react-router';
            }
            if (id.includes('@tanstack/react-query')) {
              return 'vendor-query';
            }
            if (id.includes('@tanstack/react-virtual')) {
              return 'vendor-virtual';
            }
            if (id.includes('@radix-ui')) {
              const component = id.match(/@radix-ui\/react-([^/]+)/)?.[1];
              if (component) {
                const commonComponents = ['slot', 'primitive', 'use-callback-ref', 'use-controllable-state', 'use-layout-effect', 'use-escape-keydown', 'use-rect'];
                if (commonComponents.includes(component)) {
                  return 'vendor-radix-core';
                }
                const frequentComponents = ['dialog', 'dropdown-menu', 'select', 'toast', 'tabs'];
                if (frequentComponents.includes(component)) {
                  return 'vendor-radix-frequent';
                }
                return `vendor-radix-${component}`;
              }
              return 'vendor-radix-core';
            }
            if (id.includes('recharts')) {
              return 'charts';
            }
            if (id.includes('lucide-react')) {
              return 'vendor-icons';
            }
            if (id.includes('react-markdown') || id.includes('remark') || id.includes('rehype')) {
              return 'markdown';
            }
            if (id.includes('date-fns')) {
              return 'vendor-date';
            }
            if (id.includes('framer-motion')) {
              return 'animation';
            }
            if (id.includes('react-hook-form') || id.includes('@hookform')) {
              return 'vendor-forms';
            }
            if (id.includes('xlsx') || id.includes('jspdf')) {
              return 'export';
            }
            if (id.includes('zod')) {
              return 'vendor-validation';
            }
            if (id.includes('node_modules')) {
              return 'vendor-common';
            }
          }
        },
        assetFileNames: (assetInfo: { name?: string }) => {
          const info = assetInfo.name?.split('.');
          const ext = info?.[info.length - 1];
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(ext || '')) {
            return `assets/images/[name]-[hash][extname]`;
          } else if (/woff2?|ttf|eot/i.test(ext || '')) {
            return `assets/fonts/[name]-[hash][extname]`;
          }
          return `assets/[name]-[hash][extname]`;
        },
        chunkFileNames: 'assets/js/[name]-[hash].js',
        entryFileNames: 'assets/js/[name]-[hash].js',
      },
    },
    chunkSizeWarningLimit: 2000,
    sourcemap: mode === 'development',
    reportCompressedSize: mode === 'production',
  },
  plugins: [
    react(),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./client"),
      "@shared": path.resolve(__dirname, "./shared"),
      "@/assets": path.resolve(__dirname, "./client/assets"),
    },
  },
}));
