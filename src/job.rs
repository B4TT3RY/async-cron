use std::{future::Future, str::FromStr};

use cron::Schedule;
use uuid::Uuid;

pub struct Job<'a, F>
where
    F: Future
{
    id: Uuid,
    schedule: Schedule,
    run: Box<dyn Fn(Uuid) -> F + 'a>,
}

impl<'a, F> Job<'a, F>
where
    F: Future
{
    pub fn new<T>(expression: &str, run: T) -> Result<Self, cron::error::Error>
    where
        T: Fn(Uuid) -> F + 'a,
    {
        Ok(Job {
            id: Uuid::new_v4(),
            schedule: Schedule::from_str(expression)?,
            run: Box::new(run),
        })
    }

    pub async fn run_task(&mut self) {
        (self.run)(self.id).await;
    }
}