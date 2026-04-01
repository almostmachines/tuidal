use crate::events::{errors::event_envelope_error::EventEnvelopeError, event::Event};

pub fn event_validator(evt: Event) -> Result<Event, EventEnvelopeError> {
    evt.publish()
}
