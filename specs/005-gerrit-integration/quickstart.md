# Gerrit Code Review Integration - Quick Start Guide

## Overview

This guide will help you get started with the Gerrit Code Review integration for HyperReview, enabling offline batch code review operations with seamless synchronization to Gerrit servers.

## Prerequisites

Before you begin, ensure you have:
- HyperReview installed and running
- Access to a Gerrit server (version 3.6+ recommended)
- Gerrit HTTP password token (for authentication)
- Network connectivity to your Gerrit instance

## Step 1: Configure Gerrit Instance

### 1.1 Add New Gerrit Instance

Open HyperReview and navigate to Settings → Gerrit Integration:

```typescript
// Example: Configure a new Gerrit instance
const config = {
  name: "Production Gerrit",
  url: "https://gerrit.example.com",
  username: "your-username",
  password: "your-http-password-token", // Get from Gerrit settings
  makeActive: true
};

await gerritInstanceService.createInstance(config);
```

### 1.2 Test Connection

After configuration, test the connection:

```typescript
// Test connection to verify credentials and compatibility
const result = await gerritInstanceService.testConnection(instanceId);
console.log(`Gerrit version: ${result.gerritVersion}`);
console.log(`Supported features: ${result.supportedFeatures}`);
```

**Expected Response:**
- Connection status: "Connected"
- Gerrit version: 3.6.0 or higher
- Supported features: Basic Auth, Batch Comments, Review API

## Step 2: Import Gerrit Changes

### 2.1 Import by Change ID

Import a specific change for review:

```typescript
// Import change #12345 with diffs and comments
const change = await gerritChangeService.importChange("12345", {
  includeDiffs: true,
  includeComments: true
});

console.log(`Imported change: ${change.subject}`);
console.log(`Files: ${change.totalFiles}, Comments: ${change.remoteComments}`);
```

### 2.2 Search and Import Multiple Changes

Search for changes using Gerrit query syntax:

```typescript
// Search for open changes in payment project
const results = await gerritChangeService.searchChanges(
  "status:open project:payment",
  {
    limit: 10,
    includeDiffs: false, // Faster import for multiple changes
    includeComments: true
  }
);

// Import selected changes
for (const change of results.changes) {
  if (change.totalFiles > 50) { // Focus on large changes
    await gerritChangeService.importChange(change.changeId);
  }
}
```

### 2.3 Batch Import Large Changes

For enterprise environments with many changes:

```typescript
// Import multiple changes efficiently
const changeIds = ["12345", "12346", "12347"];
const result = await gerritChangeService.importMultiple(changeIds, {
  includeDiffs: true,
  includeComments: true
});

console.log(`Imported ${result.successful.length} changes`);
console.log(`Failed: ${result.failed.length}`);
```

## Step 3: Perform Offline Review

### 3.1 Review Files with Heatmaps

Navigate to the imported change and use HyperReview's advanced features:

```typescript
// Get change details for review
const change = await gerritChangeService.getChange(changeId);

// Navigate through files
for (const file of change.files) {
  if (file.reviewStatus === 'unreviewed') {
    console.log(`Reviewing: ${file.path}`);
    
    // Use heatmap analysis, line selection, annotations
    // All operations work offline
  }
}
```

### 3.2 Create Comments and Annotations

Add comments while reviewing files:

```typescript
// Create a comment on specific line
const comment = await gerritCommentService.createComment({
  changeId: "12345",
  filePath: "src/main/java/com/example/PaymentService.java",
  patchSetNumber: 3,
  line: 147,
  message: "Consider adding null check here to prevent NPE",
  severity: "warning"
});

console.log(`Created comment: ${comment.id}`);
```

### 3.3 Create Comment Threads

Reply to existing comments or create discussion threads:

```typescript
// Reply to an existing comment
const reply = await gerritCommentService.replyToComment(parentCommentId, {
  changeId: "12345",
  filePath: "src/main/java/com/example/PaymentService.java",
  patchSetNumber: 3,
  line: 147,
  message: "Good point! I'll add the null check.",
  severity: "info"
});
```

