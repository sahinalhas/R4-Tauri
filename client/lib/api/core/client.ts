import { toast } from "sonner";
import { 
  InterceptorManager, 
  ToastConfig,
  createDefaultErrorInterceptor,
  createDefaultSuccessInterceptor,
  RequestInterceptor
} from "./interceptors";
import { 
  parseApiError, 
  handleNetworkError, 
  showErrorToast as displayErrorToast 
} from "./error-handler";
import { ApiError, isApiErrorResponse } from "../../types/api-types";
import { transport, TransportConfig } from "./transport";
import { isDesktopMode } from "@/lib/utils/platform";

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';

export interface ApiRequestConfig<T = unknown> extends ToastConfig {
  method?: HttpMethod;
  body?: T;
  headers?: Record<string, string>;
  timeout?: number;
  skipCsrf?: boolean; // Allow skipping CSRF for public endpoints
}

export interface ApiResponse<T> {
  data: T;
  ok: boolean;
  status: number;
}

class ApiClient {
  private interceptors = new InterceptorManager();
  private useTransport: boolean;

  constructor() {
    // Desktop-only application
    this.useTransport = true;
    
    // Verify Tauri is available
    if (!isDesktopMode()) {
      console.error('[ApiClient] ERROR: This is a Tauri desktop application.');
      console.error('[ApiClient] Please run: npm run tauri:dev');
      console.error('[ApiClient] Web mode is not supported (no backend available).');
    } else {
      console.log('[ApiClient] ✓ Tauri desktop mode initialized');
    }
  }


  async request<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    config: ApiRequestConfig<TBody> = {},
    isRetry = false
  ): Promise<TResponse> {

    const {
      method = 'GET',
      body,
      headers = {},
      showSuccessToast = false,
      successMessage,
      showErrorToast = true,
      errorMessage,
      errorDescription,
      timeout = 30000,
    } = config;

    const toastConfig: ToastConfig = {
      showSuccessToast,
      successMessage,
      showErrorToast,
      errorMessage,
      errorDescription,
    };

    // Desktop-only application - always use Tauri transport
    return this.requestViaTransport<TResponse, TBody>(
      endpoint,
      { method, body, headers, timeout },
      toastConfig
    );
  }

  /**
   * Request via transport layer (Tauri desktop mode)
   */
  private async requestViaTransport<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    config: TransportConfig,
    toastConfig: ToastConfig
  ): Promise<TResponse> {
    try {
      const data = await transport.request<TResponse>(endpoint, config);

      if (toastConfig.showSuccessToast && toastConfig.successMessage) {
        toast.success(toastConfig.successMessage);
      }

      return data;
    } catch (error) {
      const apiError = error instanceof Error && 'code' in error
        ? error as ApiError
        : handleNetworkError(error as Error);

      if (toastConfig.showErrorToast) {
        if (toastConfig.errorMessage) {
          toast.error(toastConfig.errorMessage, {
            description: toastConfig.errorDescription
          });
        } else {
          displayErrorToast(apiError);
        }
      }

      throw apiError;
    }
  }


  getInterceptors(): InterceptorManager {
    return this.interceptors;
  }

  async get<TResponse = unknown>(
    endpoint: string,
    config: Omit<ApiRequestConfig, 'method' | 'body'> = {}
  ): Promise<TResponse> {
    const response = await this.request<TResponse>(endpoint, { ...config, method: 'GET' });
    // Eğer response { data: ... } formatındaysa, data'yı döndür
    if (response && typeof response === 'object' && 'data' in response && !Array.isArray(response)) {
      return (response as any).data as TResponse;
    }
    return response;
  }

  async post<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    body?: TBody,
    config: Omit<ApiRequestConfig<TBody>, 'method' | 'body'> = {}
  ): Promise<TResponse> {
    return this.request<TResponse, TBody>(endpoint, { ...config, method: 'POST', body });
  }

  async put<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    body?: TBody,
    config: Omit<ApiRequestConfig<TBody>, 'method' | 'body'> = {}
  ): Promise<TResponse> {
    return this.request<TResponse, TBody>(endpoint, { ...config, method: 'PUT', body });
  }

  async delete<TResponse = unknown>(
    endpoint: string,
    config: Omit<ApiRequestConfig, 'method' | 'body'> = {}
  ): Promise<TResponse> {
    return this.request<TResponse>(endpoint, { ...config, method: 'DELETE' });
  }

  async patch<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    body?: TBody,
    config: Omit<ApiRequestConfig<TBody>, 'method' | 'body'> = {}
  ): Promise<TResponse> {
    return this.request<TResponse, TBody>(endpoint, { ...config, method: 'PATCH', body });
  }
}

export const apiClient = new ApiClient();

/**
 * Create a safe API handler with proper error handling
 */
export function createApiHandler<TResponse>(
  fetcher: () => Promise<TResponse>,
  fallback: TResponse,
  errorMessage?: string
): () => Promise<TResponse> {
  return async () => {
    try {
      return await fetcher();
    } catch (error) {
      const apiError = error instanceof Error && 'code' in error
        ? error as ApiError
        : handleNetworkError(error as Error);
      
      console.error(errorMessage || 'API error:', apiError);
      
      if (errorMessage) {
        toast.error(errorMessage);
      } else {
        displayErrorToast(apiError);
      }
      
      return fallback;
    }
  };
}

/**
 * Type-safe wrapper for safe fetch
 */
export async function safeFetch<T>(
  endpoint: string,
  config?: ApiRequestConfig
): Promise<T> {
  return apiClient.request<T>(endpoint, config);
}
