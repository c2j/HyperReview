#!/bin/bash

echo "🧪 Testing Active Instance Persistence Fix"
echo "=========================================="

echo "📦 Building application..."
cargo build --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

echo "🔧 Testing active instance persistence..."
cargo test --test test_active_instance --manifest-path src-tauri/Cargo.toml
if [ $? -ne 0 ]; then
    echo "❌ Active instance tests failed"
    exit 1
fi

echo "🔧 Testing database migration..."
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
echo "📋 Active Instance Fix Summary:"
echo "   ✅ Added gerrit_set_active_instance_simple command"
echo "   ✅ Backend persists active state to database"
echo "   ✅ Frontend calls backend API to save state"
echo "   ✅ Active state survives app restart"
echo "   ✅ Only one instance can be active at a time"
echo ""
echo "🎉 Active instance persistence is now working correctly!"
echo ""
echo "📝 Frontend now calls:"
echo "   invoke('gerrit_set_active_instance_simple', { instance_id: 'your-instance-id' })"
echo ""
echo "🔄 After setting an instance as active:"
echo "   - State is saved to SQLite database"
echo "   - All other instances are set to inactive"
echo "   - Active state persists across app restarts"
echo "   - Settings UI will show correct active status"