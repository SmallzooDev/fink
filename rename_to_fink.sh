#!/bin/bash

# Script to rename jkms to fink throughout the codebase

echo "Renaming jkms to fink..."

# Update Cargo.toml
echo "Updating Cargo.toml..."
sed -i '' 's/name = "jkms"/name = "fink"/g' Cargo.toml

# Update all Rust source files
echo "Updating Rust source files..."
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's/use jkms::/use fink::/g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's/JkmsError/FinkError/g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's/jkms Manager/fink Manager/g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's/"jkms"/"fink"/g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's/\.jkms/.fink/g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's|/jkms/|/fink/|g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's|"jkms"|"fink"|g' {} \;
find . -name "*.rs" -type f ! -path "./target/*" -exec sed -i '' 's|cargo_bin("jkms")|cargo_bin("fink")|g' {} \;

# Update CLAUDE.md
echo "Updating CLAUDE.md..."
sed -i '' 's/jkms/fink/g' CLAUDE.md
sed -i '' 's/JKMS/FINK/g' CLAUDE.md

# Update plan.md
echo "Updating plan.md..."
sed -i '' 's/jkms/fink/g' plan.md
sed -i '' 's/JKMS/FINK/g' plan.md

# Update shell scripts
echo "Updating shell scripts..."
sed -i '' 's/jkms/fink/g' generate_test_prompts.sh
sed -i '' 's/JKMS/FINK/g' generate_test_prompts.sh

echo "Rename complete! Remember to:"
echo "1. Rename the project directory from jkms to fink"
echo "2. Update any external references or documentation"
echo "3. Run 'cargo clean' and 'cargo build' to rebuild"
echo "4. Run 'cargo test' to ensure everything works"