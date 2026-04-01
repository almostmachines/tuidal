use crate::events::{errors::event_envelope_error::EventEnvelopeError, types::{error_event::ErrorEvent, notification_event::NotificationEvent}};

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

impl Event {
    pub fn publish(&self) -> Result<Event, EventEnvelopeError> {
        match self {
            Event::Notification(evt) => evt.publish().map(Event::Notification),
            Event::Error(evt) => evt.publish().map(Event::Error),
        }
    }
}
