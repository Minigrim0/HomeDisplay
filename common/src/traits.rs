use async_trait::async_trait;

#[async_trait]
/// A trait to define the API functions
/// This trait is used to define the API functions that are used to fetch data from the internet
/// The functions are async and return a Result with the data or an error message
pub trait Api<P, T> {
    async fn api_get(param: P) -> Result<T, String>;
}
