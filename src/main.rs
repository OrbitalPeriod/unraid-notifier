use unraid_notification::notifier::{Notifier, UnraidNotifier};

fn main() {
    let _ = UnraidNotifier::default()
        .send(
            "CRITICAL MESSAGE",
            unraid_notification::notificationlevel::NotificationLevel::Critical,
        )
        .unwrap();
}

// cargo build --release --target x86_64-unknown-linux-musl
