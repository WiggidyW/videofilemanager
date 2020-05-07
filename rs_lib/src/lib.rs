use notify;
use std::time;
use std::sync::mpsc;

pub struct Watcher<T, U> {
    inner: notify::RecommendedWatcher,
    event_handler: T,
    error_handler: U,
    recv: mpsc::Receiver<notify::DebouncedEvent>,
}

impl<T, U, E> Watcher<T, U> where 
    T: Fn(notify::DebouncedEvent) -> Result<(), E>,
    U: Fn(E) -> (),
{
    pub fn new(event_handler: T, error_handler: U, interval: time::Duration) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let watcher = notify::watcher(tx, interval)?;
        Ok(Self {
            inner: watcher,
            event_handler: event_handler,
            error_handler: error_handler,
            recv: rx,
        })
    }

    pub fn with_target<P>(&mut self, path: P, mode: notify::RecursiveMode) -> Result<&mut Self, notify::Error> where
        P: AsRef<std::path::Path>,
    {
        <notify::RecommendedWatcher as notify::Watcher>::watch(&mut self.inner, path, mode)?;
        Ok(self)
    }

    pub fn run(self) -> Result<(), mpsc::RecvError> {
        loop {
            match self.recv.recv() {
                Err(e) => return Err(e),
                Ok(event) =>
            match (self.event_handler)(event) {
                Err(e) => (self.error_handler)(e),
                Ok(()) => (),
        }}}
    }
}