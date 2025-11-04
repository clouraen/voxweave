#!/bin/bash

# Test script for gesture control functionality
echo "Testing gesture control functionality..."

# Test 1: Check if teleprompter component exists
echo "Test 1: Checking teleprompter component..."
if [ -f "abogen-ui/crates/ui/components/teleprompter.rs" ]; then
    echo "✓ Teleprompter component found"
else
    echo "✗ Teleprompter component not found"
    exit 1
fi

# Test 2: Check if gesture control component exists
echo "Test 2: Checking gesture control component..."
if [ -f "abogen-ui/crates/ui/components/gesture_control.rs" ]; then
    echo "✓ Gesture control component found"
else
    echo "✗ Gesture control component not found"
    exit 1
fi

# Test 3: Check if recording screen component exists
echo "Test 3: Checking recording screen component..."
if [ -f "abogen-ui/crates/ui/components/recording_screen.rs" ]; then
    echo "✓ Recording screen component found"
else
    echo "✗ Recording screen component not found"
    exit 1
fi

# Test 4: Check if our tests exist
echo "Test 4: Checking test files..."
if [ -f "abogen-ui/crates/ui/tests/gesture_control_tests.rs" ]; then
    echo "✓ Gesture control tests found"
else
    echo "✗ Gesture control tests not found"
    exit 1
fi

echo "All basic tests passed!"