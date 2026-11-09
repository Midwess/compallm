use juniper::{GraphQLScalar, ParseScalarResult, ScalarToken, ScalarValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// Positive u32 context size, encoded as an exact JSON number (1–4294967295).
#[derive(Clone, GraphQLScalar, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(extend("maximum" = u32::MAX))]
pub struct ContextTokens(NonZeroU32);

impl ContextTokens {
    fn to_output<S: ScalarValue>(&self) -> S {
        f64::from(self.0.get()).into()
    }

    fn from_input(value: f64) -> Result<Self, &'static str> {
        if value >= 1.0 && value <= f64::from(u32::MAX) && value.fract() == 0.0 {
            NonZeroU32::new(value as u32)
                .map(Self)
                .ok_or("Expected a positive u32 context size")
        } else {
            Err("Expected a positive u32 context size")
        }
    }

    fn parse_token<S: ScalarValue>(token: ScalarToken<'_>) -> ParseScalarResult<S> {
        match token {
            ScalarToken::Int(value) | ScalarToken::Float(value) => {
                value.parse::<f64>().map(Into::into).map_err(|_| {
                    juniper::ParseError::unexpected_token(juniper::parser::Token::Scalar(token))
                })
            }
            _ => Err(juniper::ParseError::unexpected_token(
                juniper::parser::Token::Scalar(token),
            )),
        }
    }
}
