define_policy!(
    BuildLogsReadPolicy,
    "build_logs:read",
    "Policy for viewing build logs"
);
define_policy!(
    BuildLogsStreamPolicy,
    "build_logs:stream",
    "Policy for streaming live build logs SSE"
);
define_policy!(
    BuildLogsDownloadPolicy,
    "build_logs:download",
    "Policy for downloading build log files"
);
