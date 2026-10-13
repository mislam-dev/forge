import http from 'k6/http';
import { check, sleep } from 'k6';
import { CONFIG, getFullUrl } from '../config.js';
import { healthLatency, successfulRequests, failedRequests, throttledRequests, errorRate } from '../helpers/metrics.js';

export const options = {
  vus: 10,
  duration: '10s',
  thresholds: CONFIG.THRESHOLDS.tier1k,
};

/**
 * Execute health probes across public health endpoints
 */
export function runHealthProbes() {
  const endpoints = [
    CONFIG.ENDPOINTS.health,
    CONFIG.ENDPOINTS.healthLive,
    CONFIG.ENDPOINTS.healthReady,
  ];

  for (const ep of endpoints) {
    const url = getFullUrl(ep);
    const start = Date.now();
    const res = http.get(url, {
      headers: { 'Accept': 'application/json' },
      timeout: '10s',
    });
    const duration = Date.now() - start;

    healthLatency.add(duration);

    if (res.status === 429 || res.status === 503) {
      throttledRequests.add(1);
      errorRate.add(1);
      continue;
    }

    const passed = check(res, {
      'health probe status is 200': (r) => r.status === 200,
      'health response time < 500ms': (r) => r.timings.duration < 500,
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

export default function () {
  runHealthProbes();
  sleep(0.1);
}