### 3.4 Review Progress Tracking

Track your review progress:

```typescript
// Get review progress
const progress = {
  totalFiles: change.totalFiles,
  reviewedFiles: change.reviewedFiles,
  localComments: change.localComments,
  completionPercentage: (change.reviewedFiles / change.totalFiles) * 100
};

console.log(`Review progress: ${progress.completionPercentage.toFixed(1)}%`);
```

## Step 4: Batch Operations

### 4.1 Prepare Review for Submission

Before pushing to Gerrit, prepare your review:

```typescript
// Prepare review with comments and scores
const reviewDraft = await gerritReviewService.prepareReview("12345", {
  autoIncludeComments: true,
  suggestedLabels: {
    "Code-Review": 2,  // +2 score
    "Verified": 1      // +1 score
  }
});

// Customize review message
reviewDraft.message = "LGTM! Added some minor suggestions for improvement.";
```

### 4.2 Submit Batch Review

Push all comments and review scores to Gerrit:

```typescript
// Submit complete review
const result = await gerritReviewService.submitReview({
  changeId: "12345",
  patchSetNumber: 3,
  message: reviewDraft.message,
  labels: reviewDraft.labels,
  commentIds: reviewDraft.commentIds,
  notify: "owner_reviewers", // Notify owner and reviewers
  draft: false
});

console.log(`Review submitted: ${result.reviewId}`);
console.log(`Comments pushed: ${result.submittedComments}`);
console.log(`Labels submitted:`, result.submittedLabels);
```

### 4.3 Handle Large Changes Efficiently

For changes with many files (>100), use optimized workflows:

```typescript
// Import large change without diffs initially
const change = await gerritChangeService.importChange("12345", {
  includeDiffs: false,  // Faster initial import
  includeComments: true
});

// Load diffs on-demand when reviewing specific files
const fileDiff = await gerritChangeService.getFileDiff(changeId, filePath);
```

## Step 5: Synchronization Management

### 5.1 Manual Synchronization

Sync changes with Gerrit server:

```typescript
// Sync specific changes
const syncResult = await gerritSyncService.syncChanges({
  changeIds: ["12345", "12346"],
  syncType: "full",
  conflictResolution: "prompt" // Ask user for conflicts
});

console.log(`Synced: ${syncResult.syncedChanges}, Conflicts: ${syncResult.conflictsDetected}`);
```

### 5.2 Handle Conflicts

When conflicts are detected between local and remote changes:

```typescript
// Get conflicts and resolve them
const conflicts = await gerritSyncService.getConflicts("12345");

for (const conflict of conflicts) {
  console.log(`Conflict: ${conflict.type} - ${conflict.description}`);
  
  // User chooses resolution strategy
  await gerritSyncService.resolveConflict(conflict.id, {
    strategy: "localWins", // or "remoteWins", "manual"
    manualResolution?: conflict.manualOptions
  });
}
```

### 5.3 Configure Auto-Sync

Set up automatic synchronization:

```typescript
// Configure sync settings
await gerritSyncService.configureSync({
  autoSync: true,
  syncInterval: 300, // 5 minutes
  conflictResolution: "auto",
  maxRetries: 3,
  batchSize: 10
});
```

## Step 6: Multi-Instance Support

### 6.1 Switch Between Instances

For environments with multiple Gerrit servers:

```typescript
// List all configured instances
const instances = await gerritInstanceService.getInstances(true);

// Switch to different instance
await gerritInstanceService.setActiveInstance("dev-gerrit-instance");

// Operations now use the active instance
const changes = await gerritChangeService.searchChanges("status:open");
```

### 6.2 Instance-Specific Operations

Perform operations on specific instances:

```typescript
// Search changes on specific instance
const devChanges = await gerritChangeService.searchChanges(
  "status:open project:payment", 
  { instanceId: "dev-gerrit-instance" }
);

const prodChanges = await gerritChangeService.searchChanges(
  "status:open project:payment", 
  { instanceId: "prod-gerrit-instance" }
);
```

## Step 7: Error Handling and Troubleshooting

