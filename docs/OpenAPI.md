# HyperReview REST API Specification

This document outlines the API contract for the web-based version of HyperReview (Client/Server architecture).
The frontend interacts with these endpoints when running in browser mode (connected to CodeArts or a standalone backend).

**Base URL**: `/api/v1`

## 1. Authentication

All requests must include the Authentication header:
`Authorization: Bearer <access_token>`

## 2. Endpoints

### 2.1 Repositories

#### Get Recent Repositories
`GET /recent-repos`
Returns a list of repositories the user has recently accessed.
**Response**: `200 OK`
```json
[
  {
    "path": "group/project-name",
    "branch": "feature/branch-a",
    "lastOpened": "10 mins ago"
  }
]
```

#### Get Branches
`GET /repos/{repoId}/branches`
**Response**: `200 OK`
```json
[
  { "name": "master", "isCurrent": false },
  { "name": "feature/login", "isCurrent": true }
]
```

### 2.2 Tasks (Pull Requests)

#### Get Task List
`GET /tasks`
**Query Parameters**:
- `type`: `pending` | `watched`
**Response**: `200 OK`
```json
[
  {
    "id": "PR-2877",
    "title": "Fix payment retry logic",
    "status": "active",
    "unreadCount": 0
  }
]
```

### 2.3 Review & Diff

#### Get File Diff
`GET /diff`
Computes the diff between base and head for a specific file.
**Query Parameters**:
- `fileId`: string (File path)
- `base`: string (Base commit/branch)
- `head`: string (Head commit/branch)
**Response**: `200 OK`
```json
[
  {
    "oldLineNumber": 10,
    "newLineNumber": 10,
    "content": "public void init() {",
    "type": "context"
  },
  {
    "newLineNumber": 11,
    "content": "  log.info(\"Initializing...\");",
    "type": "added",
    "severity": "INFO"
  }
]
```

#### Get Blame Info
`GET /blame`
**Query Parameters**:
- `fileId`: string
**Response**: `200 OK`
```json
{
  "author": "alice",
  "time": "2024-01-01 12:00",
  "prName": "PR-123",
  "comment": "Initial commit"
}
```

### 2.4 Analytics & Metadata

#### Get Review Statistics
`GET /reviews/stats`
**Response**: `200 OK`
```json
{
  "reviewedCount": 10,
  "totalCount": 20,
  "severeCount": 1,
  "warningCount": 5,
  "pendingCount": 0,
  "estimatedTime": "15m"
}
```

#### Get Heatmap
`GET /insights/heatmap`
**Response**: `200 OK`
```json
[
  { "id": "module-a", "name": "Auth", "impact": "high" }
]
```

#### Get Quality Gates
`GET /ci/quality-gates`
**Response**: `200 OK`
```json
[
  { "id": "unit-tests", "name": "Unit Tests", "status": "passed", "message": "100%" }
]
```

### 2.5 Resources

#### Get Tags
`GET /config/tags`
Returns available quick tags for review comments.

#### Get Command Palette Items
`GET /commands/search`
Returns searchable items (files, symbols).

#### Get Review Templates
`GET /reviews/templates`
Returns predefined comment templates.
