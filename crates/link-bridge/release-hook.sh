#!/bin/bash
set -exo pipefail

NAME="CHANGELOG.md"
PACKAGE="link-bridge"
REPO_DIR="../.."

VERSION="${NEW_VERSION:-${1}}"

if [[ -z "${VERSION}" ]]; then
    echo "Error: No version specified (set NEW_VERSION or pass as argument)" >&2
    exit 1
fi

gen-changelog generate \
    --display-summaries \
    --name "${NAME}" \
    --package "${PACKAGE}" \
    --repository-dir "${REPO_DIR}" \
    --next-version "${VERSION}"

echo "Generated ${NAME} for ${PACKAGE}@${VERSION}"
