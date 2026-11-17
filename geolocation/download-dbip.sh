#!/bin/sh
set -eu

release="${1:-$(date -u +%Y-%m)}"
case "$release" in
    [0-9][0-9][0-9][0-9]-[0-9][0-9]) ;;
    *) echo "Usage: $0 [YYYY-MM]" >&2; exit 1 ;;
esac
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
destination="$root/artifacts/geoip"
mkdir -p "$destination"
work=$(mktemp -d "$destination/.dbip.XXXXXX")
trap 'rm -rf "$work"' 0
trap 'exit 1' HUP INT TERM
curl --fail --location --connect-timeout 10 --max-time 180 \
    "https://download.db-ip.com/free/dbip-city-lite-$release.mmdb.gz" -o "$work/city.gz"
gzip -dc "$work/city.gz" > "$work/city.mmdb"
printf '%s\n' 'IP Geolocation by DB-IP (https://db-ip.com)' \
    'DB-IP City Lite, Creative Commons Attribution 4.0: https://creativecommons.org/licenses/by/4.0/' \
    > "$work/ATTRIBUTION.txt"
mv "$work/city.mmdb" "$destination/dbip-city-lite.mmdb"
mv "$work/ATTRIBUTION.txt" "$destination/DB-IP-ATTRIBUTION.txt"
printf '%s\n' 'DB-IP City database installed. Restart the geolocation server to use the new file.'
