import { Trend, Counter, Rate } from 'k6/metrics';

// Custom latency trends for specific Forge workflows
export const healthLatency = new Trend('forge_health_duration', true);
export const authLatency = new Trend('forge_auth_duration', true);
export const readLatency = new Trend('forge_read_duration', true);
export const mutationLatency = new Trend('forge_mutation_duration', true);

// Custom counters and rates
export const successfulRequests = new Counter('forge_req_success_total');
export const failedRequests = new Counter('forge_req_failure_total');
export const throttledRequests = new Counter('forge_req_throttled_total');
export const errorRate = new Rate('forge_error_rate');
