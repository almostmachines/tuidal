use crate::events::{errors::event_envelope_error::EventEnvelopeError, event_envelope::EventEnvelope};

#[derive(Debug, Clone)]
pub enum ErrorEvent {
    Input(EventEnvelope<String>),
}

impl ErrorEvent {
    pub fn publish(&self) -> Result<ErrorEvent, EventEnvelopeError> {
        match self {
            ErrorEvent::Input(evt) => evt.mark_as_published().map(ErrorEvent::Input),
        }
    }
}