### 7.1 Common Error Scenarios

Handle common integration errors:

```typescript
try {
  await gerritChangeService.importChange("12345");
} catch (error) {
  switch (error.code) {
    case 'AUTHENTICATION_FAILED':
      console.log('Check your Gerrit credentials');
      break;
    case 'CHANGE_NOT_FOUND':
      console.log('Change does not exist or you lack permission');
      break;
    case 'NETWORK_TIMEOUT':
      console.log('Check network connectivity to Gerrit server');
      break;
    case 'VERSION_INCOMPATIBLE':
      console.log('Gerrit server version too old (minimum: 3.6)');
      break;
    default:
      console.log(`Unexpected error: ${error.message}`);
  }
}
```

### 7.2 Connection Issues

Troubleshoot connection problems:

```typescript
// Test connection and get detailed diagnostics
const connectionTest = await gerritInstanceService.testConnection(instanceId);

if (!connectionTest.success) {
  console.log(`Connection failed: ${connectionTest.errorMessage}`);
  console.log(`Suggested action: ${connectionTest.suggestedAction}`);
}
```

### 7.3 Sync Failures

Handle synchronization failures:

```typescript
// Check sync status for detailed error information
const syncStatus = await gerritSyncService.getSyncStatus();

for (const error of syncStatus.errors) {
  console.log(`Change ${error.changeId}: ${error.errorMessage}`);
  console.log(`Suggested action: ${error.suggestedAction}`);
}
```

## Performance Tips

### 8.1 Optimize Large Change Reviews

For changes with 500+ files:

```typescript
// Use pagination and lazy loading
const searchOptions = {
  limit: 10, // Process in smaller batches
  includeDiffs: false, // Load diffs on-demand
  includeComments: true
};

// Import changes progressively
const results = await gerritChangeService.searchChanges(query, searchOptions);
```

### 8.2 Efficient Batch Operations

For submitting many comments:

```typescript
// Create multiple comments efficiently
const comments = [
  { changeId: "12345", filePath: "file1.java", line: 10, message: "Comment 1" },
  { changeId: "12345", filePath: "file2.java", line: 20, message: "Comment 2" },
  // ... more comments
];

const createdComments = await gerritCommentService.createMultiple(comments);

// Submit all at once in review
await gerritReviewService.submitReview({
  changeId: "12345",
  commentIds: createdComments.map(c => c.id),
  // ... other review data
});
```

## Security Considerations

### 9.1 Credential Security

Credentials are automatically encrypted:

```typescript
// Credentials are encrypted with AES-256-GCM
const config = {
  name: "Secure Instance",
  url: "https://gerrit.company.com",
  username: "alice.chen",
  password: "http-password-token-from-gerrit", // Will be encrypted
  makeActive: true
};

// Password is encrypted before storage
await gerritInstanceService.createInstance(config);
```

### 9.2 HTTPS Enforcement

All communications use HTTPS:

```typescript
// URL must use HTTPS
const config = {
  name: "Secure Instance",
  url: "http://gerrit.example.com", // ❌ Will be rejected
  // ... other config
};

// This will throw VALIDATION_INVALID_URL error
await gerritInstanceService.createInstance(config);
```

## Next Steps

After completing this quick start guide, you can:

1. **Explore Advanced Features**: Learn about conflict resolution, three-way merges, and custom review workflows
2. **Configure Enterprise Features**: Set up multiple instances, custom sync intervals, and audit logging
3. **Integrate with CI/CD**: Automate review processes and integrate with your development pipeline
4. **Customize UI**: Tailor the review interface to your team's preferences

## Getting Help

If you encounter issues:

1. Check the error messages and suggested actions
2. Verify your Gerrit server version and configuration
3. Review the API documentation for detailed contract information
4. Check the HyperReview logs for detailed error information

**Common Issues:**
- Authentication failures: Verify HTTP password token in Gerrit settings
- Network timeouts: Check firewall and proxy settings
- Version incompatibility: Upgrade Gerrit to 3.6+ for full compatibility
- Large file performance: Use pagination and lazy loading strategies