use futures::Stream;
use std::{pin::Pin, str::FromStr, task::{Context, Poll}};

use cron::Schedule;

pub struct Job {
    schedule: Schedule,
}

impl Job {
    pub fn new(expression: &str) -> Result<Self, cron::error::Error> {
        Ok(Job {
            schedule: Schedule::from_str(expression)?,
        })
    }
}

impl Stream for Job {
    type Item = ();

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        todo!()
    }
}