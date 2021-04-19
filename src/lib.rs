mod job;
mod scheduler;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::job::Job;

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn create_job_tokio() {
        let mut job = Job::new("0 * * * * *", |id| async move {
            println!("Fire {}", id);
        }).unwrap();

        job.run_task().await;
    }

    #[cfg(feature = "async-std")]
    #[async_std::test]
    async fn create_job_async_std() {
        let mut job = Job::new("0 * * * * *", |id| async move {
            println!("Fire {}", id);
        }).unwrap();

        job.run_task().await;
    }
}