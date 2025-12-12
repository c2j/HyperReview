// Simple demo of HyperReview core functionality
// This runs without GPUI dependencies

use hyperreview::app::AppState;
use hyperreview::services::GitService;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🚀 HyperReview Demo - Milestone 1: The Reader\n");

    // Initialize app state
    let app_state = Arc::new(AppState::new(&()));
    println!("✅ AppState initialized");

    // Initialize git service
    let git_service = GitService::new();
    println!("✅ GitService initialized");

    // Try to open current directory as a git repo
    match git_service.open_repository(std::env::current_dir()?.as_path()).await {
        Ok(repo) => {
            println!("✅ Opened repository: {:?}", repo.path());

            // Try to compute diff between HEAD~1 and HEAD
            match git_service.compute_diff(&repo, "HEAD~1", "HEAD").await {
                Ok(diff) => {
                    println!("✅ Computed diff with {} files", diff.deltas().len());
                    println!("📄 Diff preview:");
                    println!("{}", "=".repeat(60));
                }
                Err(e) => {
                    println!("⚠️  Could not compute diff: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Could not open repository: {}", e);
            println!("\nTip: Run this from a git repository with at least 2 commits");
        }
    }

    println!("\n✨ Demo complete!");
    println!("\nNext steps:");
    println!("  - Implement Milestone 2: GitHub OAuth integration");
    println!("  - Add PR inbox functionality");
    println!("  - Fix GPUI dependencies for UI");

    Ok(())
}
