use anyhow::Context;
use compallm_core::{
    model::{Model, Price},
    persistent::model::ModelRepositoryError,
};
use sea_orm::prelude::Decimal;

use crate::schema::ModelSchema;

impl TryFrom<ModelSchema> for Model {
    type Error = ModelRepositoryError;

    fn try_from(model: ModelSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id).context("invalid model ID in database")?;
        let organization_id = crate::from_bigint(model.organization_id)
            .context("invalid model organization ID in database")?;
        if id == 0 || organization_id == 0 {
            return Err(ModelRepositoryError::database(anyhow::anyhow!(
                "model and organization IDs in database must be positive"
            )));
        }
        Ok(Self {
            id,
            organization_id,
            name: model.name,
            input_price_per_million_tokens: price_from_database(
                model.input_price_per_million_tokens,
            )?,
            output_price_per_million_tokens: price_from_database(
                model.output_price_per_million_tokens,
            )?,
            cached_price_per_million_tokens: price_from_database(
                model.cached_price_per_million_tokens,
            )?,
            settings: model.settings,
        })
    }
}

fn price_from_database(price: Decimal) -> Result<Price, ModelRepositoryError> {
    if price.is_sign_negative() {
        return Err(ModelRepositoryError::database(anyhow::anyhow!(
            "model prices in database must be non-negative"
        )));
    }

    let mantissa = price.mantissa();
    let scaled = match price.scale().cmp(&Price::SCALE) {
        std::cmp::Ordering::Less => mantissa
            .checked_mul(10_i128.pow(Price::SCALE - price.scale()))
            .context("model price exceeds the Core price range")?,
        std::cmp::Ordering::Equal => mantissa,
        std::cmp::Ordering::Greater => {
            let divisor = 10_i128.pow(price.scale() - Price::SCALE);
            if mantissa % divisor != 0 {
                return Err(ModelRepositoryError::database(anyhow::anyhow!(
                    "model price has more than eight decimal places"
                )));
            }
            mantissa / divisor
        }
    };
    Ok(Price::from_scaled(
        scaled
            .try_into()
            .context("model price exceeds the Core price range")?,
    ))
}

pub(crate) fn price_to_database(price: Price) -> Result<Decimal, ModelRepositoryError> {
    let mantissa = price
        .scaled()
        .try_into()
        .context("model price exceeds the PostgreSQL decimal range")?;
    Ok(Decimal::from_i128_with_scale(mantissa, Price::SCALE))
}
