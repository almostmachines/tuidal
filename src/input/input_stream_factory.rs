use rxrust::{Context, Local, LocalCtx, LocalScheduler};

use crate::input::input_observable::InputObservable;

pub fn input_stream_factory() -> LocalCtx<InputObservable, LocalScheduler> {
    Local::new(InputObservable())
}
