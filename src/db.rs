use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::Cli;

pub async fn init(args: &Cli) -> Pool<SqliteConnectionManager> {
    let manager = r2d2_sqlite::SqliteConnectionManager::file(&args.file);
    let pool = r2d2::Pool::new(manager).unwrap();

    pool.get()
        .unwrap()
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS records (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            ) WITHOUT ROWID;
            "#,
            [],
        )
        .unwrap();

    pool
}
