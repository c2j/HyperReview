fn main() {
    // Set DATABASE_URL for sqlx offline mode (in-memory for compile-time checks)
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    }
}
