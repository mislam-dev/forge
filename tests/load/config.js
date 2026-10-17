// Forge Platform Load Testing - Global Configuration

export const CONFIG = {
  BASE_URL: __ENV.BASE_URL || 'http://localhost:9000',
  API_PREFIX: '/api/v1',
  DEFAULT_CREDENTIALS: {
    email: __ENV.TEST_EMAIL || 'admin1@forge.local',
    password: __ENV.TEST_PASSWORD || 'Password123!',
  },
  ORGANIZATION_ID: __ENV.ORG_ID || '1b913f11-5abc-4dc8-b0a6-3aada3466472',
  ENDPOINTS: {
    root: '/',
    health: '/api/v1/health',
    healthLive: '/api/v1/health/live',
    healthReady: '/api/v1/health/ready',
    healthDetails: '/api/v1/health/details',
    authLogin: '/api/v1/auth/login',
    authMe: '/api/v1/auth/me',
    userProfile: '/api/v1/users/profile',
    organizations: '/api/v1/organizations',
    projects: '/api/v1/projects',
    teams: '/api/v1/teams',
  },
  THRESHOLDS: {
    tier1k: {
      http_req_duration: ['p(95)<50', 'p(99)<100'],
      http_req_failed: ['rate<0.001'], // error rate < 0.1%
    },
    tier10k: {
      http_req_duration: ['p(95)<200', 'p(99)<500'],
      http_req_failed: ['rate<0.01'],  // error rate < 1.0% (SRS NFR-001)
    },
    tier100k: {
      http_req_duration: ['p(95)<1000', 'p(99)<3000'],
      http_req_failed: ['rate<0.15'], // allows throttled 429/503 responses under saturation
    },
  },
};

export function getFullUrl(endpoint) {
  return `${CONFIG.BASE_URL}${endpoint}`;
}
