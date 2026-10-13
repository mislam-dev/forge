import http from 'k6/http';
import { check, sleep } from 'k6';
import { CONFIG, getFullUrl } from '../config.js';
import { login, getAuthHeaders } from '../helpers/auth.js';
import { readLatency, successfulRequests, failedRequests, throttledRequests, errorRate } from '../helpers/metrics.js';

export const options = {
  vus: 10,
  duration: '10s',
  thresholds: CONFIG.THRESHOLDS.tier1k,
};

/**
 * Setup hook to pre-authenticate and return token for VUs
 */
export function setup() {
  const auth = login();
  return {
    token: auth.accessToken,
  };
}

/**
 * Execute authenticated read queries
 * @param {string} [token]
 */
export function runReadQueries(token = null) {
  const headers = getAuthHeaders(token);
  const endpoints = [
    '/api/v1/organizations',
    '/api/v1/projects',
    '/api/v1/teams',
    '/api/v1/users',
    '/api/v1/dashboard',
  ];

  for (const ep of endpoints) {
    const url = getFullUrl(ep);
    const start = Date.now();
    const res = http.get(url, {
      headers: headers,
      timeout: '10s',
    });
    const duration = Date.now() - start;
    readLatency.add(duration);

    if (res.status === 429 || res.status === 503) {
      throttledRequests.add(1);
      errorRate.add(1);
      continue;
    }

    const passed = check(res, {
      'read endpoint status is 200': (r) => r.status === 200,
      'read response time < 200ms': (r) => r.timings.duration < 200,
    });

    if (passed) {
      successfulRequests.add(1);
      errorRate.add(0);
    } else {
      failedRequests.add(1);
      errorRate.add(1);
    }
  }
}

export default function (data) {
  const token = data && data.token ? data.token : null;
  runReadQueries(token);
  sleep(0.1);
}
