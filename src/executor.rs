use iced::Executor;
use std::future::Future;
use std::io::Error;
use tokio::runtime::Runtime;

pub struct TokioExecutor {
    runtime: Runtime,
}

impl Executor for TokioExecutor {
    fn new() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(TokioExecutor {
            runtime: Runtime::new()?,
        })
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.runtime.spawn(future);
    }
}
