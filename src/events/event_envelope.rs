use chrono::Utc;

use crate::{
    core::data::maybe_date::MaybeDate, events::errors::event_envelope_error::EventEnvelopeError,
};

#[derive(Debug, Clone)]
pub struct EventEnvelope<T> {
    publication_date: MaybeDate,
    published_by: Option<String>,
    created_by: String,
    data: T,
}

impl<T: Clone> EventEnvelope<T> {
    pub fn new(created_by: String, data: T) -> Result<Self, EventEnvelopeError> {
        if created_by.is_empty() {
            Err(EventEnvelopeError::EmptyCreatedBy)
        } else {
            Ok(Self {
                publication_date: None,
                published_by: None,
                created_by,
                data,
            })
        }
    }

    pub fn mark_as_published(&self, published_by: String) -> Result<Self, EventEnvelopeError> {
        if self.is_published() {
            return Err(EventEnvelopeError::AlreadyPublished);
        }

        if published_by.is_empty() {
            return Err(EventEnvelopeError::EmptyPublishedBy);
        }

        Ok(Self {
            publication_date: Some(Utc::now()),
            published_by: Some(published_by),
            created_by: self.created_by.clone(),
            data: self.data.clone(),
        })
    }

    pub fn is_published(&self) -> bool {
        self.publication_date.is_some() && self.published_by.is_some()
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn created_by(&self) -> &String {
        &self.created_by
    }

    pub fn published_by(&self) -> &Option<String> {
        &self.published_by
    }

    pub fn publication_date(&self) -> &MaybeDate {
        &self.publication_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_envelope_can_instantiate() {
        let evt = EventEnvelope::new(String::from("creator"), 21);

        assert!(evt.is_ok());
    }

    #[test]
    fn test_event_envelope_requires_non_empty_created_by() {
        let evt = EventEnvelope::new(String::from(""), 21);

        assert!(matches!(evt, Err(EventEnvelopeError::EmptyCreatedBy)));
    }

    #[test]
    fn test_event_envelope_not_marked_as_published_on_instantiation() {
        let evt = EventEnvelope::new(String::from("creator"), 21);

        assert!(!evt.unwrap().is_published());
    }

    #[test]
    fn test_event_envelope_has_no_published_by_when_unpublished() {
        let evt = EventEnvelope::new(String::from("creator"), 21);

        assert!(evt.unwrap().published_by().is_none());
    }

    #[test]
    fn test_event_envelope_has_no_publication_date_when_unpublished() {
        let evt = EventEnvelope::new(String::from("creator"), 21);

        assert!(evt.unwrap().publication_date().is_none());
    }

    #[test]
    fn test_event_envelope_data_getter() {
        let data = 21;
        let evt = EventEnvelope::new(String::from("creator"), data);

        assert_eq!(evt.unwrap().data(), &data);
    }

    #[test]
    fn test_event_envelope_created_by_getter() {
        let created_by = String::from("creator");
        let evt = EventEnvelope::new(created_by.clone(), 21);

        assert_eq!(evt.unwrap().created_by(), &created_by);
    }

    #[test]
    fn test_event_envelope_can_mark_as_published() {
        let evt = EventEnvelope::new(String::from("creator"), 21);
        let evt = evt.unwrap().mark_as_published(String::from("publisher"));

        assert!(evt.unwrap().is_published());
    }

    #[test]
    fn test_event_envelope_can_mark_as_published_repeatedly() {
        let evt = EventEnvelope::new(String::from("creator"), 21);
        let evt = evt.unwrap().mark_as_published(String::from("publisher"));
        let evt = evt.unwrap().mark_as_published(String::from("publisher"));

        assert!(matches!(evt, Err(EventEnvelopeError::AlreadyPublished)));
    }

    #[test]
    fn test_event_envelope_published_by_cannot_be_empty() {
        let evt = EventEnvelope::new(String::from("creator"), 21);
        let evt = evt.unwrap().mark_as_published(String::from(""));

        assert!(matches!(evt, Err(EventEnvelopeError::EmptyPublishedBy)));
    }

    #[test]
    fn test_event_envelope_has_published_by_when_published() {
        let publisher = String::from("publisher");
        let evt = EventEnvelope::new(String::from("creator"), 21);
        let evt = evt.unwrap().mark_as_published(publisher.clone());

        assert_eq!(evt.unwrap().published_by(), &Some(publisher));
    }

    #[test]
    fn test_event_envelope_has_publication_date_when_published() {
        let publisher = String::from("publisher");
        let evt = EventEnvelope::new(String::from("creator"), 21);
        let before = Utc::now();
        let evt = evt.unwrap().mark_as_published(publisher.clone());
        let after = Utc::now();
        let pub_date = evt.unwrap().publication_date().unwrap();

        assert!(pub_date >= before && pub_date <= after);
    }
}
