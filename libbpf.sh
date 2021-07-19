#!/usr/bin/env bash
set -exuo pipefail

LIBNAME="libbpf"
ARCHIVE_FILE="${LIBNAME}.zip"

[[ -z $(find /usr -name "${LIBNAME}.so" -print -quit) ]] || {
  echo "${LIBNAME}.so has ben already installed"
  exit
}

if [ ! -f "${ARCHIVE_FILE}" ]; then
  wget -q -O "${ARCHIVE_FILE}" https://github.com/libbpf/libbpf/archive/refs/heads/master.zip
fi

if [ ! -d "${LIBNAME}-master" ]; then
  unzip -q "${ARCHIVE_FILE}"
fi

pushd "${LIBNAME}-master/src"
make && make install
popd
rm -rf "${ARCHIVE_FILE}" "${LIBNAME}-master"
