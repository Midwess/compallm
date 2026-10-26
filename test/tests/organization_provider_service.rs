use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository, ProviderRepositoryError},
    },
    provider::{
        ApiProtocol, OrganizationProviderService, Provider, ProviderApiKey, ProviderManagementError,
    },
};
use compallm_postgres::postgres_connection::PostgresConnection;
use sea_orm::{DbBackend, MockDatabase};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct State {
    providers: Vec<Provider>,
    stored_api_key: Option<String>,
    find_limit: Option<u64>,
    writes_in_transaction: Vec<bool>,
    fail_insert: bool,
}

struct Repository(Arc<Mutex<State>>);

#[async_trait::async_trait]
impl ProviderRepository for Repository {
    async fn find_all(
        &self,
        organization_id: u64,
        limit: u64,
    ) -> Result<Vec<Provider>, ProviderRepositoryError> {
        DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state.find_limit = Some(limit);
        Ok(state
            .providers
            .iter()
            .filter(|provider| provider.organization_id == organization_id)
            .take(limit as usize)
            .cloned()
            .collect())
    }

    async fn insert(
        &self,
        provider: Provider,
        api_key: ProviderApiKey,
    ) -> Result<Provider, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state
            .writes_in_transaction
            .push(context.postgres().is_transaction());
        if state.fail_insert {
            return Err(ProviderRepositoryError::database(anyhow::anyhow!(
                "insert failed"
            )));
        }
        state.stored_api_key = Some(api_key.into_storage_value());
        state.providers.push(provider.clone());
        Ok(provider)
    }

    async fn update(
        &self,
        update: Provider,
        api_key: Option<ProviderApiKey>,
    ) -> Result<Provider, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state
            .writes_in_transaction
            .push(context.postgres().is_transaction());
        if let Some(api_key) = api_key {
            state.stored_api_key = Some(api_key.into_storage_value());
        }
        let provider = state
            .providers
            .iter_mut()
            .find(|provider| {
                provider.id == update.id && provider.organization_id == update.organization_id
            })
            .ok_or(ProviderRepositoryError::NotFound)?;
        provider.name = update.name;
        provider.url = update.url;
        provider.api_protocol = update.api_protocol;
        Ok(provider.clone())
    }

    async fn delete(
        &self,
        organization_id: u64,
        provider_id: u64,
    ) -> Result<Provider, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state
            .writes_in_transaction
            .push(context.postgres().is_transaction());
        let index = state
            .providers
            .iter()
            .position(|provider| {
                provider.id == provider_id && provider.organization_id == organization_id
            })
            .ok_or(ProviderRepositoryError::NotFound)?;
        Ok(state.providers.remove(index))
    }
}

struct Fixture {
    db: &'static DbContext,
    database: Arc<sea_orm::DatabaseConnection>,
    state: Arc<Mutex<State>>,
    service: OrganizationProviderService,
}

impl Fixture {
    fn new(state: State) -> Self {
        match hero::distributed_id::init_scoped_id_generator("provider-management-tests") {
            Ok(())
            | Err(hero::distributed_id::generator::DistributedIdError::AlreadyInitialized) => {}
            Err(error) => panic!("{error}"),
        }
        let database = Arc::new(MockDatabase::new(DbBackend::Postgres).into_connection());
        let db = Box::leak(Box::new(DbContext::new(Arc::new(PostgresConnection::new(
            database.clone(),
        )))));
        let state = Arc::new(Mutex::new(state));
        let providers = Box::leak(Box::new(
            Box::new(Repository(state.clone())) as Box<dyn ProviderRepository>
        ));
        Self {
            db,
            database,
            state,
            service: OrganizationProviderService::new(db, providers),
        }
    }
}

