# Design

## Context

The Forge API collections imported from OpenAPI use parameterized paths and URLs referencing `{{baseUrl}}`, `{{bearerToken}}`, and `{{apiKey}}`. Running collections through Newman or importing them into Postman requires an environment file defining these variables. See `proposal.md` for background.

## Goals / Non-Goals

**Goals:**
- Provide a standard Postman Environment JSON file (`environment.json`) at the project root for CLI runners (Newman, Postman CLI).
- Provide matching Postman v12 Local Workspace environment YAML (`postman/postman/environments/Local Development.environment.yaml`) and register it in `postman/.postman/resources.yaml`.
- Define required environment variables and local development defaults that correspond to `.env` (port 9000, `/api/v1` prefix, default seed test users).

**Non-Goals:**
- Committing real production secrets or long-lived authentication tokens.
- Modifying backend Rust routes or core application code.

## Decisions

### Decision: Standard Postman Environment format for root `environment.json`
- **Choice**: Structure `environment.json` with standard Postman Environment schema (`id`, `name`, `values`, `_postman_variable_scope: "environment"`).
- **Rationale**: This format is directly accepted by Newman (`newman run ... -e environment.json`), Postman CLI, CI/CD runners, and standard Postman Import.
- **Alternatives considered**: Simple key-value JSON (`{ "baseUrl": "..." }`) was rejected because Postman and Newman require the formal Postman Environment schema.

### Decision: Dual registration in Postman v12 Workspace
- **Choice**: Alongside root `environment.json`, add `Local Development.environment.yaml` in `postman/postman/environments/` and map it in `postman/.postman/resources.yaml`.
- **Rationale**: Postman v12 uses YAML-based workspace files for version control. Maintaining both ensures both IDE-based Postman v12 workflows and root CLI automation work out-of-the-box.
- **Alternatives considered**: Only generating root `environment.json` would require manual import inside Postman v12.

### Decision: Variable defaults
- `baseUrl`: `http://localhost:9000/api/v1`
- `bearerToken`: `""` (to be captured after `/auth/login` or supplied by test runner)
- `refreshToken`: `""`
- `apiKey`: `forge_dev_api_key_placeholder`
- `adminEmail`: `admin1@forge.local`
- `adminPassword`: `Password123!`
- `testOrgId`: `00000000-0000-0000-0000-000000000001`
- `testProjectId`: `00000000-0000-0000-0000-000000000001`

## Risks / Trade-offs

- **[Risk: Accidental credential leakage]** → Mitigation: All values in `environment.json` are strictly local development defaults or empty placeholder strings. No production secrets are stored.
- **[Risk: Drift between root JSON and workspace YAML]** → Mitigation: Document both formats in `tasks.md` and define an automated verification step to ensure parity.
