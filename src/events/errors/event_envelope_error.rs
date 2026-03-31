use std::fmt;

#[derive(Debug)]
pub enum EventEnvelopeError {
    EmptyCreatedBy,
    EmptyPublishedBy,
    AlreadyPublished,
}

impl fmt::Display for EventEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventEnvelopeError::AlreadyPublished => {
                write!(f, "EventEnvelope: event is already published")
            }
            EventEnvelopeError::EmptyCreatedBy => write!(
                f,
                "EventEnvelope: created_by cannot be empty when creating an event"
            ),
            EventEnvelopeError::EmptyPublishedBy => write!(
                f,
                "EventEnvelope: published_by cannot be empty when publishing an event"
            ),
        }
    }
}

impl std::error::Error for EventEnvelopeError {}
