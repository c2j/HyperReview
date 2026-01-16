#!/bin/bash

echo "🧪 Testing Gerrit Connection Fix"
echo "================================"

echo "📦 Building application..."
cargo build --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

echo "🔧 Testing database migration and connection functionality..."
cargo test --test test_migration_functionality --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "❌ Migration tests failed"
    exit 1
fi

echo "🔧 Testing Gerrit integration..."
cargo test --test gerrit_integration_test --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "❌ Integration tests failed"
    exit 1
fi

echo "✅ All tests passed!"

echo ""
echo "📋 Connection Fix Summary:"
echo "   - Added gerrit_test_connection_by_id command"
echo "   - Fixed authentication in test_connection method"
echo "   - Updated frontend to use correct command"
echo "   - Database status updates on connection test"
echo ""
echo "🎉 Connection test should now work correctly!"
echo ""
echo "📝 Frontend should now call:"
echo "   invoke('gerrit_test_connection_by_id', { instance_id: 'your-instance-id' })"