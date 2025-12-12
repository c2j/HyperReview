# GitHub API Contract: HyperReview MVP

**Feature**: 001-pr-review-mvp
**Date**: 2025-11-23
**API Version**: GitHub REST API v3 + GraphQL v4

## Authentication

### Device Flow (OAuth 2.0 Device Authorization Grant)

**Step 1: Request Device Code**

```http
POST https://github.com/login/device/code
Content-Type: application/json
Accept: application/json

{
  "client_id": "{GITHUB_CLIENT_ID}",
  "scope": "repo read:user"
}
```

**Response**:
```json
{
  "device_code": "3584d83530557fdd1f46af8289938c8ef79f9dc5",
  "user_code": "WDJB-MJHT",
  "verification_uri": "https://github.com/login/device",
  "expires_in": 900,
  "interval": 5
}
```

**Step 2: Poll for Token**

```http
POST https://github.com/login/oauth/access_token
Content-Type: application/json
Accept: application/json

{
  "client_id": "{GITHUB_CLIENT_ID}",
  "device_code": "{device_code}",
  "grant_type": "urn:ietf:params:oauth:grant-type:device_code"
}
```

**Success Response**:
```json
{
  "access_token": "gho_16C7e42F292c6912E7710c838347Ae178B4a",
  "token_type": "bearer",
  "scope": "repo,read:user"
}
```

**Polling Response (User Still Authorizing)**:
```json
{
  "error": "authorization_pending"
}
```

---

## User Endpoints

### Get Authenticated User

```http
GET https://api.github.com/user
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response**:
```json
{
  "login": "octocat",
  "id": 1,
  "avatar_url": "https://github.com/images/error/octocat_happy.gif",
  "name": "The Octocat"
}
```

**Mapping to Data Model**:
- `login` → Account.username
- `avatar_url` → Account.avatar_url

---

## Pull Request Endpoints

### List PRs for Review (Search API)

Find all PRs where user is reviewer or mentioned:

```http
GET https://api.github.com/search/issues?q=is:pr+is:open+review-requested:{username}+OR+mentions:{username}
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response**:
```json
{
  "total_count": 42,
  "items": [
    {
      "number": 123,
      "title": "Add new feature",
      "user": {
        "login": "author",
        "avatar_url": "https://..."
      },
      "repository_url": "https://api.github.com/repos/owner/repo",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-02T00:00:00Z",
      "pull_request": {
        "url": "https://api.github.com/repos/owner/repo/pulls/123"
      }
    }
  ]
}
```

**Mapping to Data Model**:
- `number` → PullRequest.number
- `title` → PullRequest.title
- `user.login` → PullRequest.author_username
- `user.avatar_url` → PullRequest.author_avatar_url
- `updated_at` → PullRequest.updated_at
- `created_at` → PullRequest.created_at
- Parse `repository_url` → PullRequest.repo_id

### Get Single PR

```http
GET https://api.github.com/repos/{owner}/{repo}/pulls/{pull_number}
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response**:
```json
{
  "number": 123,
  "title": "Add new feature",
  "state": "open",
  "body": "PR description in markdown...",
  "head": {
    "sha": "6dcb09b5b57875f334f61aebed695e2e4193db5e"
  },
  "base": {
    "sha": "c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c"
  },
  "user": {
    "login": "author",
    "avatar_url": "https://..."
  }
}
```

**Mapping to Data Model**:
- `state` → PullRequest.status (map: "open" → Open, "closed" → Closed, "merged" → Merged)
- `head.sha` → PullRequest.head_sha
- `base.sha` → PullRequest.base_sha
- `body` → PullRequest.body_markdown

### Get PR Statuses (CI)

```http
GET https://api.github.com/repos/{owner}/{repo}/commits/{sha}/status
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response**:
```json
{
  "state": "success",
  "statuses": [...]
}
```

**Mapping to Data Model**:
- `state` → PullRequest.ci_status (map: "success" → Success, "failure" → Failure, "pending" → Pending)

---

## Comment Endpoints

### Create PR Review Comment (Inline)

```http
POST https://api.github.com/repos/{owner}/{repo}/pulls/{pull_number}/comments
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
Content-Type: application/json

{
  "body": "Comment text here",
  "commit_id": "{head_sha}",
  "path": "src/file.rs",
  "line": 10,
  "side": "RIGHT"
}
```

