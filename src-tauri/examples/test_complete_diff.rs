use hyperreview_lib::git::complete_diff::CompleteDiffEngine;
use git2::Repository;

fn main() {
    env_logger::init();
    
    println!("Testing HyperReview Complete Diff Engine...");
    
    let repo = match Repository::open("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return;
        }
    };
    
    let diff_engine = CompleteDiffEngine::new(repo);
    
    // Test complete diff
    let test_file = "frontend/api/types/checklist.ts";
    let old_commit = "origin/main";
    let new_commit = "origin/feature-merge-new-frontend/new";
    
    println!("\n=== Testing Complete Diff: {} ===", test_file);
    println!("Old commit: {}", old_commit);
    println!("New commit: {}", new_commit);
    
    match diff_engine.compute_complete_diff(test_file, old_commit, new_commit) {
        Ok(lines) => {
            println!("SUCCESS: Found {} complete diff lines", lines.len());
            
            // Show statistics
            let added = lines.iter().filter(|l| matches!(l.line_type, hyperreview_lib::models::DiffLineType::Added)).count();
            let removed = lines.iter().filter(|l| matches!(l.line_type, hyperreview_lib::models::DiffLineType::Removed)).count();
            let context = lines.iter().filter(|l| matches!(l.line_type, hyperreview_lib::models::DiffLineType::Context)).count();
            
            println!("Statistics:");
            println!("  Added lines: {}", added);
            println!("  Removed lines: {}", removed);
            println!("  Context lines: {}", context);
            
            // Show first few lines as sample
            println!("\nFirst 10 lines:");
            for (i, line) in lines.iter().take(10).enumerate() {
                let line_num_info = format!("old:{:?} new:{:?}", line.old_line_number, line.new_line_number);
                println!("  {:2}: [{:20}] {} - {}", 
                    i + 1, 
                    line_num_info,
                    match line.line_type {
                        hyperreview_lib::models::DiffLineType::Added => "+",
                        hyperreview_lib::models::DiffLineType::Removed => "-",
                        hyperreview_lib::models::DiffLineType::Context => " ",
                        _ => "?",
                    },
                    line.content.chars().take(60).collect::<String>()
                );
            }
            
            // Verify line number consistency
            let mut max_old_line = 0u32;
            let mut max_new_line = 0u32;
            for line in &lines {
                if let Some(old_num) = line.old_line_number {
                    max_old_line = max_old_line.max(old_num);
                }
                if let Some(new_num) = line.new_line_number {
                    max_new_line = max_new_line.max(new_num);
                }
            }
            println!("\nLine number verification:");
            println!("  Max old line number: {}", max_old_line);
            println!("  Max new line number: {}", max_new_line);
            
            // Check that we have a complete new file view
            let has_complete_new_file = lines.iter().all(|l| l.new_line_number.is_some() || matches!(l.line_type, hyperreview_lib::models::DiffLineType::Removed));
            println!("  Complete new file view: {}", has_complete_new_file);
        },
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
}