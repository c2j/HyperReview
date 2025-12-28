use crate::models::task::{TaskItem, TaskSeverity};
use anyhow::Result;

pub fn parse_task_text(text: &str) -> Result<Vec<TaskItem>> {
    let mut items = Vec::new();
    
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (file_path, comment) = if let Some((f, c)) = line.split_once('\t') {
            (f.trim(), Some(c.trim()))
        } else {
            // Also try double space as separator
            if let Some(idx) = line.find("  ") {
                let (f, c) = line.split_at(idx);
                (f.trim(), Some(c.trim()))
            } else {
                (line, None)
            }
        };

        if file_path.is_empty() {
            continue;
        }

        items.push(TaskItem {
            file: file_path.to_string(),
            line_range: None,
            preset_comment: comment.map(|s| s.to_string()),
            severity: None,
            tags: Vec::new(),
            reviewed: false,
            comments: Vec::new(),
        });
    }
    
    Ok(items)
}
