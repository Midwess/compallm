use compallm_core::authorization::policy::{Action, EntityType};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    juniper::GraphQLEnum,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionAction {
    MemberRead,
    MemberInvite,
    MemberRemove,
    MemberRoleAssign,
    MemberRoleUnassign,
    RoleRead,
    RoleCreate,
    RoleMetadataUpdate,
    RolePolicyRead,
    RolePolicyUpdate,
    RoleDelete,
    RoleAssign,
    RoleUnassign,
    OrganizationSettingsRead,
    OrganizationSettingsUpdate,
    OrganizationDelete,
    GroupRead,
    GroupCreate,
    GroupMetadataUpdate,
    GroupMemberAdd,
    GroupMemberRemove,
    GroupRoleAssign,
    GroupRoleUnassign,
    GroupDelete,
    ProviderRead,
    ProviderCreate,
    ProviderConfigurationUpdate,
    ProviderDelete,
    ProviderCredentialReplace,
    ProviderModelRead,
    ProviderModelCreate,
    ProviderModelConfigurationUpdate,
    ProviderModelPricingUpdate,
    ProviderModelDelete,
    ModelRead,
    ModelCreate,
    ModelConfigurationUpdate,
    ModelDelete,
    ModelUse,
    ModelRouteAdd,
    ModelRouteUpdate,
    ModelRouteRemove,
    ModelPricingUpdate,
}
impl From<PermissionAction> for Action {
    fn from(action: PermissionAction) -> Self {
        match action {
            PermissionAction::MemberRead => Action::MemberRead,
            PermissionAction::MemberInvite => Action::MemberInvite,
            PermissionAction::MemberRemove => Action::MemberRemove,
            PermissionAction::MemberRoleAssign => Action::MemberRoleAssign,
            PermissionAction::MemberRoleUnassign => Action::MemberRoleUnassign,
            PermissionAction::RoleRead => Action::RoleRead,
            PermissionAction::RoleCreate => Action::RoleCreate,
            PermissionAction::RoleMetadataUpdate => Action::RoleMetadataUpdate,
            PermissionAction::RolePolicyRead => Action::RolePolicyRead,
            PermissionAction::RolePolicyUpdate => Action::RolePolicyUpdate,
            PermissionAction::RoleDelete => Action::RoleDelete,
            PermissionAction::RoleAssign => Action::RoleAssign,
            PermissionAction::RoleUnassign => Action::RoleUnassign,
            PermissionAction::OrganizationSettingsRead => Action::OrganizationSettingsRead,
            PermissionAction::OrganizationSettingsUpdate => Action::OrganizationSettingsUpdate,
            PermissionAction::OrganizationDelete => Action::OrganizationDelete,
            PermissionAction::GroupRead => Action::GroupRead,
            PermissionAction::GroupCreate => Action::GroupCreate,
            PermissionAction::GroupMetadataUpdate => Action::GroupMetadataUpdate,
            PermissionAction::GroupMemberAdd => Action::GroupMemberAdd,
            PermissionAction::GroupMemberRemove => Action::GroupMemberRemove,
            PermissionAction::GroupRoleAssign => Action::GroupRoleAssign,
            PermissionAction::GroupRoleUnassign => Action::GroupRoleUnassign,
            PermissionAction::GroupDelete => Action::GroupDelete,
            PermissionAction::ProviderRead => Action::ProviderRead,
            PermissionAction::ProviderCreate => Action::ProviderCreate,
            PermissionAction::ProviderConfigurationUpdate => Action::ProviderConfigurationUpdate,
            PermissionAction::ProviderDelete => Action::ProviderDelete,
            PermissionAction::ProviderCredentialReplace => Action::ProviderCredentialReplace,
            PermissionAction::ProviderModelRead => Action::ProviderModelRead,
            PermissionAction::ProviderModelCreate => Action::ProviderModelCreate,
            PermissionAction::ProviderModelConfigurationUpdate => {
                Action::ProviderModelConfigurationUpdate
            }
            PermissionAction::ProviderModelPricingUpdate => Action::ProviderModelPricingUpdate,
            PermissionAction::ProviderModelDelete => Action::ProviderModelDelete,
            PermissionAction::ModelRead => Action::ModelRead,
            PermissionAction::ModelCreate => Action::ModelCreate,
            PermissionAction::ModelConfigurationUpdate => Action::ModelConfigurationUpdate,
            PermissionAction::ModelDelete => Action::ModelDelete,
            PermissionAction::ModelUse => Action::ModelUse,
            PermissionAction::ModelRouteAdd => Action::ModelRouteAdd,
            PermissionAction::ModelRouteUpdate => Action::ModelRouteUpdate,
            PermissionAction::ModelRouteRemove => Action::ModelRouteRemove,
            PermissionAction::ModelPricingUpdate => Action::ModelPricingUpdate,
        }
    }
}
impl From<Action> for PermissionAction {
    fn from(action: Action) -> Self {
        match action {
            Action::MemberRead => PermissionAction::MemberRead,
            Action::MemberInvite => PermissionAction::MemberInvite,
            Action::MemberRemove => PermissionAction::MemberRemove,
            Action::MemberRoleAssign => PermissionAction::MemberRoleAssign,
            Action::MemberRoleUnassign => PermissionAction::MemberRoleUnassign,
            Action::RoleRead => PermissionAction::RoleRead,
            Action::RoleCreate => PermissionAction::RoleCreate,
            Action::RoleMetadataUpdate => PermissionAction::RoleMetadataUpdate,
            Action::RolePolicyRead => PermissionAction::RolePolicyRead,
            Action::RolePolicyUpdate => PermissionAction::RolePolicyUpdate,
            Action::RoleDelete => PermissionAction::RoleDelete,
            Action::RoleAssign => PermissionAction::RoleAssign,
            Action::RoleUnassign => PermissionAction::RoleUnassign,
            Action::OrganizationSettingsRead => PermissionAction::OrganizationSettingsRead,
            Action::OrganizationSettingsUpdate => PermissionAction::OrganizationSettingsUpdate,
            Action::OrganizationDelete => PermissionAction::OrganizationDelete,
            Action::GroupRead => PermissionAction::GroupRead,
            Action::GroupCreate => PermissionAction::GroupCreate,
            Action::GroupMetadataUpdate => PermissionAction::GroupMetadataUpdate,
            Action::GroupMemberAdd => PermissionAction::GroupMemberAdd,
            Action::GroupMemberRemove => PermissionAction::GroupMemberRemove,
            Action::GroupRoleAssign => PermissionAction::GroupRoleAssign,
            Action::GroupRoleUnassign => PermissionAction::GroupRoleUnassign,
            Action::GroupDelete => PermissionAction::GroupDelete,
            Action::ProviderRead => PermissionAction::ProviderRead,
            Action::ProviderCreate => PermissionAction::ProviderCreate,
            Action::ProviderConfigurationUpdate => PermissionAction::ProviderConfigurationUpdate,
            Action::ProviderDelete => PermissionAction::ProviderDelete,
            Action::ProviderCredentialReplace => PermissionAction::ProviderCredentialReplace,
            Action::ProviderModelRead => PermissionAction::ProviderModelRead,
            Action::ProviderModelCreate => PermissionAction::ProviderModelCreate,
            Action::ProviderModelConfigurationUpdate => {
                PermissionAction::ProviderModelConfigurationUpdate
            }
            Action::ProviderModelPricingUpdate => PermissionAction::ProviderModelPricingUpdate,
            Action::ProviderModelDelete => PermissionAction::ProviderModelDelete,
            Action::ModelRead => PermissionAction::ModelRead,
            Action::ModelCreate => PermissionAction::ModelCreate,
            Action::ModelConfigurationUpdate => PermissionAction::ModelConfigurationUpdate,
            Action::ModelDelete => PermissionAction::ModelDelete,
            Action::ModelUse => PermissionAction::ModelUse,
            Action::ModelRouteAdd => PermissionAction::ModelRouteAdd,
            Action::ModelRouteUpdate => PermissionAction::ModelRouteUpdate,
            Action::ModelRouteRemove => PermissionAction::ModelRouteRemove,
            Action::ModelPricingUpdate => PermissionAction::ModelPricingUpdate,
        }
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    juniper::GraphQLEnum,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionResourceType {
    Organization,
    Member,
    Role,
    Group,
    Provider,
    ProviderModel,
    Model,
}
impl From<PermissionResourceType> for EntityType {
    fn from(value: PermissionResourceType) -> Self {
        match value {
            PermissionResourceType::Organization => EntityType::Organization,
            PermissionResourceType::Member => EntityType::Member,
            PermissionResourceType::Role => EntityType::Role,
            PermissionResourceType::Group => EntityType::Group,
            PermissionResourceType::Provider => EntityType::Provider,
            PermissionResourceType::ProviderModel => EntityType::ProviderModel,
            PermissionResourceType::Model => EntityType::Model,
        }
    }
}

#[derive(
    Clone,
    Debug,
    juniper::GraphQLInputObject,
    validify::Validify,
    compallm_validation_derive::ExportValidation,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PermissionCheckInput {
    pub action: PermissionAction,
    pub resource_type: PermissionResourceType,
    #[schemars(with = "Option<String>")]
    pub resource_id: Option<juniper::ID>,
    #[schemars(with = "Option<String>")]
    pub role_id: Option<juniper::ID>,
    #[schemars(with = "Option<String>")]
    pub member_id: Option<juniper::ID>,
    #[schemars(with = "Option<String>")]
    pub provider_model_id: Option<juniper::ID>,
}
