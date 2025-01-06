//! This crate provides functionality for sending notifications to the web gui in Unraid.
//!
//! # Modules
//!
//! - `notifier`: Contains the main notification logic.
//! - `notificationlevel`: Defines different levels of notifications.
//! - `verifypath`: Provides path verification utilities.
//! - 'unraidnotifiererror': Provides error handling.

pub mod notificationlevel;
pub mod notifier;
pub mod unraidnotifiererror;
pub mod verifypath;
