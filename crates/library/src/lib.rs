use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

pub const CURRENT_SCHEMA_VERSION: u32 = 1;

pub fn open_or_create_db(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;
         PRAGMA journal_mode = WAL;",
    )?;

    let version: u32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    if version > CURRENT_SCHEMA_VERSION {
        anyhow::bail!(
            "Database version {} is newer than supported version {}. Refusing downgrade.",
            version,
            CURRENT_SCHEMA_VERSION
        );
    }

    if version < 1 {
        apply_migration_v1(&conn)?;
        conn.execute("PRAGMA user_version = 1;", [])?;
    }

    Ok(conn)
}

fn apply_migration_v1(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE items (
            id              INTEGER PRIMARY KEY,
            source_id       TEXT NOT NULL,
            source_item_id  TEXT NOT NULL,
            media_kind      TEXT NOT NULL CHECK (media_kind IN ('image','video')),
            title           TEXT,
            author          TEXT,
            source_url      TEXT,
            license         TEXT NOT NULL,
            attribution     TEXT NOT NULL,
            width           INTEGER,
            height          INTEGER,
            duration_ms     INTEGER,
            sha256          TEXT,
            file_path       TEXT,
            thumb_path      TEXT,
            blurhash        TEXT,
            palette_json    TEXT,
            added_at        INTEGER NOT NULL,
            favorite        INTEGER NOT NULL DEFAULT 0,
            UNIQUE (source_id, source_item_id)
        );
        CREATE UNIQUE INDEX items_sha ON items(sha256) WHERE sha256 IS NOT NULL;

        CREATE TABLE tags (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE);
        CREATE TABLE item_tags (
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (item_id, tag_id)
        );

        CREATE TABLE collections (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id INTEGER REFERENCES collections(id) ON DELETE CASCADE,
            position INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        );
        CREATE TABLE collection_items (
            collection_id INTEGER NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            position INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (collection_id, item_id)
        );

        CREATE TABLE downloads (
            id INTEGER PRIMARY KEY,
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            state TEXT NOT NULL CHECK (state IN ('queued','running','paused','done','failed','canceled')),
            bytes_done INTEGER NOT NULL DEFAULT 0,
            bytes_total INTEGER,
            error TEXT,
            attempts INTEGER NOT NULL DEFAULT 0,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE schedules (
            id INTEGER PRIMARY KEY,
            display_id TEXT,
            pool_kind TEXT NOT NULL CHECK (pool_kind IN ('all','favorites','collection','search')),
            pool_ref TEXT,
            trigger_kind TEXT NOT NULL CHECK (trigger_kind IN ('interval','login','wake','manual')),
            interval_seconds INTEGER,
            media_kinds TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE applied_history (
            id INTEGER PRIMARY KEY,
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            display_id TEXT,
            applied_at INTEGER NOT NULL
        );

        CREATE TABLE source_cache (
            key TEXT PRIMARY KEY,
            body BLOB NOT NULL,
            fetched_at INTEGER NOT NULL,
            ttl_seconds INTEGER NOT NULL
        );

        CREATE TABLE settings (key TEXT PRIMARY KEY, value_json TEXT NOT NULL);"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_migration() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;",
        )
        .unwrap();
        apply_migration_v1(&conn).unwrap();
        conn.execute("PRAGMA user_version = 1;", []).unwrap();

        let count: i64 = conn
            .query_row("SELECT count(*) FROM items", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
