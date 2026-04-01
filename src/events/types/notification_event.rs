use crossterm::event::KeyEvent;

use crate::events::{errors::event_envelope_error::EventEnvelopeError, event_envelope::EventEnvelope};

#[derive(Debug, Clone)]
pub enum NotificationEvent {
    KeyPress(EventEnvelope<KeyEvent>),
}

impl NotificationEvent {
    pub fn publish(&self) -> Result<NotificationEvent, EventEnvelopeError> {
        match self {
            NotificationEvent::KeyPress(evt) => evt.mark_as_published().map(NotificationEvent::KeyPress),
        }
    }
}
