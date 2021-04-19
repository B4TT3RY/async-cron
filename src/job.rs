use chrono::{DateTime, Utc};
use futures::Stream;
use std::{pin::Pin, str::FromStr, task::{Context, Poll}};

use cron::Schedule;

pub struct Job {
    schedule: Schedule,
    last_tick: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new(expression: &str) -> Result<Self, cron::error::Error> {
        Ok(Job {
            schedule: Schedule::from_str(expression)?,
            last_tick: None,
        })
    }
}

impl Stream for Job {
    type Item = ();

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let now = Utc::now();
        
        if self.last_tick.is_none() {
            self.get_mut().last_tick = Some(now);
            println!("last_tick isn't set");
            return Poll::Ready(Some(()));
        }

        let event = self
            .schedule
            .after(&self.last_tick.unwrap())
            .take(1)
            .next()
            .unwrap();

        if event > now {
            println!("event: {}, now: {}", event, now);
            return Poll::Ready(Some(()));
        }

        Poll::Ready(Some(()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}