import { sleep } from 'k6';
import { CONFIG } from '../config.js';
import { login } from '../helpers/auth.js';
import { runHealthProbes } from '../scenarios/health.js';
import { runReadQueries } from '../scenarios/reads.js';
import { runFullJourney } from '../scenarios/full_journey.js';

export const options = {
  stages: [
    { duration: '30s', target: 2000 },  // Ramp to 2,000 VUs
    { duration: '1m', target: 10000 },  // Ramp to 10,000 VUs (SRS NFR-001 Target)
    { duration: '2m', target: 10000 },  // Sustain 10,000 VUs
    { duration: '30s', target: 0 },     // Controlled ramp down
  ],
  thresholds: {
    'http_req_duration': ['p(95)<200', 'p(99)<500'],
    'http_req_failed': ['rate<0.01'], // Error rate < 1.0%
    'forge_health_duration': ['p(95)<50'],
    'forge_read_duration': ['p(95)<150'],
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

  // Traffic weighting reflecting typical production access:
  if (rand < 0.35) {
    runHealthProbes();
  } else if (rand < 0.85) {
    runReadQueries(token);
  } else {
    runFullJourney(token);
  }

  sleep(0.1);
}

export function handleSummary(data) {
  const metrics = data.metrics;
  const p95 = metrics.http_req_duration ? metrics.http_req_duration.values['p(95)'].toFixed(2) : 'N/A';
  const p99 = metrics.http_req_duration ? metrics.http_req_duration.values['p(99)'].toFixed(2) : 'N/A';
  const failRate = metrics.http_req_failed ? (metrics.http_req_failed.values.rate * 100).toFixed(3) : 'N/A';
  const rps = metrics.http_reqs ? metrics.http_reqs.values.rate.toFixed(1) : 'N/A';

  const summaryText = `
=====================================================
Forge Load Test Summary: Tier 2 (10,000 Concurrent VUs - NFR-001)
=====================================================
Throughput:          ${rps} req/s
p95 Latency:         ${p95} ms (Threshold: < 200 ms)
p99 Latency:         ${p99} ms (Threshold: < 500 ms)
HTTP Failure Rate:   ${failRate} % (Threshold: < 1.0 %)
Total Requests:      ${metrics.http_reqs ? metrics.http_reqs.values.count : 'N/A'}
=====================================================
`;

  return {
    stdout: summaryText,
    'tests/load/summary-tier-10k.json': JSON.stringify(data, null, 2),
  };
}
