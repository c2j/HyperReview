-- HyperReview Initial Schema
-- Following Constitution II (Offline-First) requirements

-- Account/Auth table
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,  -- provider:username
    provider TEXT NOT NULL,  -- 'github' | 'gitlab'
    username TEXT NOT NULL,
    avatar_url TEXT,
    access_token_encrypted BLOB,
    refresh_token_encrypted BLOB,
    token_expires_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id TEXT PRIMARY KEY,  -- provider:owner/name
    provider TEXT NOT NULL,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    local_clone_path TEXT,
    last_synced_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Pull Requests table
CREATE TABLE IF NOT EXISTS pull_requests (
    id TEXT PRIMARY KEY,  -- provider:owner/repo#number
    repo_id TEXT NOT NULL REFERENCES repositories(id),
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    author_username TEXT NOT NULL,
    author_avatar_url TEXT,
    status TEXT NOT NULL,  -- 'open' | 'closed' | 'merged'
    head_sha TEXT NOT NULL,
    base_sha TEXT NOT NULL,
    ci_status TEXT,  -- 'pending' | 'success' | 'failure'
    body_markdown TEXT,
    is_read BOOLEAN DEFAULT 0,
    is_archived BOOLEAN DEFAULT 0,
    updated_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL,
    last_synced_at DATETIME
);

-- Comments table (including pending offline)
CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY,  -- local UUID or provider ID
    pr_id TEXT NOT NULL REFERENCES pull_requests(id),
    remote_id TEXT,  -- NULL if not yet synced
    file_path TEXT,
    line_number INTEGER,
    content TEXT NOT NULL,
    author_username TEXT NOT NULL,
    sync_status TEXT NOT NULL DEFAULT 'synced',  -- 'pending' | 'synced' | 'failed'
    sync_error TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Reviews table
CREATE TABLE IF NOT EXISTS reviews (
    id TEXT PRIMARY KEY,
    pr_id TEXT NOT NULL REFERENCES pull_requests(id),
    remote_id TEXT,
    decision TEXT NOT NULL,  -- 'approve' | 'request_changes' | 'comment'
    body TEXT,
    sync_status TEXT NOT NULL DEFAULT 'synced',
    submitted_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_prs_repo ON pull_requests(repo_id);
CREATE INDEX IF NOT EXISTS idx_prs_status ON pull_requests(status);
CREATE INDEX IF NOT EXISTS idx_prs_status_updated ON pull_requests(status, updated_at DESC) WHERE is_archived = 0;
CREATE INDEX IF NOT EXISTS idx_comments_pr ON comments(pr_id);
CREATE INDEX IF NOT EXISTS idx_comments_sync ON comments(sync_status) WHERE sync_status != 'synced';
CREATE INDEX IF NOT EXISTS idx_reviews_sync ON reviews(sync_status) WHERE sync_status != 'synced';
CREATE UNIQUE INDEX IF NOT EXISTS idx_prs_repo_number ON pull_requests(repo_id, number);
CREATE INDEX IF NOT EXISTS idx_comments_pr_line ON comments(pr_id, file_path, line_number);
