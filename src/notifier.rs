use std::{ffi::OsStr, fmt::Display, path::Path, process::Command};

use crate::{
    notificationlevel::{NotificationLevel, UnraidNotifierError},
    verifypath::VerifyPath,
};

pub fn send<S, M, N>(
    message: M,
    level: NotificationLevel,
    sender: N,
    path: &S,
) -> Result<(), UnraidNotifierError>
where
    S: AsRef<OsStr> + VerifyPath,
    M: Display,
    N: Display,
{
    let icon = level.icon();
    path.verify_path()?;

    if let Err(err) = Command::new(path)
        .arg("-e")
        .arg(sender.to_string())
        .arg("-s")
        .arg(level.to_levelname())
        .arg("-d")
        .arg(message.to_string())
        .arg("-i")
        .arg(icon)
        .spawn()
    {
        Err(UnraidNotifierError::IOError(err))
    } else {
        Ok(())
    }
}

pub trait Notifier<M: Display> {
    fn send(&self, message: M, level: NotificationLevel) -> Result<(), UnraidNotifierError>;
}

pub struct UnraidNotifier<S, M>
where
    S: AsRef<OsStr> + VerifyPath,
    M: Display,
{
    notify_cmd_path: S,
    sender: M,
}

impl<S, M> UnraidNotifier<S, M>
where
    S: AsRef<OsStr> + VerifyPath,
    M: Display,
{
    pub fn new(notify_cmd_path: S, sender: M) -> Result<Self, UnraidNotifierError> {
        notify_cmd_path.verify_path()?;

        Ok(Self {
            notify_cmd_path,
            sender,
        })
    }
    pub fn with_path<A: Into<S>>(self, path: A) -> Self {
        Self {
            notify_cmd_path: path.into(),
            ..self
        }
    }
    pub fn with_sender<A: Into<M>>(self, sender: A) -> Self {
        Self {
            sender: sender.into(),
            ..self
        }
    }
    pub fn modify_path<A: Into<S>>(&mut self, path: A) -> Result<(), UnraidNotifierError> {
        let path = path.into();
        path.verify_path()?;
        self.notify_cmd_path = path;
        Ok(())
    }
}

impl<S, M> Notifier<M> for UnraidNotifier<S, M>
where
    S: AsRef<OsStr> + VerifyPath,
    M: Display,
{
    fn send(&self, message: M, level: NotificationLevel) -> Result<(), UnraidNotifierError> {
        send(message, level, &self.sender, &self.notify_cmd_path)
    }
}

static DEFAULT_CMD_PATH: &str = "/usr/local/emhttp/webGui/scripts/notify";

// Implement Default for UnraidNotifier<PathBuf, String>
impl Default for UnraidNotifier<&Path, String> {
    fn default() -> Self {
        Self {
            sender: "rust-unraid-notifier".into(),
            notify_cmd_path: Path::new(DEFAULT_CMD_PATH),
        }
    }
}

// Implement Default for UnraidNotifier<PathBuf, &str>
impl Default for UnraidNotifier<&Path, &str> {
    fn default() -> Self {
        Self {
            sender: "rust-unraid-notifier",
            notify_cmd_path: Path::new(DEFAULT_CMD_PATH),
        }
    }
}
