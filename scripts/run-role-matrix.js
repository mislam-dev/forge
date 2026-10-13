#!/usr/bin/env node

/**
 * Multi-Role & Tenancy Matrix Automation Runner
 * 
 * Runs the Forge Platform API Postman collection sequentially across all 6
 * tenant and role environments to verify RBAC security isolation and tenancy contracts.
 */

const fs = require('fs');
const path = require('path');
const newman = require('newman');

const COLLECTION_FILE = path.resolve(__dirname, '../postman/forge-platform-api.collection.json');
const ENV_DIR = path.resolve(__dirname, '../postman/environments');

const MATRIX_ENVIRONMENTS = [
  { id: 'admin-with-org', name: 'Admin - With Organization', file: 'admin-with-org.environment.json', role: 'admin', hasOrg: true },
  { id: 'admin-without-org', name: 'Admin - Without Organization', file: 'admin-without-org.environment.json', role: 'admin', hasOrg: false },
  { id: 'developer-with-org', name: 'Developer - With Organization', file: 'developer-with-org.environment.json', role: 'developer', hasOrg: true },
  { id: 'developer-without-org', name: 'Developer - Without Organization', file: 'developer-without-org.environment.json', role: 'developer', hasOrg: false },
  { id: 'viewer-with-org', name: 'Viewer - With Organization', file: 'viewer-with-org.environment.json', role: 'viewer', hasOrg: true },
  { id: 'viewer-without-org', name: 'Viewer - Without Organization', file: 'viewer-without-org.environment.json', role: 'viewer', hasOrg: false },
];

// CLI arguments
const args = process.argv.slice(2);
const folderArgIndex = args.indexOf('--folder');
const targetFolder = folderArgIndex !== -1 ? args[folderArgIndex + 1] : undefined;
const isDryRun = args.includes('--dry-run');

function ensureCollectionExists() {
  if (!fs.existsSync(COLLECTION_FILE)) {
    console.log('Collection file missing, bundling collection first...');
    require('./bundle-collection.js');
  }
}

function runEnvironment(envConfig) {
  const envPath = path.join(ENV_DIR, envConfig.file);
  if (!fs.existsSync(envPath)) {
    return Promise.reject(new Error(`Environment file not found: ${envPath}`));
  }

  const runOptions = {
    collection: COLLECTION_FILE,
    environment: envPath,
    reporters: ['cli'],
    reporter: {
      cli: {
        noSummary: false,
        silent: false
      }
    }
  };

  if (targetFolder) {
    runOptions.folder = targetFolder;
  }

  return new Promise((resolve) => {
    console.log(`\n============================================================`);
    console.log(`▶ Running Matrix Iteration: ${envConfig.name} (${envConfig.id})`);
    console.log(`============================================================\n`);

    newman.run(runOptions, (err, summary) => {
      const stats = summary?.run?.stats || {};
      const assertionsFailed = stats.assertions?.failed || 0;
      const requestsFailed = stats.requests?.failed || 0;
      const totalRequests = stats.requests?.total || 0;
      const totalAssertions = stats.assertions?.total || 0;

      const passed = !err && assertionsFailed === 0 && requestsFailed === 0;

      resolve({
        id: envConfig.id,
        name: envConfig.name,
        role: envConfig.role,
        hasOrg: envConfig.hasOrg,
        passed,
        requests: totalRequests,
        assertions: totalAssertions,
        failedRequests: requestsFailed,
        failedAssertions: assertionsFailed,
        error: err ? err.message : null
      });
    });
  });
}

function printSummaryTable(results) {
  console.log(`\n========================================================================================`);
  console.log(`                       MULTI-ROLE RBAC MATRIX TEST SUMMARY                              `);
  console.log(`========================================================================================`);
  console.log(
    '| ' + 'Environment'.padEnd(32) +
    ' | ' + 'Role'.padEnd(10) +
    ' | ' + 'Tenancy'.padEnd(12) +
    ' | ' + 'Status'.padEnd(8) +
    ' | ' + 'Requests'.padEnd(10) +
    ' | ' + 'Failures'.padEnd(10) + ' |'
  );
  console.log(`|${'-'.repeat(34)}|${'-'.repeat(12)}|${'-'.repeat(14)}|${'-'.repeat(10)}|${'-'.repeat(12)}|${'-'.repeat(12)}|`);

  let allPassed = true;

  for (const r of results) {
    const status = r.passed ? '✓ PASS' : '✗ FAIL';
    if (!r.passed) allPassed = false;

    const tenancyStr = r.hasOrg ? 'With Org' : 'No Org';
    const failCountStr = String(r.failedAssertions + r.failedRequests);

    console.log(
      '| ' + r.name.padEnd(32) +
      ' | ' + r.role.padEnd(10) +
      ' | ' + tenancyStr.padEnd(12) +
      ' | ' + status.padEnd(8) +
      ' | ' + String(r.requests).padEnd(10) +
      ' | ' + failCountStr.padEnd(10) + ' |'
    );
  }

  console.log(`========================================================================================\n`);
  return allPassed;
}

async function main() {
  ensureCollectionExists();

  if (isDryRun) {
    console.log('Dry-run mode: Verifying environment configuration files...');
    for (const env of MATRIX_ENVIRONMENTS) {
      const envPath = path.join(ENV_DIR, env.file);
      if (!fs.existsSync(envPath)) {
        console.error(`Missing environment file: ${envPath}`);
        process.exit(1);
      }
      console.log(`✓ Verified ${env.name} -> ${env.file}`);
    }
    console.log('\nAll 6 role and tenancy environments verified successfully!');
    process.exit(0);
  }

  const results = [];
  for (const env of MATRIX_ENVIRONMENTS) {
    const res = await runEnvironment(env);
    results.push(res);
  }

  const allPassed = printSummaryTable(results);
  if (!allPassed) {
    console.error('RBAC Matrix test failed: One or more environments failed assertion policies.');
    process.exit(1);
  } else {
    console.log('All 6 role and tenancy environments passed RBAC security assertions successfully!');
    process.exit(0);
  }
}

main().catch(err => {
  console.error('Fatal error running matrix runner:', err);
  process.exit(1);
});
