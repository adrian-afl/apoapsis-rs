#!/usr/bin/env sh
set -euo pipefail
set -o verbose

export TS_RS_LARGE_INT=number
export TS_RS_IMPORT_EXTENSION=js
export TS_RS_EXPORT_DIR=$PWD/generate_ts_api/types

echo "TS_RS_LARGE_INT = $TS_RS_LARGE_INT"
echo "TS_RS_IMPORT_EXTENSION = $TS_RS_IMPORT_EXTENSION"
echo "TS_RS_EXPORT_DIR = $TS_RS_EXPORT_DIR"

cp generate_ts_api/empty_generated.rs ../packages/core/src/remote_api/api/generated.rs

rm generate_ts_api/types/* || true

cd ../
cargo test export_bindings
cd packages/ecs
cargo test export_bindings
cd ../core
cargo test export_bindings
cd ../math
cargo test export_bindings

cd ../../codegen/generate_ts_api
cd types
sed -i 's/: bigint/: number/g' *.ts
cd ../
npx tsx generate_rs.ts > ../../packages/core/src/remote_api/api/generated.rs
npx tsx generate_ts.ts > ./RemoteGameApi.ts
npx tsx generate_ts_events.ts > ./RemoteGameEvents.ts

npx prettier --write types
npx prettier --write *.ts

rm -rf ../../game-script/generated
mkdir ../../game-script/generated
mkdir ../../game-script/generated/types
mv types ../../game-script/generated/
mv RemoteGameApi.ts ../../game-script/generated/
mv RemoteGameEvents.ts ../../game-script/generated/