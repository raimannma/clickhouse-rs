#![cfg(feature = "test-util")]

use crate::SimpleRow;
use clickhouse::{Client, test};

#[tokio::test]
async fn stream_decodes_rows() {
    let mock = test::Mock::new();
    let client = Client::default().with_mock(&mock);
    let expected = vec![SimpleRow::new(1, "one"), SimpleRow::new(2, "two")];
    mock.add(test::handlers::provide(expected.clone()));

    let mut cursor = client
        .query("SELECT ?fields FROM events STREAM")
        .stream::<SimpleRow>()
        .unwrap();

    let mut actual = Vec::new();
    while let Some(row) = cursor.next().await.unwrap() {
        actual.push(row);
    }

    assert_eq!(actual, expected);
}
