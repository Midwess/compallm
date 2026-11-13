use crate::{postgres_connection::PostgresConnection, schema::UserPreferenceSchema};
use compallm_core::{
    persistent::{
        DbContext,
        preference::user::{UserPreferenceRepository as Repository, UserPreferenceRepositoryError},
    },
    preference::{
        UserPreferences,
        patch::{UserPreferenceGroup, UserPreferencesPatch},
    },
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, QuerySelect, Set, sea_query::OnConflict};

type Entity = <UserPreferenceSchema as ModelTrait>::Entity;
type Column = <Entity as EntityTrait>::Column;
type ActiveModel = <Entity as EntityTrait>::ActiveModel;

#[derive(Default)]
pub struct UserPreferenceRepository;

impl TryFrom<UserPreferenceSchema> for UserPreferences {
    type Error = UserPreferenceRepositoryError;
    fn try_from(model: UserPreferenceSchema) -> Result<Self, Self::Error> {
        let mut value = Self {
            navigation: serde_json::from_value(model.navigation).map_err(anyhow::Error::from)?,
            sidebar: serde_json::from_value(model.sidebar).map_err(anyhow::Error::from)?,
            member_display: serde_json::from_value(model.member_display)
                .map_err(anyhow::Error::from)?,
            calendar: serde_json::from_value(model.calendar).map_err(anyhow::Error::from)?,
            theme: serde_json::from_value(model.theme).map_err(anyhow::Error::from)?,
            typography: serde_json::from_value(model.typography).map_err(anyhow::Error::from)?,
            accessibility: serde_json::from_value(model.accessibility)
                .map_err(anyhow::Error::from)?,
            message_composition: serde_json::from_value(model.message_composition)
                .map_err(anyhow::Error::from)?,
            code_appearance: serde_json::from_value(model.code_appearance)
                .map_err(anyhow::Error::from)?,
        };
        validify::Validate::validate(&value).map_err(anyhow::Error::from)?;
        value.sidebar.append_missing_defaults();
        Ok(value)
    }
}

struct PreferenceInsert(ActiveModel);
impl TryFrom<(u64, UserPreferences)> for PreferenceInsert {
    type Error = UserPreferenceRepositoryError;
    fn try_from((id, value): (u64, UserPreferences)) -> Result<Self, Self::Error> {
        Ok(Self(ActiveModel {
            user_id: Set(crate::to_bigint(id).map_err(anyhow::Error::from)?),
            navigation: Set(serde_json::to_value(value.navigation).map_err(anyhow::Error::from)?),
            sidebar: Set(serde_json::to_value(value.sidebar).map_err(anyhow::Error::from)?),
            member_display: Set(
                serde_json::to_value(value.member_display).map_err(anyhow::Error::from)?
            ),
            calendar: Set(serde_json::to_value(value.calendar).map_err(anyhow::Error::from)?),
            theme: Set(serde_json::to_value(value.theme).map_err(anyhow::Error::from)?),
            typography: Set(serde_json::to_value(value.typography).map_err(anyhow::Error::from)?),
            accessibility: Set(
                serde_json::to_value(value.accessibility).map_err(anyhow::Error::from)?
            ),
            message_composition: Set(
                serde_json::to_value(value.message_composition).map_err(anyhow::Error::from)?
            ),
            code_appearance: Set(
                serde_json::to_value(value.code_appearance).map_err(anyhow::Error::from)?
            ),
        }))
    }
}

impl UserPreferenceRepository {
    async fn change(
        &self,
        user_id: u64,
        patch: Option<&UserPreferencesPatch>,
        groups: &[UserPreferenceGroup],
    ) -> Result<UserPreferences, UserPreferenceRepositoryError> {
        let context = DbContext::current()?;
        // Insert first so competing first writes serialize on the unique user key.
        context
            .transaction(async {
                self.create_defaults(user_id).await?;
                let context = DbContext::current()?;
                let db = PostgresConnection::from_context(&context)?;
                let model =
                    Entity::find_by_id(crate::to_bigint(user_id).map_err(anyhow::Error::from)?)
                        .lock_exclusive()
                        .one(db)
                        .await
                        .map_err(anyhow::Error::from)?
                        .ok_or_else(|| {
                            UserPreferenceRepositoryError(anyhow::anyhow!("missing preference row"))
                        })?;
                let mut value = UserPreferences::try_from(model)?;
                if let Some(patch) = patch {
                    patch.apply(&mut value).map_err(anyhow::Error::from)?;
                }
                let defaults = UserPreferences::default();
                for group in groups {
                    match group {
                        UserPreferenceGroup::Navigation => {
                            value.navigation = defaults.navigation.clone()
                        }
                        UserPreferenceGroup::Sidebar => value.sidebar = defaults.sidebar.clone(),
                        UserPreferenceGroup::MemberDisplay => {
                            value.member_display = defaults.member_display.clone()
                        }
                        UserPreferenceGroup::Calendar => value.calendar = defaults.calendar.clone(),
                        UserPreferenceGroup::Theme => value.theme = defaults.theme.clone(),
                        UserPreferenceGroup::Typography => {
                            value.typography = defaults.typography.clone()
                        }
                        UserPreferenceGroup::Accessibility => {
                            value.accessibility = defaults.accessibility.clone()
                        }
                        UserPreferenceGroup::MessageComposition => {
                            value.message_composition = defaults.message_composition.clone()
                        }
                        UserPreferenceGroup::CodeAppearance => {
                            value.code_appearance = defaults.code_appearance.clone()
                        }
                        UserPreferenceGroup::All => value = defaults.clone(),
                    }
                }
                let PreferenceInsert(model) = (user_id, value).try_into()?;
                let saved = model.update(db).await.map_err(anyhow::Error::from)?;
                saved.try_into()
            })
            .await
    }
}

#[async_trait::async_trait]
impl Repository for UserPreferenceRepository {
    async fn find(
        &self,
        user_id: u64,
    ) -> Result<Option<UserPreferences>, UserPreferenceRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        Entity::find_by_id(crate::to_bigint(user_id).map_err(anyhow::Error::from)?)
            .one(db)
            .await
            .map_err(anyhow::Error::from)?
            .map(TryInto::try_into)
            .transpose()
    }

    async fn create_defaults(&self, user_id: u64) -> Result<(), UserPreferenceRepositoryError> {
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;
        let PreferenceInsert(model) = (user_id, UserPreferences::default()).try_into()?;
        Entity::insert(model)
            .on_conflict(OnConflict::column(Column::UserId).do_nothing().to_owned())
            .try_insert()
            .exec(db)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }

    async fn update(
        &self,
        user_id: u64,
        patch: &UserPreferencesPatch,
    ) -> Result<UserPreferences, UserPreferenceRepositoryError> {
        if patch.is_empty() {
            return Ok(self.find(user_id).await?.unwrap_or_default());
        }
        self.change(user_id, Some(patch), &[]).await
    }

    async fn reset(
        &self,
        user_id: u64,
        groups: &[UserPreferenceGroup],
    ) -> Result<UserPreferences, UserPreferenceRepositoryError> {
        self.change(user_id, None, groups).await
    }
}
