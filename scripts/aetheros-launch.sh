#!/usr/bin/env bash
# LAUNCH SCRIPT - aetheros-launch.sh
set -euo pipefail

curl_opts=("--fail" "--max-time" "5" "--connect-timeout" "3" "-s")

echo "=========================================="
echo "🚀 AETHEROS v10.2 LAUNCH SEQUENCE"
echo "=========================================="
echo "Timestamp: $(date -u)"
echo "Version: v10.2.0"
echo "Build: $(git rev-parse --short HEAD || echo 'unknown')"
echo ""

echo "[1/5] Verifying infrastructure..."
if curl "${curl_opts[@]}" https://aetheros.dev/health > /dev/null; then
    echo "✅ Website: ONLINE"
else
    echo "❌ Website: OFFLINE"
    exit 1
fi

echo "[2/5] Checking services..."
services=(
    "https://api.aetheros.dev/health"
    "https://docs.aetheros.dev"
    "https://cdn.aetheros.dev"
)
for service in "${services[@]}"; do
    if curl "${curl_opts[@]}" "$service" > /dev/null; then
        echo "✅ $(echo $service | cut -d'/' -f3): ONLINE"
    else
        echo "❌ $(echo $service | cut -d'/' -f3): OFFLINE"
        exit 1
    fi
done

echo "[3/5] Monitoring metrics..."
if command -v jq >/dev/null 2>&1; then
  stars=$(curl -s https://api.github.com/repos/HaKaTo99/AetherOS | jq -r '.stargazers_count')
  echo "GitHub Stars: ${stars}"
else
  echo "GitHub Stars: (jq missing)"
fi

echo "[4/5] Final checks..."
if make test-all >/dev/null 2>&1; then
    echo "✅ All tests passing"
else
    echo "❌ Tests failing or make test-all not available"
    exit 1
fi

echo "[5/5] 🚀 LAUNCHING AETHEROS v1.0..."

echo "🎉 AETHEROS IS NOW LIVE!"
echo "🌐 Website: https://aetheros.dev"
echo "📦 GitHub: https://github.com/HaKaTo99/AetherOS"
echo "💬 Discord: https://discord.gg/aetheros"

echo "Thank you for being part of the revolution!"
