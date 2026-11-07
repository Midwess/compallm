//! Authentication workflows and the token claims they operate on.

pub mod error;
pub mod policy;
pub mod service;
pub mod session;
pub mod token;

pub use error::AuthError;
pub use policy::UserAuth;
pub use service::{AuthService, TokenLifetimes};
pub use session::Session;
pub use token::{
    AccessToken, EmailOtpAccessToken, EncryptedAccessToken, EncryptedToken, RefreshToken,
    TokenValidity, UserAccessToken,
};
pub use token::{EmailOtpChallenge, IssuedEmailOtp, UserTokens};
