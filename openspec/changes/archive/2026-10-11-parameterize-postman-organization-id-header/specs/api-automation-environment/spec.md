# Spec Delta

## ADDED Requirements

### Requirement: Organization ID Header Parameterization across Collections
All collection requests within the Forge Platform API that require or accept tenant scoping via the `Organization-ID` header SHALL reference the environment variable `{{organization_id}}` as their header value rather than hardcoded or static UUID values.

#### Scenario: Inspecting tenant-scoped request headers
- **WHEN** any collection request under `postman/postman/collections/Forge Platform API/` that includes the `Organization-ID` header is inspected
- **THEN** the `Organization-ID` header value is configured as `"{{organization_id}}"`
- **AND** the header retains `disabled: false`, description, and UUID schema metadata.

#### Scenario: Dynamic variable resolution during execution
- **WHEN** requests are executed within Postman or Newman against a configured environment containing `organization_id`
- **THEN** the `Organization-ID` header dynamically resolves to the active environment's `organization_id` value.
