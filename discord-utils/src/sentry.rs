#[macro_export]
macro_rules! sentry_track_command {
    ($msg:expr) => {
        sentry::configure_scope(|scope| {
            scope.set_user(Some(sentry::User {
                id: Some($msg.author.id.to_string()),
                email: None,
                ip_address: None,
                username: Some($msg.author.name.clone()),
                ..Default::default()
            }));
        });
        sentry::add_breadcrumb(sentry::Breadcrumb {
            category: Some("command".into()),
            message: Some($msg.content.clone()),
            ..Default::default()
        });
    };
}
