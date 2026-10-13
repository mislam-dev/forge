import http from 'k6/http';
import { check } from 'k6';
import { CONFIG, getFullUrl } from '../config.js';
import { authLatency, successfulRequests, failedRequests, throttledRequests } from './metrics.js';

let cachedToken = null;

/**
 * Execute login against /api/v1/auth/login
 * @param {string} [email]
 * @param {string} [password]
 * @returns {{ accessToken: string|null, refreshToken: string|null, success: boolean, status: number }}
 */
export function login(email = CONFIG.DEFAULT_CREDENTIALS.email, password = CONFIG.DEFAULT_CREDENTIALS.password) {
  const url = getFullUrl(CONFIG.ENDPOINTS.authLogin);
  const payload = JSON.stringify({ email, password });
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
    timeout: '15s',
  };

  const start = Date.now();
  const res = http.post(url, payload, params);
  const duration = Date.now() - start;
  authLatency.add(duration);

  if (res.status === 429 || res.status === 503) {
    throttledRequests.add(1);
    return { accessToken: null, refreshToken: null, success: false, status: res.status };
  }

  const isOk = check(res, {
    'login status is 200': (r) => r.status === 200,
  });

  if (!isOk) {
    failedRequests.add(1);
    return { accessToken: null, refreshToken: null, success: false, status: res.status };
  }

  successfulRequests.add(1);

  try {
    const json = res.json();
    const token = json.body ? json.body.access_token : null;
    const refreshToken = json.body ? json.body.refresh_token : null;
    return {
      accessToken: token,
      refreshToken: refreshToken,
      success: !!token,
      status: res.status,
    };
  } catch (_e) {
    failedRequests.add(1);
    return { accessToken: null, refreshToken: null, success: false, status: res.status };
  }
}

/**
 * Obtain a cached or new bearer token
 * @returns {string|null}
 */
export function getCachedOrNewToken() {
  if (cachedToken) {
    return cachedToken;
  }
  const result = login();
  if (result.success && result.accessToken) {
    cachedToken = result.accessToken;
    return cachedToken;
  }
  return null;
}

/**
 * Generate standard request headers with Authorization and tenant Organization-ID
 * @param {string} [token]
 * @param {string} [orgId]
 * @returns {Object}
 */
export function getAuthHeaders(token = null, orgId = CONFIG.ORGANIZATION_ID) {
  const headers = {
    'Content-Type': 'application/json',
  };

  const resolvedToken = token || getCachedOrNewToken();
  if (resolvedToken) {
    headers['Authorization'] = `Bearer ${resolvedToken}`;
  }

  if (orgId) {
    headers['Organization-ID'] = orgId;
  }

  return headers;
}
