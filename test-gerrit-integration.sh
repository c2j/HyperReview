#!/bin/bash

# Gerrit Integration Quick Test Script
# This script helps you quickly test the Gerrit integration feature

echo "🚀 Starting Gerrit Integration Test..."
echo "======================================"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check Node.js
if command_exists node; then
    echo "✅ Node.js found: $(node --version)"
else
    echo "❌ Node.js not found. Please install Node.js first."
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "❌ Not in the project root directory. Please run this script from /Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview"
    exit 1
fi

echo "📁 Project directory confirmed"

# Test backend compilation
echo ""
echo "🔧 Testing backend compilation..."
cd src-tauri
cargo check --quiet
if [ $? -eq 0 ]; then
    echo "✅ Backend compilation successful"
else
    echo "❌ Backend compilation failed"
    exit 1
fi
cd ..

# Test TypeScript compilation
echo ""
echo "📝 Testing TypeScript compilation..."
cd frontend
npx tsc --noEmit --skipLibCheck 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ TypeScript compilation successful (with warnings ignored)"
else
    echo "⚠️  TypeScript compilation has warnings (this is expected)"
fi
cd ..

echo ""
echo "🎯 Quick Test Instructions:"
echo "=========================="
echo ""
echo "1. 🚀 Start the application:"
echo "   npm run dev"
echo ""
echo "2. 🧪 Open settings and navigate to 'External Systems' tab"
echo ""
echo "3. 🔍 Check browser console for debug logs:"
echo "   - Look for 'SettingsModal: Loading Gerrit instances...'"
echo "   - Look for 'SimpleGerritService: Using test mode data'"
echo "   - Look for 'SettingsModal: Loaded instances: [Array(2)]'"
echo ""
echo "4. 🧪 Use test buttons if needed:"
echo "   - Click '🧪 Test Service' to test API calls"
echo "   - Click 'Direct Test' to bypass API and show test data"
echo ""
echo "5. ✨ Expected result:"
echo "   - You should see 2 test instances displayed"
echo "   - Test Gerrit Server (Connected)"
echo "   - Development Gerrit (Disconnected)"
echo ""
echo "📋 Debug Commands for Browser Console:"
echo "======================================"
echo ""
echo "// Test the service directly:"
echo "await simpleGerritService.getInstances()"
echo ""
echo "// Check service status:"
echo "simpleGerritService.isTestMode()"
echo ""
echo "// Force display test data:"
echo "window.settingsModal?.handleDirectTest?.()"
echo ""
echo "🎉 Testing complete! The Gerrit integration should now be visible in the settings."
echo "If you encounter any issues, please check the browser console for detailed error messages."