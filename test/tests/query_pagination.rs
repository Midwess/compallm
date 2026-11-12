use compallm_query::{Cursor, Pagination};

#[test]
fn pages_use_the_configured_cap_in_both_directions() {
    let forward = Pagination {
        first: Some(100),
        after: Some(Cursor::from("next")),
        ..Default::default()
    };
    let backward = Pagination {
        last: Some(100),
        before: Some(Cursor::from("previous")),
        ..Default::default()
    };
    assert_eq!(forward.page_size(30).unwrap(), 30);
    assert_eq!(backward.page_size(30).unwrap(), 30);
    assert_eq!(forward.page_size(200).unwrap(), 100);
}

#[test]
fn omitted_page_size_uses_the_configured_limit_and_explicit_zero_stays_empty() {
    assert_eq!(
        Pagination::default()
            .page_size(Pagination::DEFAULT_MAX_ROWS)
            .unwrap(),
        30
    );
    assert_eq!(Pagination::default().page_size(75).unwrap(), 75);
    let empty = Pagination {
        first: Some(0),
        ..Default::default()
    };
    assert_eq!(empty.page_size(30).unwrap(), 0);
}