#[tokio::test]
async fn protocol_specific_additions_generate_ids_and_use_transactions() {
    let fixture = Fixture::new(State::default());
    let openai = fixture
        .service
        .add_openai_api(
            84,
            " Primary ",
            " https://provider.example/v1 ",
            ProviderApiKey::new("protected-key").unwrap(),
        )
        .await
        .unwrap();
    let anthropic = fixture
        .service
        .add_anthropic_api(
            84,
            "Anthropic",
            "https://anthropic.example",
            ProviderApiKey::new("protected-key").unwrap(),
        )
        .await
        .unwrap();
    let compatible = fixture
        .service
        .add_openai_compatible_api(
            84,
            "Compatible",
            "https://compatible.example/v1",
            ProviderApiKey::new("protected-key").unwrap(),
        )
        .await
        .unwrap();

    assert!(openai.id > 0);
    assert_eq!(openai.organization_id, 84);
    assert_eq!(openai.name, "Primary");
    assert_eq!(openai.url, "https://provider.example/v1");
    assert_eq!(openai.api_protocol, ApiProtocol::Openai);
    assert_eq!(anthropic.api_protocol, ApiProtocol::Anthropic);
    assert_eq!(compatible.api_protocol, ApiProtocol::OpenaiCompatible);
    let state = fixture.state.lock().unwrap();
    assert_eq!(state.stored_api_key.as_deref(), Some("protected-key"));
    assert_eq!(state.writes_in_transaction, [true, true, true]);
    drop(state);
    let log = format!(
        "{:?}",
        fixture.database.as_ref().clone().into_transaction_log()
    );
    assert!(log.contains("BEGIN"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn invalid_add_input_does_not_open_a_transaction() {
    let fixture = Fixture::new(State::default());
    assert!(matches!(
        fixture
            .service
            .add_anthropic_api(
                84,
                "  ",
                "https://provider.example/v1",
                ProviderApiKey::new("protected-key").unwrap(),
            )
            .await,
        Err(ProviderManagementError::InvalidName)
    ));
    assert!(matches!(
        fixture
            .service
            .add_openai_api(
                84,
                "Primary",
                "  ",
                ProviderApiKey::new("protected-key").unwrap(),
            )
            .await,
        Err(ProviderManagementError::InvalidUrl)
    ));
    assert!(
        fixture
            .database
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn find_all_uses_the_entry_point_scope_and_passes_the_required_limit() {
    let fixture = Fixture::new(State {
        providers: vec![Provider {
            id: 100,
            organization_id: 84,
            name: "Primary".into(),
            url: "https://provider.example/v1".into(),
            api_protocol: ApiProtocol::Openai,
        }],
        ..State::default()
    });
    let providers = fixture
        .db
        .scope(fixture.service.find_all(84, 32))
        .await
        .unwrap();
    assert_eq!(providers.len(), 1);
    assert_eq!(fixture.state.lock().unwrap().find_limit, Some(32));
    assert!(
        fixture
            .database
            .as_ref()
            .clone()
            .into_transaction_log()
            .is_empty()
    );
}

#[tokio::test]
async fn delete_is_an_organization_scoped_transactional_workflow() {
    let fixture = Fixture::new(State {
        providers: vec![Provider {
            id: 100,
            organization_id: 84,
            name: "Primary".into(),
            url: "https://provider.example/v1".into(),
            api_protocol: ApiProtocol::Openai,
        }],
        stored_api_key: Some("old-protected-key".into()),
        ..State::default()
    });
    let deleted = fixture.service.delete(84, 100).await.unwrap();
    assert_eq!(deleted.id, 100);
    let state = fixture.state.lock().unwrap();
    assert!(state.providers.is_empty());
    assert_eq!(state.writes_in_transaction, [true]);
}

#[tokio::test]
async fn repository_failure_rolls_back_and_preserves_the_typed_error() {
    let fixture = Fixture::new(State {
        fail_insert: true,
        ..State::default()
    });
    let error = fixture
        .service
        .add_openai_api(
            84,
            "Primary",
            "https://provider.example/v1",
            ProviderApiKey::new("protected-key").unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ProviderManagementError::Persistence(ProviderRepositoryError::Database(_))
    ));
    let log = format!(
        "{:?}",
        fixture.database.as_ref().clone().into_transaction_log()
    );
    assert!(log.contains("ROLLBACK"));
    assert!(!log.contains("COMMIT"));
}
