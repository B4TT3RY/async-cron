pub mod job;

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    #[allow(unused_imports)]
    use crate::job::Job;

    #[tokio::test]
    async fn create_job_tokio() {
        let mut job = Job::new("* * * * * *").unwrap();

        while let Some(()) = job.next().await {
            println!("Hi~~~");
        }

        println!("Tokio End");
    }

    #[async_std::test]
    async fn create_job_async_std() {
        
    }
}