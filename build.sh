#!/bin/bash

echo "Building frontend..."
cd frontend
pnpm build

echo "frontend built. Preparing Rust server"
cd ../backend
cargo build
echo "Backend build finished -- Executable: backend/target/debug/backend"
# If you want to build it for relaase add --relaase on cargo build
# backend/target/debug/backend
