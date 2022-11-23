#!/bin/sh
cd backend 

cargo clippy --workspace -- -D warnings

cargo fmt --all -- 

cargo test --workspace

cd ..
cd frontend

npm run build

cd ..

$SHELL