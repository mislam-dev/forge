# Tasks

## 1. Core Project Endpoints

- [x] 1.1 Add test assertion script to `Create a project.request.yaml` validating 201 Created (`ProjectResponse`), 400/401/403/409 (`ErrorResponse`), and environment variable persistence (`project_id`), and verify YAML syntax
- [x] 1.2 Add test assertion script to `List projects for organization.request.yaml` validating 200 OK (`data` array of `ProjectResponse`) and 400/401/403 (`ErrorResponse`), and verify YAML syntax
- [x] 1.3 Add test assertion script to `Get project by ID.request.yaml` under `{id}/` validating 200 OK (`ProjectResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.4 Add test assertion script to `Update project configuration.request.yaml` under `{id}/` validating 200 OK (`ProjectResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.5 Add test assertion script to `Delete project.request.yaml` under `{id}/` validating 200 OK deletion confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 2. Git Repository Endpoints

- [x] 2.1 Add test assertion script to `Connect or reconnect Git repository to project.request.yaml` under `{id}/repository/` validating 201 Created (`ProjectRepositoryResponse`) and 400/401/403/404/409 (`ErrorResponse`), and verify YAML syntax
- [x] 2.2 Add test assertion script to `Get connected repository configuration.request.yaml` under `{id}/repository/` validating 200 OK (`ProjectRepositoryResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.3 Add test assertion script to `Update connected repository configuration.request.yaml` under `{id}/repository/` validating 200 OK (`ProjectRepositoryResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.4 Add test assertion script to `Disconnect repository from project.request.yaml` under `{id}/repository/` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 3. Project Members and Teams Endpoints

- [x] 3.1 Add test assertion script to `List users assigned to project.request.yaml` under `{id}/members/` validating 200 OK (`data` array of members) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.2 Add test assertion script to `Assign user to project.request.yaml` under `{id}/members/` validating 200/201 success confirmation and 400/401/403/404/409 (`ErrorResponse`), and verify YAML syntax
- [x] 3.3 Add test assertion script to `Remove user from project.request.yaml` under `{id}/members/{user_id}/` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.4 Add test assertion script to `List teams assigned to project.request.yaml` under `{id}/teams/` validating 200 OK (`data` array of teams) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.5 Add test assertion script to `Assign team to project.request.yaml` under `{id}/teams/` validating 200/201 success confirmation and 400/401/403/404/409 (`ErrorResponse`), and verify YAML syntax
- [x] 3.6 Add test assertion script to `Remove team from project.request.yaml` under `{id}/teams/{team_id}/` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 4. Project Environment Variables Endpoints

- [x] 4.1 Add test assertion script to `Create an environment variable.request.yaml` under `{id}/env-vars/` validating 201 Created (`ProjectEnvironmentVariableResponse`), 400/401/403/404/409 (`ErrorResponse`), and environment variable persistence (`env_id`), and verify YAML syntax
- [x] 4.2 Add test assertion script to `List environment variables for project.request.yaml` under `{id}/env-vars/` validating 200 OK (`data` array) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 4.3 Add test assertion script to `Bulk create or update environment variables.request.yaml` under `{id}/env-vars/bulk/` validating 200/201 success (`data` array) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 4.4 Add test assertion script to `Update environment variable.request.yaml` under `{id}/env-vars/{env_id}/` validating 200 OK (`ProjectEnvironmentVariableResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 4.5 Add test assertion script to `Delete environment variable.request.yaml` under `{id}/env-vars/{env_id}/` validating 200 OK confirmation message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 5. Deployments and Logs Endpoints

- [x] 5.1 Add test assertion script to `Trigger a new project deployment.request.yaml` under `{id}/deployments/` validating 201 Created (`DeploymentResponse`), 400/401/403/404/409 (`ErrorResponse`), and environment variable persistence (`deployment_id`), and verify YAML syntax
- [x] 5.2 Add test assertion script to `List deployment history for a project.request.yaml` under `{id}/deployments/` validating 200 OK (`data` array) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.3 Add test assertion script to `Get deployment details by ID.request.yaml` under `{id}/deployments/{deployment_id}/` validating 200 OK (`DeploymentResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.4 Add test assertion script to `Re-trigger deployment at same commit.request.yaml` under `{id}/deployments/{deployment_id}/redeploy/` validating 201/202 success (`DeploymentResponse`), 400/401/403/404 (`ErrorResponse`), and `deployment_id` persistence, and verify YAML syntax
- [x] 5.5 Add test assertion script to `Rollback project to last successful deployment commit.request.yaml` under `{id}/deployments/rollback/` validating 201/202 success (`DeploymentResponse`), 400/401/403/404 (`ErrorResponse`), and `deployment_id` persistence, and verify YAML syntax
- [x] 5.6 Add test assertion script to `Get stored build logs.request.yaml` under `{id}/deployments/{deployment_id}/logs/` validating 200 OK (`data` log entries) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.7 Add test assertion script to `Download complete deployment log file.request.yaml` under `{id}/deployments/{deployment_id}/logs/download/` validating 200 OK (text/plain Content-Type) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.8 Add test assertion script to `Search build logs by keyword.request.yaml` under `{id}/deployments/{deployment_id}/logs/search/` validating 200 OK search results and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.9 Add test assertion script to `Open Server-Sent Events (SSE) log stream.request.yaml` under `{id}/deployments/{deployment_id}/logs/stream/` validating 200 OK (text/event-stream Content-Type) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 5.10 Add test assertion script to `Update deployment status (Internal Build Worker Only).request.yaml` under `internal/deployments/{deployment_id}/status/` validating 200 OK confirmation and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax

## 6. Collection Validation and Integrity Verification

- [x] 6.1 Validate YAML formatting and schema compliance across all 30 updated project request files against `openapi.yaml`
- [x] 6.2 Verify Postman workspace and collection integrity using collection validation tools
