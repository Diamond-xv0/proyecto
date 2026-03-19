#!/usr/bin/env bash
set -euo pipefail

TARGET="x86_64-pc-windows-gnu"
APP_NAME="proyecto_uno"
RELEASE_DIR="target/${TARGET}/release"
DIST_DIR="dist/windows"
PACKAGE_DIR="${DIST_DIR}/${APP_NAME}"
ZIP_PATH="${DIST_DIR}/${APP_NAME}-windows.zip"

if ! command -v rustup >/dev/null 2>&1; then
  echo "Error: rustup no está instalado o no está en PATH"
  exit 1
fi

if ! rustup target list --installed | grep -qx "${TARGET}"; then
  echo "Instalando target ${TARGET}..."
  rustup target add "${TARGET}"
fi

if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  echo "Error: falta x86_64-w64-mingw32-gcc (instala mingw-w64)"
  exit 1
fi

if ! command -v zip >/dev/null 2>&1; then
  echo "Error: falta comando zip. Instala con: sudo apt install -y zip"
  exit 1
fi

echo "Compilando en release para ${TARGET}..."
cargo build --release --target "${TARGET}"

mkdir -p "${PACKAGE_DIR}"
cp "${RELEASE_DIR}/${APP_NAME}.exe" "${PACKAGE_DIR}/"

copy_runtime_dll() {
  local dll_name="$1"
  local dll_path
  dll_path="$(x86_64-w64-mingw32-gcc -print-file-name="${dll_name}")"

  if [[ -z "${dll_path}" || "${dll_path}" == "${dll_name}" || ! -f "${dll_path}" ]]; then
    echo "Aviso: no se encontró ${dll_name} automáticamente."
    return 0
  fi

  cp "${dll_path}" "${PACKAGE_DIR}/"
}

copy_runtime_dll "libstdc++-6.dll"
copy_runtime_dll "libgcc_s_seh-1.dll"
copy_runtime_dll "libwinpthread-1.dll"

rm -f "${ZIP_PATH}"
(
  cd "${DIST_DIR}"
  zip -r "$(basename "${ZIP_PATH}")" "${APP_NAME}" >/dev/null
)

echo "Listo. Paquete generado en: ${ZIP_PATH}"
echo "Contenido listo para compartir en: ${PACKAGE_DIR}"
