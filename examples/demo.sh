#!/bin/bash


echo "=== Rust Shell Demo ==="
echo

echo "1. Testing Basic Shell:"
echo -e "help\necho Hello from basic shell\npwd\nexit" | cargo run --quiet

echo
echo "2. Testing Enhanced Shell:"
echo -e "help\nset DEMO=enhanced\necho Running \$DEMO shell demo\nhistory\nwhich cargo\nexit" | cargo run --quiet -- --enhanced

echo
echo "=== Demo Complete ===" 