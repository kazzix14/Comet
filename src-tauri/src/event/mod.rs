mod dispatcher;

pub use dispatcher::Dispatcher;

pub enum Event {
    HealthCheck,
}
