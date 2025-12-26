#!/usr/bin/env sh
set -euo pipefail

export TS_RS_LARGE_INT=number
export TS_RS_IMPORT_EXTENSION=js
export TS_RS_EXPORT_DIR=$PWD/generate_ts_api/types

echo "TS_RS_LARGE_INT = $TS_RS_LARGE_INT"
echo "TS_RS_IMPORT_EXTENSION = $TS_RS_IMPORT_EXTENSION"
echo "TS_RS_EXPORT_DIR = $TS_RS_EXPORT_DIR"

rm generate_ts_api/types/* || true

cd ../packages/ecs
cargo test export_bindings
cd ../remote
cargo test export_bindings

cd ../../codegen/generate_ts_api
npx prettier --write types