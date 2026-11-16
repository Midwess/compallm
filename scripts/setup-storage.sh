#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
storage_project="${STORAGE_COMPOSE_PROJECT:-cosin}"

mocker compose create --project-name "$storage_project" s3
mocker compose stop --project-name "$storage_project" s3
mocker run --rm --user 0:0 --volume "${storage_project}-s3-data:/data" \
  --entrypoint /bin/sh docker.io/rustfs/rustfs:1.0.5 \
  -c 'chown 10001:10001 /data'
mocker compose up --project-name "$storage_project" -d --force-recreate s3

storage_ready=false
for attempt in {1..60}; do
  if curl --silent --fail --max-time 2 http://127.0.0.1:9000/health >/dev/null; then
    storage_ready=true
    break
  fi
  sleep 1
done
if [[ "$storage_ready" != true ]]; then
  mocker compose logs --project-name "$storage_project" s3
  exit 1
fi
cargo run -p compallm-api -- storage setup
