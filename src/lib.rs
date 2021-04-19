pub mod job;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::job::Job;

    #[tokio::test]
    async fn create_job_tokio() {
        
    }

    #[async_std::test]
    async fn create_job_async_std() {
        
    }
}