**Response**:
```json
{
  "id": 10,
  "body": "Comment text here",
  "path": "src/file.rs",
  "line": 10,
  "user": {...},
  "created_at": "2025-01-01T00:00:00Z"
}
```

**Mapping to Data Model**:
- Request: Comment.content → `body`, Comment.file_path → `path`, Comment.line_number → `line`
- Response: `id` → Comment.remote_id

### Reply to Comment Thread

```http
POST https://api.github.com/repos/{owner}/{repo}/pulls/{pull_number}/comments
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
Content-Type: application/json

{
  "body": "Reply text",
  "in_reply_to": {parent_comment_id}
}
```

### List PR Comments

```http
GET https://api.github.com/repos/{owner}/{repo}/pulls/{pull_number}/comments
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response**:
```json
[
  {
    "id": 10,
    "body": "Great point!",
    "path": "src/file.rs",
    "line": 10,
    "user": {
      "login": "reviewer",
      "avatar_url": "https://..."
    },
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
]
```

---

## Review Endpoints

### Submit PR Review

```http
POST https://api.github.com/repos/{owner}/{repo}/pulls/{pull_number}/reviews
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
Content-Type: application/json

{
  "body": "Overall review summary",
  "event": "APPROVE",
  "comments": [
    {
      "path": "src/file.rs",
      "line": 10,
      "body": "Inline comment"
    }
  ]
}
```

**Request Mapping**:
- Review.body → `body`
- Review.decision → `event` (map: Approve → "APPROVE", RequestChanges → "REQUEST_CHANGES", Comment → "COMMENT")
- Review.comment_ids → `comments` array

**Response**:
```json
{
  "id": 80,
  "state": "APPROVED",
  "submitted_at": "2025-01-01T00:00:00Z"
}
```

**Response Mapping**:
- `id` → Review.remote_id
- `submitted_at` → Review.submitted_at

---

## Repository Clone URL

For git2-rs shallow clone:

```http
GET https://api.github.com/repos/{owner}/{repo}
Authorization: Bearer {access_token}
Accept: application/vnd.github+json
```

**Response** (relevant fields):
```json
{
  "clone_url": "https://github.com/owner/repo.git",
  "ssh_url": "git@github.com:owner/repo.git"
}
```

**Clone with Token**:
```
https://{access_token}@github.com/{owner}/{repo}.git
```

---

## Rate Limiting

**Headers in Response**:
```
X-RateLimit-Limit: 5000
X-RateLimit-Remaining: 4999
X-RateLimit-Reset: 1372700873
```

**Handling**:
- Check `X-RateLimit-Remaining` before requests
- If 0, wait until `X-RateLimit-Reset` (Unix timestamp)
- Show user-friendly message: "GitHub rate limit reached. Resuming in X minutes."

---

## Error Responses

**401 Unauthorized** (Token Invalid/Expired):
```json
{
  "message": "Bad credentials",
  "documentation_url": "https://docs.github.com/rest"
}
```
→ Trigger re-authentication flow

**403 Forbidden** (Rate Limited):
```json
{
  "message": "API rate limit exceeded",
  "documentation_url": "https://docs.github.com/rest/overview/resources-in-the-rest-api#rate-limiting"
}
```
→ Queue request for retry after reset

**404 Not Found** (PR Deleted/Private):
```json
{
  "message": "Not Found"
}
```
→ Mark PR as stale, offer removal from inbox

**422 Unprocessable Entity** (Invalid Comment):
```json
{
  "message": "Validation Failed",
  "errors": [
    {
      "resource": "PullRequestReviewComment",
      "code": "missing_field",
      "field": "line"
    }
  ]
}
```
→ Comment.sync_status = Failed, show error to user

---

## Request/Response Mapping Summary

| Operation | FR | Endpoint | Request Entity | Response Entity |
|-----------|----|---------|--------------|--------------------|
| Authenticate | FR-001 | POST /login/device/code | - | Account.access_token |
| Get User | FR-001 | GET /user | - | Account |
| List PRs | FR-005,006 | GET /search/issues | - | List<PullRequest> |
| Get PR | FR-008 | GET /repos/.../pulls/{n} | - | PullRequest |
| Get CI | FR-008 | GET /repos/.../commits/{sha}/status | - | PullRequest.ci_status |
| Create Comment | FR-025 | POST /repos/.../pulls/{n}/comments | Comment | Comment.remote_id |
| Submit Review | FR-029 | POST /repos/.../pulls/{n}/reviews | Review | Review.remote_id |
