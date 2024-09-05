use crate::connection::get_connection_url;

#[test]
fn test_connection_url() {
    let url = get_connection_url();
    assert!(
        url.contains("mysql") || url.contains("postgres"),
        "The URL does not contain 'mysql' or 'postgres'"
    );
}
