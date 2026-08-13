# Module 18 — Notifications

> **Module Type:** Core Module
> **Priority:** P2 — Post-MVP
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Notifications Module](../../modules/notifications/notifications-module.md)

---

## 1. Module Overview

### Purpose

The Notifications module provides **in-app event notifications** for platform events such as deployment success/failure, team membership changes, and org invitations. Notifications are delivered asynchronously via RabbitMQ and stored in PostgreSQL.

### Responsibilities

- Store notification records per user
- List notifications (paginated, with unread filtering)
- Mark individual notification as read
- Mark all notifications as read
- Get unread count
- SSE stream for real-time notification delivery (optional MVP enhancement)
- Publish notifications to `forge.notifications.jobs` queue

### Scope

**Included:**
- `GET /notifications` — list user's notifications
- `GET /notifications/unread-count` — get unread count
- `PATCH /notifications/:id/read` — mark as read
- `PATCH /notifications/read-all` — mark all as read
- `GET /notifications/stream` — SSE stream (optional — mark as P3)
- Background consumer: consume from `forge.notifications.jobs` and persist to DB

**Excluded:**
- Email notifications (future)
- Push notifications (future)
- Notification preferences/settings (future)

---

## 2. Dependencies

### Depends On
- **Users** (notifications are per-user)
- **RabbitMQ** (async delivery queue)
- **Authentication**

### Used By
- Build Worker (publishes deployment events)
- Org Members (publishes invite events)

---

## 3. Database Table

### `notifications`

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| user_id | UUID | FK -> users.id CASCADE, Not Null |
| type | VARCHAR(100) | Not Null (e.g., deployment_success, deployment_failed, org_invite) |
| title | VARCHAR(255) | Not Null |
| message | TEXT | Not Null |
| reference_id | UUID | Nullable (related deployment, org, project ID) |
| reference_type | VARCHAR | Nullable (deployment, org, project) |
| is_read | BOOLEAN | Default false, Not Null |
| created_at | TIMESTAMP | Not Null |

**Key Index:** `(user_id, is_read, created_at DESC)` — for efficient unread queries

---

## 4. Notification Types

| Type | Trigger | Source |
|------|---------|--------|
| `deployment_success` | Deployment reaches Success state | Build Worker |
| `deployment_failed` | Deployment reaches Failed state | Build Worker |
| `org_member_invited` | User added to organization | Org Members service |
| `org_member_removed` | User removed from organization | Org Members service |
| `project_assigned` | User assigned to project | Project Assignments service |

---

## 5. API Implementation

### GET /notifications

- **Auth:** JWT (own notifications only)
- **Query params:** `page`, `per_page`, `is_read` (optional: true/false)
- **Response:** `200 { message, data: [notifications], meta: pagination }`

### GET /notifications/unread-count

- **Auth:** JWT
- **Response:** `200 { message, data: { count: 5 } }`

### PATCH /notifications/:id/read

- **Auth:** JWT (own notification only)
- **Service logic:** Verify notification belongs to `jwt_user_id`, set `is_read = true`
- **Response:** `200 { message: "Notification marked as read." }`

### PATCH /notifications/read-all

- **Auth:** JWT
- **Service logic:** Update all `is_read = false` notifications for `jwt_user_id` to `is_read = true`
- **Response:** `200 { message: "All notifications marked as read." }`

---

## 6. Background Consumer

The notification worker is a RabbitMQ consumer that runs as a background task:

1. Consume from `forge.notifications.jobs` queue
2. Deserialize notification payload: `{ user_id, type, title, message, reference_id?, reference_type? }`
3. Insert `notifications` record to DB
4. Ack message on success
5. Nack (requeue=false) on unrecoverable error

---

## 7. Testing

### Integration Tests
- [ ] `GET /notifications` — returns user's notifications only
- [ ] `GET /notifications` — filters by is_read
- [ ] `GET /notifications/unread-count` — correct count
- [ ] `PATCH /notifications/:id/read` — marks as read
- [ ] `PATCH /notifications/:id/read` — other user's notification: 403
- [ ] `PATCH /notifications/read-all` — all marked as read
- [ ] Background consumer: message consumed and notification created in DB

---

## 8. Implementation Tasks

- [ ] Create `notifications` migration with index on `(user_id, is_read, created_at DESC)`
- [ ] Generate SeaORM entity for `notifications`
- [ ] Implement `NotificationsService` with all CRUD operations
- [ ] Implement background consumer task (tokio::spawn)
- [ ] Implement notification publisher helper (used by Build Worker, Org Members, etc.)
- [ ] Implement handlers for all 4 public endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 9. Definition of Done

- [ ] All 4 notification endpoints functional
- [ ] Notifications only visible to owning user
- [ ] Background consumer persists notifications from RabbitMQ
- [ ] Unread count correct
- [ ] Read-all marks all as read atomically
- [ ] All tests pass

---

## 10. Estimated Effort

**Medium (2 days)**

---

## 11. Recommendations

**Required:**
- Notifications must be user-scoped — users can only see and modify their own notifications.
- Read-all must use an atomic UPDATE WHERE user_id = ? and is_read = false.

**Recommended:**
- Include Redis caching for unread count (key: `forge:notif_count:{user_id}`, TTL: 30s) to avoid count query on every page load.

**Future Enhancement:**
- Real-time SSE notification stream.
- Email notification delivery.
- User notification preferences (opt-in/opt-out per type).
