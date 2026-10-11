# Tasks

## 1. Create Multi-Role & Tenancy Matrix Environments

- [x] 1.1 Create `postman/postman/environments/Admin - With Organization.environment.yaml` with admin credentials (`admin1@forge.local`), test password, and active `organization_id` (`00000000-0000-0000-0000-000000000001`), and verify file existence and YAML validity
- [x] 1.2 Create `postman/postman/environments/Admin - Without Organization.environment.yaml` with admin credentials (`admin1@forge.local`), test password, and empty `organization_id` (`""`), and verify file existence and YAML validity
- [x] 1.3 Create `postman/postman/environments/Developer - With Organization.environment.yaml` with developer credentials (`dev1@forge.local`), test password, and active `organization_id` (`00000000-0000-0000-0000-000000000001`), and verify file existence and YAML validity
- [x] 1.4 Create `postman/postman/environments/Developer - Without Organization.environment.yaml` with developer credentials (`dev1@forge.local`), test password, and empty `organization_id` (`""`), and verify file existence and YAML validity
- [x] 1.5 Create `postman/postman/environments/Viewer - With Organization.environment.yaml` with viewer credentials (`viewer1@forge.local`), test password, and active `organization_id` (`00000000-0000-0000-0000-000000000001`), and verify file existence and YAML validity
- [x] 1.6 Create `postman/postman/environments/Viewer - Without Organization.environment.yaml` with viewer credentials (`viewer1@forge.local`), test password, and empty `organization_id` (`""`), and verify file existence and YAML validity

## 2. Create Development & Production Tier Environments

- [x] 2.1 Create `postman/postman/environments/Development.environment.yaml` targeting `http://localhost:9000/api/v1` with standard test credentials and tokens, and verify file existence and YAML validity
- [x] 2.2 Create `postman/postman/environments/Production.environment.yaml` with production base URL placeholder (`https://api.forge.local/api/v1`) and empty secret placeholders for sensitive credentials, and verify file existence and YAML validity

## 3. Register Environments in Postman Workspace Configuration

- [x] 3.1 Update `postman/.postman/resources.yaml` to declare all 8 environments under `localResources.environments` and verify YAML syntax
- [x] 3.2 Update `postman/.postman/resources.yaml` to define display name mappings under `resourceNameMappings.environments` and verify mapping consistency

## 4. Generate Postman CLI and Newman Compatible JSON Environment Files

- [x] 4.1 Generate standard Postman JSON format environment files for all 8 environments under `postman/environments/` and verify JSON schema validity
- [x] 4.2 Generate or update root `environment.json` mirroring the Development environment configuration and verify schema compliance
