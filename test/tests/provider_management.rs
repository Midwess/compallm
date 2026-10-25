use compallm_core::{
    persistent::{
        DbContext,
        provider::{ProviderRepository, ProviderRepositoryError},
    },
    provider::{
        ApiProtocol, ManagementService, NewProvider, Provider, ProviderApiKey,
        ProviderManagementError, ProviderUpdate,
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

    async fn insert(&self, provider: NewProvider) -> Result<Provider, ProviderRepositoryError> {
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
        state.stored_api_key = Some(provider.api_key.into_storage_value());
        let provider = Provider {
            id: provider.id,
            organization_id: provider.organization_id,
            name: provider.name,
            url: provider.url,
            api_protocol: provider.api_protocol,
        };
        state.providers.push(provider.clone());
        Ok(provider)
    }

    async fn update(
        &self,
        organization_id: u64,
        provider_id: u64,
        update: ProviderUpdate,
    ) -> Result<Provider, ProviderRepositoryError> {
        let context = DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state
            .writes_in_transaction
            .push(context.postgres().is_transaction());
        if let Some(api_key) = update.api_key {
            state.stored_api_key = Some(api_key.into_storage_value());
        }
        let provider = state
            .providers
            .iter_mut()
            .find(|provider| {
                provider.id == provider_id && provider.organization_id == organization_id
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
    ) -> Result<(), ProviderRepositoryError> {
        let context = DbContext::current()?;
        let mut state = self.0.lock().unwrap();
        state
            .writes_in_transaction
            .push(context.postgres().is_transaction());
        let before = state.providers.len();
        state.providers.retain(|provider| {
            provider.id != provider_id || provider.organization_id != organization_id
        });
        if state.providers.len() == before {
            return Err(ProviderRepositoryError::NotFound);
        }
        Ok(())
    }
}

struct Fixture {
    db: &'static DbContext,
    database: Arc<sea_orm::DatabaseConnection>,
    state: Arc<Mutex<State>>,
    service: ManagementService,
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
            service: ManagementService::new(db, providers),
        }
    }
}

#[tokio::test]
async fn create_normalizes_input_generates_an_id_and_uses_a_transaction() {
    let fixture = Fixture::new(State::default());
    let provider = fixture
        .service
        .create(
            84,
            " Primary ",
            " https://provider.example/v1 ",
            ProviderApiKey::new("protected-key").unwrap(),
            ApiProtocol::OpenaiCompatible,
        )
        .await
        .unwrap();

    assert!(provider.id > 0);
    assert_eq!(provider.organization_id, 84);
    assert_eq!(provider.name, "Primary");
    assert_eq!(provider.url, "https://provider.example/v1");
    let state = fixture.state.lock().unwrap();
    assert_eq!(state.stored_api_key.as_deref(), Some("protected-key"));
    assert_eq!(state.writes_in_transaction, [true]);
    drop(state);
    let log = format!(
        "{:?}",
        fixture.database.as_ref().clone().into_transaction_log()
    );
    assert!(log.contains("BEGIN"));
    assert!(log.contains("COMMIT"));
}

#[tokio::test]
async fn invalid_create_or_update_input_does_not_open_a_transaction() {
    let fixture = Fixture::new(State::default());
    assert!(matches!(
        fixture
            .service
            .create(
                84,
                "  ",
                "https://provider.example/v1",
                ProviderApiKey::new("protected-key").unwrap(),
                ApiProtocol::Anthropic,
            )
            .await,
        Err(ProviderManagementError::InvalidName)
    ));
    assert!(matches!(
        fixture
            .service
            .update(84, 100, "Primary", "  ", None, ApiProtocol::Openai)
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
async fn update_and_delete_are_organization_scoped_transactional_workflows() {
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
    let provider = fixture
        .service
        .update(
            84,
            100,
            " Renamed ",
            " https://provider.example/v2 ",
            None,
            ApiProtocol::Anthropic,
        )
        .await
        .unwrap();
    assert_eq!(provider.name, "Renamed");
    assert_eq!(
        fixture.state.lock().unwrap().stored_api_key.as_deref(),
        Some("old-protected-key")
    );
    fixture.service.delete(84, 100).await.unwrap();
    let state = fixture.state.lock().unwrap();
    assert!(state.providers.is_empty());
    assert_eq!(state.writes_in_transaction, [true, true]);
}

#[tokio::test]
async fn repository_failure_rolls_back_and_preserves_the_typed_error() {
    let fixture = Fixture::new(State {
        fail_insert: true,
        ..State::default()
    });
    let error = fixture
        .service
        .create(
            84,
            "Primary",
            "https://provider.example/v1",
            ProviderApiKey::new("protected-key").unwrap(),
            ApiProtocol::Openai,
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
