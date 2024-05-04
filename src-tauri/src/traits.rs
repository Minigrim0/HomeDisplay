use async_trait::async_trait;

#[async_trait]
pub trait Api<T> {
    async fn api_get() -> Result<T, String>;
}

#[async_trait]
pub trait Api1Param<P, T> {
    async fn api_get(param: P) -> Result<T, String>;
}