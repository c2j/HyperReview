use hyperreview_lib::git::diff::DiffEngine;
use git2::Repository;

fn main() {
    env_logger::init();
    
    println!("Testing HyperReview diff fix...");
    
    let repo = match Repository::open("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return;
        }
    };
    
    // Debug: Check repository workdir
    if let Some(workdir) = repo.workdir() {
        println!("Repository workdir: {:?}", workdir);
        if let Some(workdir_str) = workdir.to_str() {
            println!("Workdir string: {}", workdir_str);
            println!("Workdir ends with slash: {}", workdir_str.ends_with('/'));
        }
    }
    
    let diff_engine = DiffEngine::new(repo);
    
    // Test both relative and absolute paths
    let test_cases = vec![
        ("frontend/api/types/checklist.ts", "relative path"),
        ("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/frontend/api/types/checklist.ts", "absolute path"),
        ("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/frontend/api/types/checklist.ts", "absolute path (with debug)"),
    ];
    
    for (file_path, description) in test_cases {
        println!("\n=== Testing {}: {} ===", description, file_path);
        
        // Set RUST_LOG for this specific test
        std::env::set_var("RUST_LOG", "hyperreview_lib::git::diff=debug");
        
        match diff_engine.compute_file_diff(
            file_path,
            Some("origin/main"),
            Some("origin/feature-merge-new-frontend/new")
        ) {
            Ok(lines) => {
                println!("SUCCESS: Found {} diff lines", lines.len());
                if lines.len() > 0 {
                    println!("First few lines:");
                    for (i, line) in lines.iter().take(3).enumerate() {
                        println!("  {}: {:?} - {}", i + 1, line.line_type, line.content.chars().take(50).collect::<String>());
                    }
                }
            },
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
        
        // Clear RUST_LOG after this test
        std::env::remove_var("RUST_LOG");
    }
}