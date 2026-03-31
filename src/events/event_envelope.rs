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
