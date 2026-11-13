use aws_sdk_s3::error::SdkError;
use compallm_core::persistent::storage::user_space::StorageError;

pub(super) fn status<E: std::fmt::Debug>(error: &SdkError<E>) -> Option<u16> {
    error
        .raw_response()
        .map(|response| response.status().as_u16())
}

pub(super) fn map_error<E: std::fmt::Debug>(_error: SdkError<E>) -> StorageError {
    StorageError::Unavailable
}
