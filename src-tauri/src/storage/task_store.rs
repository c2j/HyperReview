use crate::models::task::LocalTask;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct TaskStore {
    base_path: PathBuf,
}

impl TaskStore {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let base_path = home.join(".hyperreview").join("local_tasks");
        
        if !base_path.exists() {
            fs::create_dir_all(&base_path)?;
        }
        
        Ok(Self { base_path })
    }

    pub fn save_task(&self, task: &LocalTask) -> Result<()> {
        let file_path = self.get_task_path(task.id);
        let json = serde_json::to_string_pretty(task)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn load_task(&self, task_id: Uuid) -> Result<LocalTask> {
        let file_path = self.get_task_path(task_id);
        let content = fs::read_to_string(file_path)?;
        let task = serde_json::from_str(&content)?;
        Ok(task)
    }

    pub fn delete_task(&self, task_id: Uuid) -> Result<()> {
        let file_path = self.get_task_path(task_id);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<LocalTask>> {
        let mut tasks = Vec::new();
        if self.base_path.exists() {
            for entry in fs::read_dir(&self.base_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(task) = serde_json::from_str::<LocalTask>(&content) {
                            tasks.push(task);
                        }
                    }
                }
            }
        }
        Ok(tasks)
    }

    fn get_task_path(&self, task_id: Uuid) -> PathBuf {
        self.base_path.join(format!("{}.json", task_id))
    }
}
