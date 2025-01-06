use unraid_notification::notifier::{Notifier, UnraidNotifier};

fn main() {
    UnraidNotifier::default()
        .send(
            "CRITICAL MESSAGE",
            unraid_notification::notificationlevel::NotificationLevel::Critical,
        )
        .unwrap();

    let _test = UnraidNotifier::new("wat", "sender");
}

// cargo build --release --target x86_64-unknown-linux-musl
