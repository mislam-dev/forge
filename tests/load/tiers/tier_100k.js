import { sleep } from 'k6';
import { CONFIG } from '../config.js';
import { login } from '../helpers/auth.js';
import { runHealthProbes } from '../scenarios/health.js';
import { runReadQueries } from '../scenarios/reads.js';
import { runFullJourney } from '../scenarios/full_journey.js';

export const options = {
  stages: [
    { duration: '30s', target: 10000 },  // Ramp to 10k baseline
    { duration: '1m', target: 50000 },   // Push into high stress
    { duration: '2m', target: 100000 },  // Reach extreme 100,000 VU saturation limit
    { duration: '1m', target: 100000 },  // Sustain peak stress
    { duration: '1m', target: 1000 },    // Ramp down to verify latency recovery
    { duration: '30s', target: 0 },      // Ramp down to complete
  ],
  thresholds: {
    'http_req_duration': ['p(95)<1000', 'p(99)<3000'], // Expect higher latencies under peak stress
    'http_req_failed': ['rate<0.20'],                  // Allow up to 20% degradation/backpressure
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

  // Traffic weighting for stress testing (health & cached reads dominate to test I/O and multiplexing):
  if (rand < 0.40) {
    runHealthProbes();
  } else if (rand < 0.90) {
    runReadQueries(token);
  } else {
    runFullJourney(token);
  }

  sleep(0.05);
}

export function handleSummary(data) {
  const metrics = data.metrics;
  const p95 = metrics.http_req_duration ? metrics.http_req_duration.values['p(95)'].toFixed(2) : 'N/A';
  const p99 = metrics.http_req_duration ? metrics.http_req_duration.values['p(99)'].toFixed(2) : 'N/A';
  const failRate = metrics.http_req_failed ? (metrics.http_req_failed.values.rate * 100).toFixed(3) : 'N/A';
  const rps = metrics.http_reqs ? metrics.http_reqs.values.rate.toFixed(1) : 'N/A';
  const throttled = metrics.forge_req_throttled_total ? metrics.forge_req_throttled_total.values.count : 0;

  const summaryText = `
=====================================================
Forge Load Test Summary: Tier 3 (100,000 VUs Extreme Stress)
=====================================================
Throughput:          ${rps} req/s
p95 Latency:         ${p95} ms (Threshold: < 1000 ms)
p99 Latency:         ${p99} ms (Threshold: < 3000 ms)
HTTP Failure Rate:   ${failRate} % (Threshold: < 20.0 %)
Throttled (429/503): ${throttled}
Total Requests:      ${metrics.http_reqs ? metrics.http_reqs.values.count : 'N/A'}
=====================================================
`;

  return {
    stdout: summaryText,
    'tests/load/summary-tier-100k.json': JSON.stringify(data, null, 2),
  };
}
