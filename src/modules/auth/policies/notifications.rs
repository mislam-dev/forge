define_policy!(
    NotificationsReadPolicy,
    "notifications:read",
    "Policy for viewing notifications"
);
define_policy!(
    NotificationsUpdatePolicy,
    "notifications:update",
    "Policy for marking notifications as read"
);
define_policy!(
    NotificationsDeletePolicy,
    "notifications:delete",
    "Policy for dismissing notifications"
);
define_policy!(
    NotificationsStreamPolicy,
    "notifications:stream",
    "Policy for streaming real-time notifications SSE"
);
