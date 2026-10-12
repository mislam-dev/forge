# Tasks

## 1. Team Core Endpoints

- [x] 1.1 Add test assertion script to `Create a team.request.yaml` validating 201 Created (`TeamResponse`), 400/401/403 (`ErrorResponse`), and environment variable persistence (`team_id`), and verify YAML syntax
- [x] 1.2 Add test assertion script to `List teams in organization.request.yaml` validating 200 OK (`data` array of `TeamResponse`) and 400/401/403 (`ErrorResponse`), and verify YAML syntax
- [x] 1.3 Add test assertion script to `Get team details.request.yaml` under `{id}/` validating 200 OK (`TeamResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.4 Add test assertion script to `Update team.request.yaml` under `{id}/` validating 200 OK (`TeamResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.5 Add test assertion script to `Delete team.request.yaml` under `{id}/` validating 200 OK deletion confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 2. Team Members Endpoints

- [x] 2.1 Add test assertion script to `List team members.request.yaml` under `{id}/members/` validating 200 OK (`data` array of `TeamMemberResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.2 Add test assertion script to `Add user to team.request.yaml` under `{id}/members/` validating 201 Created (`TeamMemberResponse`) and 400/401/403/404/409 (`ErrorResponse`), and verify YAML syntax
- [x] 2.3 Add test assertion script to `Update team member role.request.yaml` under `{id}/members/{user_id}/` validating 200 OK (`TeamMemberResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.4 Add test assertion script to `Remove user from team.request.yaml` under `{id}/members/{user_id}/` validating 200 OK removal confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 3. Collection Validation and Integrity Verification

- [x] 3.1 Validate YAML formatting and schema compliance across all 9 updated team request files against `openapi.yaml`
- [x] 3.2 Verify Postman workspace and collection integrity using collection validation or test runners
