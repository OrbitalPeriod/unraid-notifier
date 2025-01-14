/// Notification levels equivelent to unraid's
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl NotificationLevel {
    pub fn icon(&self) -> &'static str {
        match self {
            NotificationLevel::Debug | NotificationLevel::Info => "normal",
            NotificationLevel::Warning => "warning",
            NotificationLevel::Error | NotificationLevel::Critical => "alert",
        }
    }
    pub fn to_levelname(&self) -> &'static str {
        match self {
            NotificationLevel::Debug => "DEBUG",
            NotificationLevel::Info => "INFO",
            NotificationLevel::Warning => "WARNING",
            NotificationLevel::Error => "ERROR",
            NotificationLevel::Critical => "CRITICAL",
        }
    }
}

impl std::fmt::Display for NotificationLevel{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_levelname())
    }
}
