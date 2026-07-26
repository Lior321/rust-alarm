use crate::timer::Timer;
use events::event::EventHandler;

pub enum GenericHandler {
    TimerEvent(Timer),
}

impl EventHandler for GenericHandler {
    fn handle(&mut self) -> bool {
        match self {
            GenericHandler::TimerEvent(timer) => timer.handle(),
        }
    }
}

impl Clone for GenericHandler {
    fn clone(&self) -> GenericHandler {
        match self {
            GenericHandler::TimerEvent(timer) => GenericHandler::TimerEvent(timer.clone()),
        }
    }
}
