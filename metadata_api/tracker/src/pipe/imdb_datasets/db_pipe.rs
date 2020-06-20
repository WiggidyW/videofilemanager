use sqlx::sqlite::SqliteConnection;

pub struct SqlitePipe<P> {
    rows_pipe: P,
    conn: SqliteConnection,
}