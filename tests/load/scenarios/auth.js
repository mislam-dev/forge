import http from 'k6/http';
import { check, sleep } from 'k6';
import { CONFIG, getFullUrl } from '../config.js';
import { authLatency, successfulRequests, failedRequests, throttledRequests, errorRate } from '../helpers/metrics.js';

export const options = {
  vus: 10,
  duration: '10s',
  thresholds: {
    'http_req_duration': ['p(95)<300'], // Auth uses bcrypt, higher duration expected
    'http_req_failed': ['rate<0.05'],
  },
};

/**
 * Execute login and token retrieval flow
 */
export function runAuthScenario() {
  const loginUrl = getFullUrl(CONFIG.ENDPOINTS.authLogin);
  const payload = JSON.stringify({
    email: CONFIG.DEFAULT_CREDENTIALS.email,
    password: CONFIG.DEFAULT_CREDENTIALS.password,
  });

  const start = Date.now();
  const res = http.post(loginUrl, payload, {
    headers: { 'Content-Type': 'application/json' },
    timeout: '15s',
  });
  const duration = Date.now() - start;
  authLatency.add(duration);

  if (res.status === 429 || res.status === 503) {
    throttledRequests.add(1);
    errorRate.add(1);
    return;
  }

  const loginCheck = check(res, {
    'login returns 200': (r) => r.status === 200,
    'login has body object': (r) => {
      try {
        const data = r.json();
        return data && data.body && !!data.body.access_token;
      } catch (_e) {
        return false;
      }
    },
  });

  if (!loginCheck) {
    failedRequests.add(1);
    errorRate.add(1);
    return;
  }

  successfulRequests.add(1);
  errorRate.add(0);

  // Validate /api/v1/auth/me with newly obtained token
  const token = res.json().body.access_token;
  const meRes = http.get(getFullUrl(CONFIG.ENDPOINTS.authMe), {
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
      'Organization-ID': CONFIG.ORGANIZATION_ID,
    },
    timeout: '10s',
  });

  const meCheck = check(meRes, {
    'auth/me returns 200': (r) => r.status === 200,
  });

  if (meCheck) {
    successfulRequests.add(1);
  } else {
    failedRequests.add(1);
  }
}

export default function () {
  runAuthScenario();
  sleep(0.5);
}
