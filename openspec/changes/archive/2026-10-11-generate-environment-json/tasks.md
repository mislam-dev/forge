# Tasks

## 1. Create Root Environment File

- [x] 1.1 Generate `environment.json` in the project root following the Postman Environment format schema with variables `baseUrl` (`http://localhost:9000/api/v1`), `bearerToken`, `refreshToken`, `apiKey`, and default credentials (`adminEmail`, `adminPassword`), and verify JSON schema validity
- [x] 1.2 Verify that variable keys match the token patterns used across `postman/postman/collections/Forge Platform API`

## 2. Postman Workspace Environment Alignment

- [x] 2.1 Create `postman/postman/environments/Local Development.environment.yaml` with corresponding key-value pairs for the local Postman workspace
- [x] 2.2 Register the environment file in `postman/.postman/resources.yaml` under `localResources.environments` and `resourceNameMappings.environments`
- [x] 2.3 Verify Postman workspace file structure and confirm no unresolved environment reference warnings exist
