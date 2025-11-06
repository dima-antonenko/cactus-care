use anyhow::Error;
use warp::reject::Reject;

// Custom error type for the application
// Wraps anyhow::Error to work with Warp's rejection system
#[derive(Debug)]
pub struct AppError(pub Error);

impl Reject for AppError {}

// Convert anyhow::Error to AppError
// Allows using ? operator in handlers
impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError(err)
    }
}
