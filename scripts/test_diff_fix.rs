use hyperreview_lib::git::diff::DiffEngine;
use git2::Repository;

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Open the repository
    let repo = Repository::open("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview").expect("Failed to open repository");
    let diff_engine = DiffEngine::new(repo);
    
    // Test the diff computation with both relative and absolute paths
    let relative_path = "frontend/api/types/checklist.ts";
    let absolute_path = "/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/frontend/api/types/checklist.ts";
    
    println!("Testing diff with relative path: {}", relative_path);
    match diff_engine.compute_file_diff(relative_path, Some("origin/main"), Some("origin/feature-merge-new-frontend/new")) {
        Ok(lines) => println!("Relative path: Found {} diff lines", lines.len()),
        Err(e) => println!("Relative path error: {:?}", e),
    }
    
    println!("Testing diff with absolute path: {}", absolute_path);
    match diff_engine.compute_file_diff(absolute_path, Some("origin/main"), Some("origin/feature-merge-new-frontend/new")) {
        Ok(lines) => println!("Absolute path: Found {} diff lines", lines.len()),
        Err(e) => println!("Absolute path error: {:?}", e),
    }
}