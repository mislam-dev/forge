# Tasks

## 1. Organization Core Endpoints

- [x] 1.1 Add test assertion script to `Create a new organization.request.yaml` validating 201 Created (`OrganizationResponse`), 400/401 (`ErrorResponse`), and environment variable persistence (`organization_id`), and verify YAML syntax
- [x] 1.2 Add test assertion script to `List user's organizations.request.yaml` validating 200 OK (`data` array of `OrganizationResponse`) and 401 Unauthorized (`ErrorResponse`), and verify YAML syntax
- [x] 1.3 Add test assertion script to `Get organization details.request.yaml` validating 200 OK (`OrganizationResponse`) and 401/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.4 Add test assertion script to `Update organization.request.yaml` validating 200 OK (`OrganizationResponse`) and 400/401/404 (`ErrorResponse`), and verify YAML syntax
- [x] 1.5 Add test assertion script to `Delete organization.request.yaml` validating 200 OK deletion message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 2. Organization Invitations Endpoints

- [x] 2.1 Add test assertion script to `Invite member to organization.request.yaml` validating 201 Created (`OrgInvitationResponse`) and 400/401/403/409 (`ErrorResponse`), and verify YAML syntax
- [x] 2.2 Add test assertion script to `List organization invitations.request.yaml` validating 200 OK (`data` array of `OrgInvitationResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 2.3 Add test assertion script to `Accept organization invitation.request.yaml` validating 200 OK (`OrgMemberResponse`) and 400/401/404 (`ErrorResponse`), and verify YAML syntax

## 3. Organization Members Endpoints

- [x] 3.1 Add test assertion script to `List organization members.request.yaml` validating 200 OK (`data` array of `OrgMemberResponse`) and 401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.2 Add test assertion script to `Update member organization role.request.yaml` validating 200 OK (`OrgMemberResponse`) and 400/401/403/404 (`ErrorResponse`), and verify YAML syntax
- [x] 3.3 Add test assertion script to `Remove member from organization.request.yaml` validating 200 OK removal message and 401/403/404 (`ErrorResponse`), and verify YAML syntax

## 4. Collection Validation and Integrity Verification

- [x] 4.1 Validate YAML formatting and schema compliance across all 11 updated organization request files against `openapi.yaml`
- [x] 4.2 Verify Postman workspace and collection integrity using collection validation or test runners
