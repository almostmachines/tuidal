use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use rxrust::{Context, CoreObservable, ObservableType, Observer};
use std::convert::Infallible;

#[derive(Clone)]
pub struct InputObservable();

impl ObservableType for InputObservable {
    type Item<'a>
        = KeyEvent
    where
        Self: 'a;
    type Err = Infallible;
}

impl<C> CoreObservable<C> for InputObservable
where
    C: Context,
    C::Inner: Observer<KeyEvent, Infallible>,
{
    type Unsub = ();

    fn subscribe(self, context: C) -> Self::Unsub {
        let mut observer = context.into_inner();

        loop {
            if observer.is_closed() {
                break;
            }

            match event::read() {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                    observer.next(key_event);
                }
                Ok(_) => {}
                Err(e) => {
                    if !observer.is_closed() {
                        eprintln!("Event error: {e}");
                    }
                }
            };
        }
    }
}
