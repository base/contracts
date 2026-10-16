#!/usr/bin/env bash

set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=common.sh
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

BUMP_TYPE="${1:-}"

if [[ -z "$BUMP_TYPE" ]]; then
    echo "Usage: $0 <bump_type> (major|minor|patch)" >&2
    exit 1
fi

main() {
    echo "=== Start Release (bump: $BUMP_TYPE) ==="

    CURRENT_VERSION=$(get_latest_final_version)
    NEW_VERSION=$(get_next_version "$BUMP_TYPE")
    RELEASE_BRANCH="releases/v${NEW_VERSION}"

    local base=main major minor tag
    if [[ "$BUMP_TYPE" == patch ]]; then
        IFS='.' read -r major minor _ <<< "$CURRENT_VERSION"
        base=$(git ls-remote --heads origin "releases/v${major}.${minor}.*" \
            | awk '{print $2}' \
            | sed -nE "s|^refs/heads/(releases/v${major}\\.${minor}\\.[0-9]+)$|\\1|p" \
            | sort -V | tail -1)
        if [[ -z "$base" ]]; then
            echo "Error: No existing releases/v${major}.${minor}.* branch found on remote." >&2
            echo "Cannot create a patch release without a prior release branch." >&2
            exit 1
        fi
    fi

    echo "Current version: $CURRENT_VERSION"
    echo "New version: $NEW_VERSION"
    echo "Release branch: $RELEASE_BRANCH"
    echo "Base: $base"

    if [[ -n "$(git ls-remote --heads origin "refs/heads/$RELEASE_BRANCH")" ]]; then
        echo "Error: Branch $RELEASE_BRANCH already exists on remote." >&2
        exit 1
    fi

    configure_git
    git fetch origin "refs/heads/$base:refs/remotes/origin/$base"
    git checkout -b "$RELEASE_BRANCH" "origin/$base"
    tag="v${NEW_VERSION}-rc.$(get_next_rc_number "$NEW_VERSION")"
    git tag -a "$tag" -m "Release $tag"
    git push --atomic origin "refs/heads/$RELEASE_BRANCH" "refs/tags/$tag"

    if [[ -n "${RUNNER_TEMP:-}" ]]; then
        echo "$CURRENT_VERSION" > "${RUNNER_TEMP}/current_version"
        echo "$NEW_VERSION" > "${RUNNER_TEMP}/new_version"
        echo "$RELEASE_BRANCH" > "${RUNNER_TEMP}/release_branch"
        echo "$tag" > "${RUNNER_TEMP}/release_tag"
    fi

    echo "=== Branch $RELEASE_BRANCH and tag $tag created (v$CURRENT_VERSION to v$NEW_VERSION) ==="
}

main
