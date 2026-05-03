#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/target/helper"
MODULE_CACHE_DIR="${ROOT_DIR}/target/swift-module-cache"
SRC="${ROOT_DIR}/swift/ReminderHelper.swift"
OUT="${OUT_DIR}/ReminderHelper"
ARCH="$(uname -m)"
DEPLOYMENT_TARGET="${MACOSX_DEPLOYMENT_TARGET:-13.0}"

if ! command -v xcrun >/dev/null 2>&1; then
  echo "error: xcrun was not found. Install Xcode Command Line Tools with: xcode-select --install" >&2
  exit 1
fi

if ! xcrun --sdk macosx --find swiftc >/dev/null 2>&1; then
  echo "error: swiftc was not found. Install Xcode Command Line Tools with: xcode-select --install" >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"
mkdir -p "${MODULE_CACHE_DIR}"

SWIFT_TARGET_ARCH="${ARCH}"
SDK_PATH="$(xcrun --sdk macosx --show-sdk-path)"
SWIFT_MODULE_DIR="${SDK_PATH}/usr/lib/swift/Swift.swiftmodule"
if [[ "${ARCH}" == "arm64" \
  && ! -e "${SWIFT_MODULE_DIR}/arm64-apple-macos.swiftmodule" \
  && ! -e "${SWIFT_MODULE_DIR}/arm64-apple-macos.swiftinterface" \
  && -e "${SWIFT_MODULE_DIR}/arm64e-apple-macos.swiftinterface" ]]; then
  SWIFT_TARGET_ARCH="arm64e"
fi

xcrun --sdk macosx swiftc \
  -target "${SWIFT_TARGET_ARCH}-apple-macosx${DEPLOYMENT_TARGET}" \
  -module-cache-path "${MODULE_CACHE_DIR}" \
  -parse-as-library \
  "${SRC}" \
  -o "${OUT}"
echo "Built ${OUT}"
