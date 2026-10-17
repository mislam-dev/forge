import { sleep } from 'k6';
import { CONFIG } from '../config.js';
import { login } from '../helpers/auth.js';
import { runHealthProbes } from '../scenarios/health.js';
import { runReadQueries } from '../scenarios/reads.js';
import { runFullJourney } from '../scenarios/full_journey.js';

export const options = {
  stages: [
    { duration: '20s', target: 200 },  // Ramp up to 200 VUs
    { duration: '30s', target: 1000 }, // Ramp up to 1,000 VUs
    { duration: '1m', target: 1000 },  // Sustain at 1,000 VUs
    { duration: '20s', target: 0 },    // Ramp down
  ],
  thresholds: {
    'http_req_duration': ['p(95)<50', 'p(99)<100'],
    'http_req_failed': ['rate<0.001'], // Error rate < 0.1%
    'forge_health_duration': ['p(95)<30'],
    'forge_read_duration': ['p(95)<60'],
  },
};

export function setup() {
  const auth = login();
  return {
    token: auth.accessToken,
    startTime: new Date().toISOString(),
  };
}

export default function (data) {
  const token = data && data.token ? data.token : null;
  const rand = Math.random();

  if (rand < 0.40) {
    runHealthProbes();
  } else if (rand < 0.90) {
    runReadQueries(token);
  } else {
    runFullJourney(token);
  }

  sleep(0.1);
}

export function handleSummary(data) {
  const metrics = data.metrics || {};
  const p95Val = metrics.http_req_duration && metrics.http_req_duration.values ? metrics.http_req_duration.values['p(95)'] : undefined;
  const p99Val = metrics.http_req_duration && metrics.http_req_duration.values ? metrics.http_req_duration.values['p(99)'] : undefined;
  const p95 = p95Val !== undefined ? p95Val.toFixed(2) : 'N/A';
  const p99 = p99Val !== undefined ? p99Val.toFixed(2) : 'N/A';
  const failRate = metrics.http_req_failed && metrics.http_req_failed.values ? (metrics.http_req_failed.values.rate * 100).toFixed(3) : 'N/A';
  const rps = metrics.http_reqs && metrics.http_reqs.values ? metrics.http_reqs.values.rate.toFixed(1) : 'N/A';

  const summaryText = `
=====================================================
Forge Load Test Summary: Tier 1 (1,000 Concurrent VUs)
=====================================================
Throughput:          ${rps} req/s
p95 Latency:         ${p95} ms (Threshold: < 50 ms)
p99 Latency:         ${p99} ms (Threshold: < 100 ms)
HTTP Failure Rate:   ${failRate} % (Threshold: < 0.1 %)
Total Requests:      ${metrics.http_reqs ? metrics.http_reqs.values.count : 'N/A'}
=====================================================
`;

  return {
    stdout: summaryText,
    'tests/load/summary-tier-1k.json': JSON.stringify(data, null, 2),
  };
}
