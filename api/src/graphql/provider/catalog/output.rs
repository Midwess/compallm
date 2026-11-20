#[derive(juniper::GraphQLObject)]
pub struct CatalogRequiredInput {
    pub name: String,
    pub kind: TemplateInputKind,
    pub required: bool,
    pub allowed_values: Option<Vec<String>>,
}
impl From<compallm_core::provider_template::semantics::RequiredInput> for CatalogRequiredInput {
    fn from(value: compallm_core::provider_template::semantics::RequiredInput) -> Self {
        Self {
            name: value.name,
            kind: value.kind.into(),
            required: value.required,
            allowed_values: value.allowed_values.map(|v| v.into_iter().collect()),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateAuthConfig {
    pub status: TemplateAuthStatus,
    pub placements: Option<Vec<TemplateCredentialPlacement>>,
    pub flow: Option<String>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub reason: Option<String>,
    pub source_urls: Option<Vec<String>>,
    pub adapter: Option<String>,
}
impl From<compallm_core::provider_template::semantics::AuthConfig> for TemplateAuthConfig {
    fn from(value: compallm_core::provider_template::semantics::AuthConfig) -> Self {
        Self {
            status: value.status.into(),
            placements: value
                .placements
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            flow: value.flow.map(|v| {
                serde_json::to_value(v)
                    .expect("serializable catalog enum")
                    .as_str()
                    .expect("string catalog enum")
                    .to_owned()
            }),
            authorization_url: value.authorization_url,
            token_url: value.token_url,
            device_authorization_url: value.device_authorization_url,
            scopes: value.scopes.map(|v| v.into_iter().collect()),
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reason: value.reason,
            source_urls: value.source_urls.map(|v| v.into_iter().collect()),
            adapter: value.adapter,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateCredentialPlacement {
    pub operations: Vec<String>,
    pub location: TemplateCredentialLocation,
    pub name: String,
    pub prefix: String,
    pub adapter_requirements: Vec<String>,
}
impl From<compallm_core::provider_template::semantics::CredentialPlacement>
    for TemplateCredentialPlacement
{
    fn from(value: compallm_core::provider_template::semantics::CredentialPlacement) -> Self {
        Self {
            operations: value.operations.into_iter().collect(),
            location: value.location.into(),
            name: value.name,
            prefix: value.prefix,
            adapter_requirements: value.adapter_requirements.into_iter().collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateEndpointConfig {
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub source_urls: Option<Vec<String>>,
    pub reason: Option<String>,
    pub source_index: Option<String>,
}
impl From<compallm_core::provider_template::semantics::EndpointConfig> for TemplateEndpointConfig {
    fn from(value: compallm_core::provider_template::semantics::EndpointConfig) -> Self {
        Self {
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            source_urls: value.source_urls.map(|v| v.into_iter().collect()),
            reason: value.reason,
            source_index: value.source_index.map(|v| v.to_string()),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplatePricingConfig {
    pub currency: Option<String>,
    pub unit_quantity: Option<String>,
    pub tiers: Option<Vec<TemplateTokenTier>>,
    pub schedule: Option<TemplatePricingSchedule>,
    pub subscription: Option<TemplateSubscription>,
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub reason: Option<String>,
    pub native_reference: Option<String>,
    pub rates: Option<Vec<TemplateUnitRate>>,
    pub conversion: Option<TemplateCreditConversion>,
    pub source_index: Option<String>,
    pub free_evidence: Option<TemplateFreeEvidence>,
    pub cache_evidence: Option<TemplateSourceEvidence>,
    pub zero_price_evidence: Option<Vec<TemplatePricingConfigZeroPriceEvidenceEntry>>,
    pub fallback: Option<TemplateFallbackPricing>,
    pub cost_proxy: Option<TemplateFallbackPricing>,
    pub route_options: Option<Vec<TemplateRoutePricing>>,
    pub route_policy: Option<TemplateRoutePolicy>,
}
impl From<compallm_core::provider_template::semantics::PricingConfig> for TemplatePricingConfig {
    fn from(value: compallm_core::provider_template::semantics::PricingConfig) -> Self {
        Self {
            currency: value.currency,
            unit_quantity: value.unit_quantity.map(|v| v.to_string()),
            tiers: value
                .tiers
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            schedule: value.schedule.map(|v| v.into()),
            subscription: value.subscription.map(|v| v.into()),
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            reason: value.reason,
            native_reference: value.native_reference.map(|v| v.to_string()),
            rates: value
                .rates
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            conversion: value.conversion.map(|v| v.into()),
            source_index: value.source_index.map(|v| v.to_string()),
            free_evidence: value.free_evidence.map(|v| v.into()),
            cache_evidence: value.cache_evidence.map(|v| v.into()),
            zero_price_evidence: value.zero_price_evidence.map(|v| {
                v.into_iter()
                    .map(|(key, value)| TemplatePricingConfigZeroPriceEvidenceEntry {
                        key,
                        value: value.into(),
                    })
                    .collect()
            }),
            fallback: value.fallback.map(|v| v.into()),
            cost_proxy: value.cost_proxy.map(|v| v.into()),
            route_options: value
                .route_options
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            route_policy: value.route_policy.map(|v| v.into()),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateFreeEvidence {
    pub source_index: String,
    pub reason: String,
    pub restrictions: Vec<TemplateFreeEvidenceRestrictionsEntry>,
}
impl From<compallm_core::provider_template::semantics::FreeEvidence> for TemplateFreeEvidence {
    fn from(value: compallm_core::provider_template::semantics::FreeEvidence) -> Self {
        Self {
            source_index: value.source_index.to_string(),
            reason: value.reason,
            restrictions: value
                .restrictions
                .into_iter()
                .map(|(key, value)| TemplateFreeEvidenceRestrictionsEntry {
                    key,
                    value: value.to_string(),
                })
                .collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSourceEvidence {
    pub source_index: String,
    pub reason: String,
}
impl From<compallm_core::provider_template::semantics::SourceEvidence> for TemplateSourceEvidence {
    fn from(value: compallm_core::provider_template::semantics::SourceEvidence) -> Self {
        Self {
            source_index: value.source_index.to_string(),
            reason: value.reason,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateTokenTier {
    pub from_tokens: String,
    pub to_tokens: Option<String>,
    pub rates: TemplateTokenRates,
}
impl From<compallm_core::provider_template::semantics::TokenTier> for TemplateTokenTier {
    fn from(value: compallm_core::provider_template::semantics::TokenTier) -> Self {
        Self {
            from_tokens: value.from_tokens.to_string(),
            to_tokens: value.to_tokens.map(|v| v.to_string()),
            rates: value.rates.into(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateTokenRates {
    pub input_price: String,
    pub output_price: String,
    pub cached_price: Option<String>,
}
impl From<compallm_core::provider_template::semantics::TokenRates> for TemplateTokenRates {
    fn from(value: compallm_core::provider_template::semantics::TokenRates) -> Self {
        Self {
            input_price: value.input_price,
            output_price: value.output_price,
            cached_price: value.cached_price,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSubscription {
    pub product: String,
    pub plan_mode: TemplateSubscriptionPlanMode,
    pub required_inputs: Option<Vec<CatalogRequiredInput>>,
    pub source_urls: Vec<String>,
    pub plan: Option<String>,
    pub entitlement: Option<TemplateEntitlement>,
    pub fee: Option<TemplateSubscriptionFee>,
}
impl From<compallm_core::provider_template::semantics::Subscription> for TemplateSubscription {
    fn from(value: compallm_core::provider_template::semantics::Subscription) -> Self {
        Self {
            product: value.product,
            plan_mode: value.plan_mode.into(),
            required_inputs: value
                .required_inputs
                .map(|v| v.into_iter().map(|v| v.into()).collect()),
            source_urls: value.source_urls.into_iter().collect(),
            plan: value.plan,
            entitlement: value.entitlement.map(|v| v.into()),
            fee: value.fee.map(|v| v.into()),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSubscriptionFee {
    pub amount: String,
    pub currency: String,
    pub period: String,
}
impl From<compallm_core::provider_template::semantics::SubscriptionFee>
    for TemplateSubscriptionFee
{
    fn from(value: compallm_core::provider_template::semantics::SubscriptionFee) -> Self {
        Self {
            amount: value.amount,
            currency: value.currency,
            period: value.period,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateEntitlement {
    pub unit: String,
    pub quota: String,
    pub period: String,
    pub overage_policy: String,
}
impl From<compallm_core::provider_template::semantics::Entitlement> for TemplateEntitlement {
    fn from(value: compallm_core::provider_template::semantics::Entitlement) -> Self {
        Self {
            unit: value.unit,
            quota: value.quota.to_string(),
            period: value.period,
            overage_policy: value.overage_policy,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateUnitRate {
    pub amount: String,
    pub quantity: String,
    pub conditions: Vec<TemplateUnitRateConditionsEntry>,
}
impl From<compallm_core::provider_template::semantics::UnitRate> for TemplateUnitRate {
    fn from(value: compallm_core::provider_template::semantics::UnitRate) -> Self {
        Self {
            amount: value.amount,
            quantity: value.quantity.to_string(),
            conditions: value
                .conditions
                .into_iter()
                .map(|(key, value)| TemplateUnitRateConditionsEntry { key, value })
                .collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateCreditConversion {
    pub usd_per_credit: String,
    pub currency: String,
}
impl From<compallm_core::provider_template::semantics::CreditConversion>
    for TemplateCreditConversion
{
    fn from(value: compallm_core::provider_template::semantics::CreditConversion) -> Self {
        Self {
            usd_per_credit: value.usd_per_credit,
            currency: value.currency,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateUnresolvedField {
    pub status: TemplateMissingStatus,
    pub reason: String,
}
impl From<compallm_core::provider_template::semantics::UnresolvedField>
    for TemplateUnresolvedField
{
    fn from(value: compallm_core::provider_template::semantics::UnresolvedField) -> Self {
        Self {
            status: value.status.into(),
            reason: value.reason,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateRoutePricing {
    pub model_id: String,
    pub tag: String,
    pub provider: String,
    pub context_tokens: String,
    pub max_input_tokens: Option<String>,
    pub max_output_tokens: Option<String>,
    pub implicit_caching: Option<bool>,
    pub rates: TemplateTokenRates,
    pub tiers: Vec<TemplateTokenTier>,
    pub source_index: String,
}
impl From<compallm_core::provider_template::semantics::RoutePricing> for TemplateRoutePricing {
    fn from(value: compallm_core::provider_template::semantics::RoutePricing) -> Self {
        Self {
            model_id: value.model_id,
            tag: value.tag,
            provider: value.provider,
            context_tokens: value.context_tokens.to_string(),
            max_input_tokens: value.max_input_tokens.map(|v| v.to_string()),
            max_output_tokens: value.max_output_tokens.map(|v| v.to_string()),
            implicit_caching: value.implicit_caching,
            rates: value.rates.into(),
            tiers: value.tiers.into_iter().map(|v| v.into()).collect(),
            source_index: value.source_index.to_string(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateRoutePolicy {
    pub allow_fallbacks: bool,
    pub exact_provider_selection: bool,
}
impl From<compallm_core::provider_template::semantics::RoutePolicy> for TemplateRoutePolicy {
    fn from(value: compallm_core::provider_template::semantics::RoutePolicy) -> Self {
        Self {
            allow_fallbacks: value.allow_fallbacks,
            exact_provider_selection: value.exact_provider_selection,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplatePricingSchedule {
    pub timezone: String,
    pub slots: Vec<TemplateTimePriceSlot>,
}
impl From<compallm_core::provider_template::semantics::PricingSchedule>
    for TemplatePricingSchedule
{
    fn from(value: compallm_core::provider_template::semantics::PricingSchedule) -> Self {
        Self {
            timezone: value.timezone,
            slots: value.slots.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateTimePriceSlot {
    pub from_hour: i32,
    pub to_hour: i32,
    pub rates: TemplateTokenRates,
}
impl From<compallm_core::provider_template::semantics::TimePriceSlot> for TemplateTimePriceSlot {
    fn from(value: compallm_core::provider_template::semantics::TimePriceSlot) -> Self {
        Self {
            from_hour: i32::from(value.from_hour),
            to_hour: i32::from(value.to_hour),
            rates: value.rates.into(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateFallbackPricing {
    pub currency: String,
    pub unit_quantity: String,
    pub input_price: Option<String>,
    pub output_price: Option<String>,
    pub cached_price: Option<String>,
    pub basis: TemplateFallbackBasis,
    pub reference_provider: String,
    pub reference_model: String,
    pub source_urls: Vec<String>,
    pub confidence: TemplateEstimateConfidence,
    pub notes: String,
}
impl From<compallm_core::provider_template::semantics::FallbackPricing>
    for TemplateFallbackPricing
{
    fn from(value: compallm_core::provider_template::semantics::FallbackPricing) -> Self {
        Self {
            currency: value.currency,
            unit_quantity: value.unit_quantity.to_string(),
            input_price: value.input_price,
            output_price: value.output_price,
            cached_price: value.cached_price,
            basis: value.basis.into(),
            reference_provider: value.reference_provider,
            reference_model: value.reference_model,
            source_urls: value.source_urls.into_iter().collect(),
            confidence: value.confidence.into(),
            notes: value.notes,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateModelInfoSource {
    pub kind: TemplateSourceKind,
    pub url: Option<String>,
    pub reference: Option<String>,
    pub access: TemplateSourceAccess,
    pub status: TemplateSourceStatus,
    pub fields: Vec<String>,
    pub request: Option<TemplateSourceRequest>,
    pub response: Option<TemplateSourceResponse>,
    pub pagination: Option<TemplatePagePagination>,
    pub revision: Option<String>,
    pub notes: String,
}
impl From<compallm_core::provider_template::model_info::ModelInfoSource>
    for TemplateModelInfoSource
{
    fn from(value: compallm_core::provider_template::model_info::ModelInfoSource) -> Self {
        Self {
            kind: value.kind.into(),
            url: value.url,
            reference: value.reference,
            access: value.access.into(),
            status: value.status.into(),
            fields: value
                .fields
                .into_iter()
                .map(|v| {
                    serde_json::to_value(v)
                        .expect("serializable catalog enum")
                        .as_str()
                        .expect("string catalog enum")
                        .to_owned()
                })
                .collect(),
            request: value.request.map(|v| v.into()),
            response: value.response.map(|v| v.into()),
            pagination: value.pagination.map(|v| v.into()),
            revision: value.revision,
            notes: value.notes,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSourceRequest {
    pub method: TemplateRetrievalMethod,
    pub authentication: TemplateSourceAuthentication,
}
impl From<compallm_core::provider_template::model_info::SourceRequest> for TemplateSourceRequest {
    fn from(value: compallm_core::provider_template::model_info::SourceRequest) -> Self {
        Self {
            method: value.method.into(),
            authentication: value.authentication.into(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSourceResponse {
    pub format: TemplateResponseFormat,
    pub object_pointer: String,
    pub identity_pointer: String,
    pub identity_value: Option<String>,
    pub fields: Vec<TemplateSourceResponseFieldsEntry>,
}
impl From<compallm_core::provider_template::model_info::SourceResponse> for TemplateSourceResponse {
    fn from(value: compallm_core::provider_template::model_info::SourceResponse) -> Self {
        Self {
            format: value.format.into(),
            object_pointer: value.object_pointer,
            identity_pointer: value.identity_pointer,
            identity_value: value.identity_value,
            fields: value
                .fields
                .into_iter()
                .map(|(key, value)| TemplateSourceResponseFieldsEntry {
                    key: serde_json::to_value(key)
                        .expect("serializable catalog enum")
                        .as_str()
                        .expect("string catalog enum")
                        .to_owned(),
                    value: value.into(),
                })
                .collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateFieldLocation {
    pub pointer: String,
    pub unit: TemplateSourceUnit,
}
impl From<compallm_core::provider_template::model_info::FieldLocation> for TemplateFieldLocation {
    fn from(value: compallm_core::provider_template::model_info::FieldLocation) -> Self {
        Self {
            pointer: value.pointer,
            unit: value.unit.into(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplatePagePagination {
    pub page_parameter: String,
    pub page_size_parameter: String,
    pub page_size: i32,
    pub total_pages_pointer: String,
}
impl From<compallm_core::provider_template::model_info::PagePagination> for TemplatePagePagination {
    fn from(value: compallm_core::provider_template::model_info::PagePagination) -> Self {
        Self {
            page_parameter: value.page_parameter,
            page_size_parameter: value.page_size_parameter,
            page_size: i32::from(value.page_size),
            total_pages_pointer: value.total_pages_pointer,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct TemplatePricingConfigZeroPriceEvidenceEntry {
    pub key: String,
    pub value: TemplateSourceEvidence,
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateFreeEvidenceRestrictionsEntry {
    pub key: String,
    pub value: String,
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateUnitRateConditionsEntry {
    pub key: String,
    pub value: String,
}

#[derive(juniper::GraphQLObject)]
pub struct TemplateSourceResponseFieldsEntry {
    pub key: String,
    pub value: TemplateFieldLocation,
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateInputKind {
    String,
    Url,
    Object,
    PositiveInteger,
    Price,
}
impl From<compallm_core::provider_template::semantics::InputKind> for TemplateInputKind {
    fn from(v: compallm_core::provider_template::semantics::InputKind) -> Self {
        match v {
            compallm_core::provider_template::semantics::InputKind::String => Self::String,
            compallm_core::provider_template::semantics::InputKind::Url => Self::Url,
            compallm_core::provider_template::semantics::InputKind::Object => Self::Object,
            compallm_core::provider_template::semantics::InputKind::PositiveInteger => {
                Self::PositiveInteger
            }
            compallm_core::provider_template::semantics::InputKind::Price => Self::Price,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateAuthStatus {
    Configured,
    Unknown,
}
impl From<compallm_core::provider_template::semantics::AuthStatus> for TemplateAuthStatus {
    fn from(v: compallm_core::provider_template::semantics::AuthStatus) -> Self {
        match v {
            compallm_core::provider_template::semantics::AuthStatus::Configured => Self::Configured,
            compallm_core::provider_template::semantics::AuthStatus::Unknown => Self::Unknown,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateCredentialLocation {
    Header,
    Query,
}
impl From<compallm_core::provider_template::semantics::CredentialLocation>
    for TemplateCredentialLocation
{
    fn from(v: compallm_core::provider_template::semantics::CredentialLocation) -> Self {
        match v {
            compallm_core::provider_template::semantics::CredentialLocation::Header => Self::Header,
            compallm_core::provider_template::semantics::CredentialLocation::Query => Self::Query,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSubscriptionPlanMode {
    Stored,
    AccountSpecific,
}
impl From<compallm_core::provider_template::semantics::SubscriptionPlanMode>
    for TemplateSubscriptionPlanMode
{
    fn from(v: compallm_core::provider_template::semantics::SubscriptionPlanMode) -> Self {
        match v {
            compallm_core::provider_template::semantics::SubscriptionPlanMode::Stored => {
                Self::Stored
            }
            compallm_core::provider_template::semantics::SubscriptionPlanMode::AccountSpecific => {
                Self::AccountSpecific
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateMissingStatus {
    Unknown,
    Unsupported,
    Unpublished,
}
impl From<compallm_core::provider_template::semantics::MissingStatus> for TemplateMissingStatus {
    fn from(v: compallm_core::provider_template::semantics::MissingStatus) -> Self {
        match v {
            compallm_core::provider_template::semantics::MissingStatus::Unknown => Self::Unknown,
            compallm_core::provider_template::semantics::MissingStatus::Unsupported => {
                Self::Unsupported
            }
            compallm_core::provider_template::semantics::MissingStatus::Unpublished => {
                Self::Unpublished
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateFallbackBasis {
    SameModelOtherProvider,
    RelatedModel,
    TargetProviderEstimate,
}
impl From<compallm_core::provider_template::semantics::FallbackBasis> for TemplateFallbackBasis {
    fn from(v: compallm_core::provider_template::semantics::FallbackBasis) -> Self {
        match v {
            compallm_core::provider_template::semantics::FallbackBasis::SameModelOtherProvider => {
                Self::SameModelOtherProvider
            }
            compallm_core::provider_template::semantics::FallbackBasis::RelatedModel => {
                Self::RelatedModel
            }
            compallm_core::provider_template::semantics::FallbackBasis::TargetProviderEstimate => {
                Self::TargetProviderEstimate
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateEstimateConfidence {
    High,
    Medium,
    Low,
}
impl From<compallm_core::provider_template::semantics::EstimateConfidence>
    for TemplateEstimateConfidence
{
    fn from(v: compallm_core::provider_template::semantics::EstimateConfidence) -> Self {
        match v {
            compallm_core::provider_template::semantics::EstimateConfidence::High => Self::High,
            compallm_core::provider_template::semantics::EstimateConfidence::Medium => Self::Medium,
            compallm_core::provider_template::semantics::EstimateConfidence::Low => Self::Low,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSourceKind {
    Api,
    Documentation,
    Repository,
    Manual,
}
impl From<compallm_core::provider_template::model_info::SourceKind> for TemplateSourceKind {
    fn from(v: compallm_core::provider_template::model_info::SourceKind) -> Self {
        match v {
            compallm_core::provider_template::model_info::SourceKind::Api => Self::Api,
            compallm_core::provider_template::model_info::SourceKind::Documentation => {
                Self::Documentation
            }
            compallm_core::provider_template::model_info::SourceKind::Repository => {
                Self::Repository
            }
            compallm_core::provider_template::model_info::SourceKind::Manual => Self::Manual,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSourceAccess {
    Public,
    ProviderCredential,
    Manual,
    Unknown,
    Unavailable,
}
impl From<compallm_core::provider_template::model_info::SourceAccess> for TemplateSourceAccess {
    fn from(v: compallm_core::provider_template::model_info::SourceAccess) -> Self {
        match v {
            compallm_core::provider_template::model_info::SourceAccess::Public => Self::Public,
            compallm_core::provider_template::model_info::SourceAccess::ProviderCredential => {
                Self::ProviderCredential
            }
            compallm_core::provider_template::model_info::SourceAccess::Manual => Self::Manual,
            compallm_core::provider_template::model_info::SourceAccess::Unknown => Self::Unknown,
            compallm_core::provider_template::model_info::SourceAccess::Unavailable => {
                Self::Unavailable
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSourceStatus {
    Observed,
    Documented,
    Reference,
    Unavailable,
}
impl From<compallm_core::provider_template::model_info::SourceStatus> for TemplateSourceStatus {
    fn from(v: compallm_core::provider_template::model_info::SourceStatus) -> Self {
        match v {
            compallm_core::provider_template::model_info::SourceStatus::Observed => Self::Observed,
            compallm_core::provider_template::model_info::SourceStatus::Documented => {
                Self::Documented
            }
            compallm_core::provider_template::model_info::SourceStatus::Reference => {
                Self::Reference
            }
            compallm_core::provider_template::model_info::SourceStatus::Unavailable => {
                Self::Unavailable
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateRetrievalMethod {
    Get,
}
impl From<compallm_core::provider_template::model_info::RetrievalMethod>
    for TemplateRetrievalMethod
{
    fn from(v: compallm_core::provider_template::model_info::RetrievalMethod) -> Self {
        match v {
            compallm_core::provider_template::model_info::RetrievalMethod::Get => Self::Get,
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSourceAuthentication {
    None,
    ProviderApiKey,
    ProviderOauth,
    Unknown,
}
impl From<compallm_core::provider_template::model_info::SourceAuthentication>
    for TemplateSourceAuthentication
{
    fn from(v: compallm_core::provider_template::model_info::SourceAuthentication) -> Self {
        match v {
            compallm_core::provider_template::model_info::SourceAuthentication::None => Self::None,
            compallm_core::provider_template::model_info::SourceAuthentication::ProviderApiKey => {
                Self::ProviderApiKey
            }
            compallm_core::provider_template::model_info::SourceAuthentication::ProviderOauth => {
                Self::ProviderOauth
            }
            compallm_core::provider_template::model_info::SourceAuthentication::Unknown => {
                Self::Unknown
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateResponseFormat {
    ModelList,
    ModelObject,
}
impl From<compallm_core::provider_template::model_info::ResponseFormat> for TemplateResponseFormat {
    fn from(v: compallm_core::provider_template::model_info::ResponseFormat) -> Self {
        match v {
            compallm_core::provider_template::model_info::ResponseFormat::ModelList => {
                Self::ModelList
            }
            compallm_core::provider_template::model_info::ResponseFormat::ModelObject => {
                Self::ModelObject
            }
        }
    }
}

#[derive(juniper::GraphQLEnum)]
pub enum TemplateSourceUnit {
    Identifier,
    Text,
    Tokens,
    UsdPerToken,
    UsdPerMillionTokens,
    ProviderNative,
}
impl From<compallm_core::provider_template::model_info::SourceUnit> for TemplateSourceUnit {
    fn from(v: compallm_core::provider_template::model_info::SourceUnit) -> Self {
        match v {
            compallm_core::provider_template::model_info::SourceUnit::Identifier => {
                Self::Identifier
            }
            compallm_core::provider_template::model_info::SourceUnit::Text => Self::Text,
            compallm_core::provider_template::model_info::SourceUnit::Tokens => Self::Tokens,
            compallm_core::provider_template::model_info::SourceUnit::UsdPerToken => {
                Self::UsdPerToken
            }
            compallm_core::provider_template::model_info::SourceUnit::UsdPerMillionTokens => {
                Self::UsdPerMillionTokens
            }
            compallm_core::provider_template::model_info::SourceUnit::ProviderNative => {
                Self::ProviderNative
            }
        }
    }
}
