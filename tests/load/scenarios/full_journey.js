import http from 'k6/http';
import { check, sleep } from 'k6';
import { CONFIG, getFullUrl } from '../config.js';
import { login, getAuthHeaders } from '../helpers/auth.js';
import {
  healthLatency,
  readLatency,
  mutationLatency,
  successfulRequests,
  failedRequests,
  throttledRequests,
  errorRate,
} from '../helpers/metrics.js';

export const options = {
  vus: 10,
  duration: '15s',
  thresholds: CONFIG.THRESHOLDS.tier1k,
};

export function setup() {
  const auth = login();
  return {
    token: auth.accessToken,
  };
}

/**
 * Execute full developer user journey simulating real lifecycle traffic
 * @param {string} [token]
 */
export function runFullJourney(token = null) {
  // 1. Health check probe
  const healthStart = Date.now();
  const healthRes = http.get(getFullUrl(CONFIG.ENDPOINTS.health), { timeout: '10s' });
  healthLatency.add(Date.now() - healthStart);

  if (healthRes.status === 429 || healthRes.status === 503) {
    throttledRequests.add(1);
    errorRate.add(1);
    return;
  }
  check(healthRes, { 'journey health 200': (r) => r.status === 200 });

  const headers = getAuthHeaders(token);

  // 2. Fetch authenticated profile /auth/me
  const profileStart = Date.now();
  const profileRes = http.get(getFullUrl(CONFIG.ENDPOINTS.authMe), { headers, timeout: '10s' });
  readLatency.add(Date.now() - profileStart);

  if (profileRes.status === 429 || profileRes.status === 503) {
    throttledRequests.add(1);
    errorRate.add(1);
    return;
  }

  const profileOk = check(profileRes, { 'journey profile 200': (r) => r.status === 200 });
  if (profileOk) {
    successfulRequests.add(1);
    errorRate.add(0);
  } else {
    failedRequests.add(1);
    errorRate.add(1);
  }

  // 3. Fetch organizations and teams
  const orgStart = Date.now();
  const orgRes = http.get(getFullUrl(CONFIG.ENDPOINTS.organizations), { headers, timeout: '10s' });
  readLatency.add(Date.now() - orgStart);
  if (orgRes.status === 200) successfulRequests.add(1); else failedRequests.add(1);

  const teamStart = Date.now();
  const teamRes = http.get(getFullUrl(CONFIG.ENDPOINTS.teams), { headers, timeout: '10s' });
  readLatency.add(Date.now() - teamStart);
  if (teamRes.status === 200) successfulRequests.add(1); else failedRequests.add(1);

  // 4. Fetch projects
  const projStart = Date.now();
  const projRes = http.get(getFullUrl(CONFIG.ENDPOINTS.projects), { headers, timeout: '10s' });
  readLatency.add(Date.now() - projStart);
  if (projRes.status === 200) successfulRequests.add(1); else failedRequests.add(1);

  // 5. 10% probability write operation (Project creation)
  if (Math.random() < 0.10) {
    const uniqueId = `${Date.now()}-${Math.floor(Math.random() * 10000)}`;
    const newProjectPayload = JSON.stringify({
      name: `load-test-proj-${uniqueId}`,
      description: 'Automated load test ephemeral project',
      project_type: 'repo',
      runtime: 'node_js',
      port: 3000,
    });

    const mutateStart = Date.now();
    const mutateRes = http.post(getFullUrl(CONFIG.ENDPOINTS.projects), newProjectPayload, {
      headers,
      timeout: '15s',
    });
    mutationLatency.add(Date.now() - mutateStart);

    if (mutateRes.status === 429 || mutateRes.status === 503) {
      throttledRequests.add(1);
      errorRate.add(1);
    } else {
      const mutateOk = check(mutateRes, {
        'create project 200 or 201': (r) => r.status === 200 || r.status === 201,
      });
      if (mutateOk) {
        successfulRequests.add(1);
        errorRate.add(0);
      } else {
        failedRequests.add(1);
        errorRate.add(1);
      }
    }
  }
}

export default function (data) {
  const token = data && data.token ? data.token : null;
  runFullJourney(token);
  sleep(0.2);
}
