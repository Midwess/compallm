use compallm_core::repository::Queryable;
use modql::filter::ListOptions;
use std::sync::Mutex;

struct Rows {
    values: Vec<u64>,
    options: Mutex<Option<ListOptions>>,
}

#[async_trait::async_trait]
impl Queryable<compallm_core::repository::user::UserFilter> for Rows {
    type Entity = u64;
    type Error = ();

    async fn list(
        &self,
        _: compallm_core::repository::user::UserFilter,
        options: ListOptions,
    ) -> Result<Vec<u64>, ()> {
        let rows = self
            .values
            .iter()
            .copied()
            .skip(options.offset.unwrap_or(0) as usize)
            .take(options.limit.unwrap_or(100) as usize)
            .collect();
        *self.options.lock().unwrap() = Some(options);
        Ok(rows)
    }
}

#[tokio::test]
async fn one_limits_the_list_and_preserves_offset_and_ordering() {
    let repository = Rows {
        values: vec![42, 43, 44],
        options: Mutex::new(None),
    };
    let filter = compallm_core::repository::user::UserFilter::default();
    let first = repository
        .one(
            filter,
            ListOptions::from_offset_limit(1, 20).with_order_bys("id"),
        )
        .await
        .unwrap();
    assert_eq!(first, Some(43));
    let options = repository.options.lock().unwrap();
    let options = options.as_ref().unwrap();
    assert_eq!(options.limit, Some(1));
    assert_eq!(options.offset, Some(1));
    assert!(
        matches!(options.order_bys.as_ref().unwrap().into_iter().next(), Some(modql::filter::OrderBy::Asc(column)) if column == "id")
    );
}

#[tokio::test]
async fn one_returns_none_for_an_empty_list() {
    let repository = Rows {
        values: vec![],
        options: Mutex::new(None),
    };
    assert_eq!(
        repository
            .one(
                compallm_core::repository::user::UserFilter::default(),
                ListOptions::default(),
            )
            .await
            .unwrap(),
        None
    );
}
