#!/usr/bin/env bash

set -euo pipefail

parse_branch_version() {
    local branch="$1"
    if [[ "$branch" =~ ^releases/v([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
        echo "${BASH_REMATCH[1]}"
    else
        echo "Error: Invalid branch name format: $branch" >&2
        echo "Expected: releases/v<major>.<minor>.<patch>" >&2
        return 1
    fi
}

get_latest_final_version() {
    local tag
    while IFS= read -r tag; do
        if [[ "$tag" =~ ^v([0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
            echo "${BASH_REMATCH[1]}"
            return 0
        fi
    done < <(git tag -l 'v[0-9]*.[0-9]*.[0-9]*' --sort=-version:refname)
    echo "Error: No final release tags found. Cannot determine current version." >&2
    return 1
}

# Release notes need this explicitly: final tags live on release branches, so the
# previous one is often missing from the new tag's history and GitHub falls back further.
get_previous_final_tag() {
    local version="$1" tag major minor patch t_major t_minor t_patch
    IFS='.' read -r major minor patch <<< "$version"
    while IFS= read -r tag; do
        if [[ "$tag" =~ ^v([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
            t_major=$((10#${BASH_REMATCH[1]})); t_minor=$((10#${BASH_REMATCH[2]})); t_patch=$((10#${BASH_REMATCH[3]}))
            if (( t_major < 10#$major
                || (t_major == 10#$major && t_minor < 10#$minor)
                || (t_major == 10#$major && t_minor == 10#$minor && t_patch < 10#$patch) )); then
                echo "$tag"
                return 0
            fi
        fi
    done < <(git tag -l 'v[0-9]*.[0-9]*.[0-9]*' --sort=-version:refname)
}

get_next_version() {
    local bump_type="$1" current_version major minor patch
    if [[ "$bump_type" != major && "$bump_type" != minor && "$bump_type" != patch ]]; then
        echo "Error: bump_type must be 'major', 'minor', or 'patch'" >&2
        return 1
    fi
    current_version=$(get_latest_final_version) || return 1
    IFS='.' read -r major minor patch <<< "$current_version"
    major=$((10#$major)); minor=$((10#$minor)); patch=$((10#$patch))
    case "$bump_type" in
        major) major=$((major + 1)); minor=0; patch=0 ;;
        minor) minor=$((minor + 1)); patch=0 ;;
        patch) patch=$((patch + 1)) ;;
    esac
    echo "${major}.${minor}.${patch}"
}

get_next_rc_number() {
    local version="$1" tag suffix number latest_rc=0
    while IFS= read -r tag; do
        suffix=${tag#"v${version}"}
        if [[ "$suffix" =~ ^(-rc\.|\.rc)([0-9]+)$ ]]; then
            number=$((10#${BASH_REMATCH[2]}))
            if (( number > latest_rc )); then
                latest_rc=$number
            fi
        fi
    done < <(git tag -l "v${version}-rc.*" "v${version}.rc*")
    echo "$((latest_rc + 1))"
}

get_rc_tag() {
    local version="$1" tag suffix rc_number
    # Only new-format tags are reused; legacy tags still reserve their RC numbers.
    while IFS= read -r tag; do
        suffix=${tag#"v${version}"}
        if [[ "$suffix" =~ ^-rc\.[0-9]+$ ]]; then
            echo "$tag"
            return 0
        fi
    done < <(git tag --list "v${version}-rc.*" --points-at HEAD --sort=-version:refname)
    rc_number=$(get_next_rc_number "$version")
    echo "v${version}-rc.${rc_number}"
}

tag_exists() {
    local tag="$1"
    git rev-parse --verify "refs/tags/$tag" >/dev/null 2>&1
}

configure_git() {
    git config user.name "github-actions[bot]"
    git config user.email "github-actions[bot]@users.noreply.github.com"
}

create_and_push_tag() {
    local tag="$1"
    local message="${2:-Release $tag}"

    git tag -a "$tag" -m "$message"
    git push origin "refs/tags/$tag"
}
