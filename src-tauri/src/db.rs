use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSession {
    pub app_name: String,
    pub window_title: String,
    pub category: String,
    pub start_time: u64,
    pub end_time: u64,
}

fn get_db_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let mut path = PathBuf::from(std::env::var("APPDATA").unwrap());
        path.push("deskflow/usage_data.db");
        path
    } else {
        let mut path = PathBuf::from(std::env::var("HOME").unwrap());
        path.push(".deskflow/usage_data.db");
        path
    }
}

pub fn init_db() -> Result<()> {
    let db_path = get_db_path();
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_usage (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL,
            window_title TEXT NOT NULL,
            category TEXT NOT NULL,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,
            date TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS productivity_scores (
            date TEXT PRIMARY KEY,
            score REAL NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn save_session_to_db(session: &AppSession) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    let date = chrono::NaiveDateTime::from_timestamp(session.start_time as i64, 0)
        .date()
        .to_string();

    conn.execute(
        "INSERT INTO app_usage (app_name, window_title, category, start_time, end_time, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            session.app_name,
            session.window_title,
            session.category,
            session.start_time,
            session.end_time,
            date
        ],
    )?;
    Ok(())
}

pub fn get_category_summary_today() -> Result<std::collections::HashMap<String, u64>> {
    let conn = Connection::open(get_db_path())?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut stmt = conn.prepare(
        "SELECT category, SUM(end_time - start_time) as total FROM app_usage WHERE date = ?1 GROUP BY category",
    )?;

    let mut map = std::collections::HashMap::new();

    let rows = stmt.query_map([today], |row| {
        let category: String = row.get(0)?;
        let total: u64 = row.get(1)?;
        Ok((category, total))
    })?;

    for row in rows {
        let (category, total) = row?;
        map.insert(category, total);
    }

    Ok(map)
}
