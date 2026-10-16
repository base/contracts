#!/usr/bin/env bash

set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=common.sh
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

RELEASE_BRANCH="${1:-}"
RELEASE_TYPE="${2:-}"

if [[ -z "$RELEASE_BRANCH" || -z "$RELEASE_TYPE" ]]; then
    echo "Usage: $0 <release_branch> <release_type> (rc|final)" >&2
    exit 1
fi

if [[ "$RELEASE_TYPE" != rc && "$RELEASE_TYPE" != final ]]; then
    echo "Error: release_type must be 'rc' or 'final'" >&2
    exit 1
fi

main() {
    echo "=== Create Release Tag ==="
    echo "Branch: $RELEASE_BRANCH"
    echo "Type: $RELEASE_TYPE"

    VERSION=$(parse_branch_version "$RELEASE_BRANCH")
    echo "Version: $VERSION"

    local TAG TAG_SHA HEAD_SHA
    if [[ "$RELEASE_TYPE" == rc ]]; then
        TAG=$(get_rc_tag "$VERSION")
        echo "Using RC tag: $TAG"
    else
        TAG="v${VERSION}"
        echo "Using final release tag: $TAG"
    fi

    if tag_exists "$TAG"; then
        TAG_SHA=$(git rev-parse "refs/tags/$TAG^{}")
        HEAD_SHA=$(git rev-parse HEAD)
        if [[ "$TAG_SHA" == "$HEAD_SHA" ]]; then
            echo "Tag $TAG already exists at HEAD, skipping creation"
        else
            echo "Error: Tag $TAG already exists but points at $TAG_SHA, not HEAD ($HEAD_SHA)" >&2
            exit 1
        fi
    else
        configure_git
        create_and_push_tag "$TAG"
        echo "=== Tag $TAG created and pushed ==="
    fi

    if [[ -n "${RUNNER_TEMP:-}" ]]; then
        echo "$TAG" > "${RUNNER_TEMP}/release_tag"
    fi
}

main
