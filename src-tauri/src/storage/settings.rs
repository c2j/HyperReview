/// Settings persistence module
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params};
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub font_size: Option<i32>,
    pub enable_ligatures: Option<bool>,
    pub enable_vim: Option<bool>,
}

pub struct SettingsPersistence {
    conn: Connection,
}

impl SettingsPersistence {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    fn ensure_table_exists(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<UserSettings, Box<dyn std::error::Error>> {
        self.ensure_table_exists()?;

        let mut stmt = self.conn.prepare(
            "SELECT key, value FROM user_settings"
        )?;

        let mut settings = UserSettings {
            font_size: None,
            enable_ligatures: None,
            enable_vim: None,
        };

        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((key, value))
        })?;

        for row in rows {
            if let Ok((key, value)) = row {
                match key.as_str() {
                    "fontSize" => {
                        if let Ok(size) = value.parse::<i32>() {
                            settings.font_size = Some(size);
                        }
                    }
                    "enableLigatures" => {
                        settings.enable_ligatures = Some(value == "true");
                    }
                    "enableVim" => {
                        settings.enable_vim = Some(value == "true");
                    }
                    _ => {}
                }
            }
        }

        Ok(settings)
    }

    pub fn save_setting(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_table_exists()?;

        self.conn.execute(
            "INSERT OR REPLACE INTO user_settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;

        info!("Saved setting: {} = {}", key, value);
        Ok(())
    }
}
