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
cd ../core
cargo test export_bindings

cd ../../codegen/generate_ts_api
cd types
sed -i 's/: bigint/: number/g' *.ts
cd ../
npx tsx generate_rs.ts > ../../packages/core/src/remote_api/api/generated.rs
npx tsx generate_ts.ts > ./RemoteGameApi.ts

npx prettier --write types
npx prettier --write *.ts

rm -rf ../../game-script/generated
cp -R types ../../game-script/generated
cp -R RemoteGameApi.ts ../../game-script/generated