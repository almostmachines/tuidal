use crate::events::types::{error_event::ErrorEvent, notification_event::NotificationEvent};

#[derive(Debug, Clone)]
pub enum Event {
    Error(ErrorEvent),
    Notification(NotificationEvent),
}

impl From<ErrorEvent> for Event {
    fn from(error_event: ErrorEvent) -> Self {
        Event::Error(error_event)
    }
}

impl From<NotificationEvent> for Event {
    fn from(notification_event: NotificationEvent) -> Self {
        Event::Notification(notification_event)
    }
}
