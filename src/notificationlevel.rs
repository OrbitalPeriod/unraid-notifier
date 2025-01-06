#[derive(Debug)]
pub enum UnraidNotifierError {
    InvalidPath,
    IOError(std::io::Error),
}

#[derive(Clone, Copy, Debug)]
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
    pub fn to_levelname(self) -> &'static str {
        match self {
            NotificationLevel::Debug => "DEBUG",
            NotificationLevel::Info => "INFO",
            NotificationLevel::Warning => "WARNING",
            NotificationLevel::Error => "ERROR",
            NotificationLevel::Critical => "CRITICAL",
        }
    }
}
