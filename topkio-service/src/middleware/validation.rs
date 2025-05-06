use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    async_trait,
    RequestPartsExt,
};
use validator::Validate;

#[async_trait]
impl<S> FromRequestParts<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = parts.extract::<Json<T>>().await?;
        value.validate().map_err(|e| ApiError::Validation(e))?;
        Ok(ValidatedJson(value))
    }
}

// Wrapper type for validated requests
pub struct ValidatedJson<T>(pub T);