use sqlx::postgres::PgPoolOptions;
use tracing::error;

const MAX_POOL_RETRIES: u8 = 3;

pub async fn get_connection_pool(identifier: &str) -> sqlx::PgPool {
    let conn_str = get_connection_string_from_identifier(identifier);

    let mut last_error = None;
    let mut attempt: u8 = 0;
    while attempt < MAX_POOL_RETRIES {
        attempt += 1;
        match connect_to_pool(&conn_str).await {
            Ok(value) => {
                return value;
            }
            Err(e) => {
                error!("Failed to connect to '{identifier}': {e}");
                last_error = Some(e);
            }
        };
    }

    let err = last_error.expect("Loop finished without running error branch");
    panic!("Fatal: Max connection attempts ({MAX_POOL_RETRIES}) reached for '{identifier}': {err}")
}

fn get_connection_string_from_identifier(identifier: &str) -> String {
    let conn_str =
        std::env::var(identifier).unwrap_or_else(|e| panic!("Missing env var '{identifier}: {e}"));
    conn_str
}

async fn connect_to_pool(conn_str: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await
}
