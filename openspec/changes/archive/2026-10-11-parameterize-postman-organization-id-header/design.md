# Design

## Context

The Forge Platform API Postman workspace stores HTTP requests as YAML documents (`$kind: http-request`). Numerous request definitions under `postman/postman/collections/Forge Platform API/` currently contain a hardcoded UUID (`07c0060e-8e8c-44c1-942c-3004f5a6c5b6`) in their `Organization-ID` header entry. The environment file `postman/postman/environments/Local Development.environment.yaml` contains the `organization_id` variable to serve as the single shared configuration value. Two requests (`Create a project.request.yaml` and `List projects for organization.request.yaml`) already demonstrate the intended pattern.

## Goals / Non-Goals

**Goals:**
- Replace hardcoded UUID values for `Organization-ID` in all `*.request.yaml` files across `postman/postman/collections/Forge Platform API/` with `"{{organization_id}}"`.
- Match the schema and formatting established in `projects/List projects for organization.request.yaml` while preserving existing comments, descriptions, and type metadata.
- Ensure all collection endpoints operate consistently against the active environment's organization context.

**Non-Goals:**
- Modifying static response example fixtures (`.resources/**/*.example.yaml`), which represent recorded API contract examples rather than active executable request definitions.
- Modifying other path or body parameters unless they are specifically the `Organization-ID` header.
- Modifying backend server logic or OpenAPI specification source files.

## Decisions

- **Target Executable Request Files Only**:
  *Decision*: Scope changes to `*.request.yaml` files across `postman/postman/collections/Forge Platform API/`.
  *Rationale*: Active collection runs in Postman and Newman execute the `*.request.yaml` specifications. Static examples under `.resources/` serve as documentation snapshots.
  *Alternative Considered*: Updating `.example.yaml` headers as well; rejected to keep example payload contracts intact unless necessary.

- **Preserve Header Block Structure and Metadata**:
  *Decision*: Keep `disabled: false`, `description: ...`, and the `$type` JSON schema definitions intact, only changing `value: 07c0060e-8e8c-44c1-942c-3004f5a6c5b6` to `value: "{{organization_id}}"`.
  *Rationale*: Retaining schema metadata allows Postman to continue performing client-side schema validation when variable substitution occurs.

## Risks / Trade-offs

- **[Unset Environment Variable]** If `organization_id` is left empty in the active Postman environment, requests requiring tenant authorization will fail with a 400 Bad Request (`Invalid Organization-ID header: must be a valid UUID`).
  *Mitigation*: The environment variable is already present in `Local Development.environment.yaml`. Provide clear documentation that tests and developers should assign a valid organization UUID to this variable before executing requests.
