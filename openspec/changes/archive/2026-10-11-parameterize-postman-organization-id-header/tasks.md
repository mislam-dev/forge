# Tasks

## 1. Organizations Collection Updates

- [x] 1.1 Update `Organization-ID` header value to `"{{organization_id}}"` in `organizations/{id}/Get organization details.request.yaml`, `Update organization.request.yaml`, and `Delete organization.request.yaml`, and verify via git diff
- [x] 1.2 Update `Organization-ID` header value to `"{{organization_id}}"` in `organizations/{id}/invitations/Invite member to organization.request.yaml` and `List organization invitations.request.yaml`, and verify via git diff
- [x] 1.3 Update `Organization-ID` header value to `"{{organization_id}}"` in `organizations/{id}/members/List organization members.request.yaml`, `members/{user_id}/Remove member from organization.request.yaml`, and `members/{user_id}/Update member organization role.request.yaml`, and verify via git diff

## 2. Teams Collection Updates

- [x] 2.1 Update `Organization-ID` header value to `"{{organization_id}}"` in `teams/Create a team.request.yaml` and `teams/List teams in organization.request.yaml`, and verify via git diff
- [x] 2.2 Update `Organization-ID` header value to `"{{organization_id}}"` in `teams/{id}/Get team details.request.yaml`, `Update team.request.yaml`, and `Delete team.request.yaml`, and verify via git diff
- [x] 2.3 Update `Organization-ID` header value to `"{{organization_id}}"` in `teams/{id}/members/Add user to team.request.yaml`, `List team members.request.yaml`, `members/{user_id}/Remove user from team.request.yaml`, and `members/{user_id}/Update team member role.request.yaml`, and verify via git diff

## 3. Projects Collection Updates

- [x] 3.1 Update `Organization-ID` header value to `"{{organization_id}}"` in `projects/{id}/Get project by ID.request.yaml`, `Update project configuration.request.yaml`, and `Delete project.request.yaml`, and verify via git diff
- [x] 3.2 Update `Organization-ID` header value to `"{{organization_id}}"` in `projects/{id}/deployments/*.request.yaml` files (triggers, rollback, history, details, and build logs), and verify via git diff
- [x] 3.3 Update `Organization-ID` header value to `"{{organization_id}}"` in `projects/{id}/env-vars/*.request.yaml` files (single, bulk, update, delete, list), and verify via git diff
- [x] 3.4 Update `Organization-ID` header value to `"{{organization_id}}"` in `projects/{id}/members/*.request.yaml` and `projects/{id}/teams/*.request.yaml`, and verify via git diff
- [x] 3.5 Update `Organization-ID` header value to `"{{organization_id}}"` in `projects/{id}/repository/*.request.yaml` (connect, disconnect, get, update), and verify via git diff

## 4. Verification and Validation

- [x] 4.1 Search all `*.request.yaml` files under `postman/postman/collections/Forge Platform API` and verify no `Organization-ID` header contains the hardcoded UUID `07c0060e-8e8c-44c1-942c-3004f5a6c5b6`
- [x] 4.2 Verify every `Organization-ID` header in `*.request.yaml` files resolves to `"{{organization_id}}"` with valid YAML formatting
