#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=common.sh
source "$SCRIPT_DIR/common.sh"

TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT
export GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_GLOBAL=/dev/null
export RUNNER_TEMP="$TEMP_DIR"
git init -q "$TEMP_DIR/repo"
cd "$TEMP_DIR/repo"
git config user.name 'Release test'
git config user.email 'release-test@example.com'
git config commit.gpgsign false
git config tag.gpgsign false
git config core.hooksPath /dev/null
git commit --allow-empty -qm 'Initial commit'

assert_equal() {
    local expected="$1" actual="$2" scenario="$3"
    if [[ "$actual" != "$expected" ]]; then
        echo "FAIL: $scenario: expected '$expected', got '$actual'" >&2
        exit 1
    fi
    echo "PASS: $scenario"
}

assert_rejected() {
    local expected="$1"
    shift
    if "$@" >"$TEMP_DIR/output" 2>&1; then
        echo "FAIL: expected rejection from $*" >&2
        exit 1
    fi
    if ! grep -Fq "$expected" "$TEMP_DIR/output"; then
        echo "FAIL: expected error '$expected', got '$(cat "$TEMP_DIR/output")'" >&2
        exit 1
    fi
    echo "PASS: $* rejects with '$expected'"
}

test_rc_numbering() {
    git tag v8.4.0.rc1
    git tag v8.4.0.rc2
    git tag v8.4.1-rc.5
    git tag v8.4.10.rc7
    assert_equal 3 "$(get_next_rc_number 8.4.0)" 'legacy RC numbering continues and excludes other versions'
    assert_equal 1 "$(get_next_rc_number 8.5.0)" 'a version without RCs starts at one'
    assert_equal v8.4.0-rc.3 "$(get_rc_tag 8.4.0)" 'legacy RCs produce the next tag in the new format'
    assert_equal v8.5.0-rc.1 "$(get_rc_tag 8.5.0)" 'a version without RCs produces the first tag'

    git tag v8.4.0-rc.9
    assert_equal 10 "$(get_next_rc_number 8.4.0)" 'RC numbering is numeric'
    git tag v8.4.0-rc.10
    git tag v8.4.0.rc12
    git tag v8.4.0-rc.invalid
    git tag v8.4.0.rc99-extra
    assert_equal 13 "$(get_next_rc_number 8.4.0)" 'mixed RC formats use the highest valid number'

    git tag -a v8.4.0-rc.11 -m 'Annotated RC'
    assert_equal v8.4.0-rc.11 "$(get_rc_tag 8.4.0)" 'retries reuse the highest new-format tag at HEAD'
    bash "$SCRIPT_DIR/create-tag.sh" releases/v8.4.0 rc
    assert_equal v8.4.0-rc.11 "$(cat "$RUNNER_TEMP/release_tag")" 'RC script retries without a remote or push'

    git commit --allow-empty -qm 'Next candidate'
    git tag v8.4.0.rc15
    assert_equal v8.4.0-rc.16 "$(get_rc_tag 8.4.0)" 'legacy tags at HEAD are counted but not reused'
}

test_version_bumps() {
    git tag v8.9.0
    git tag v8.10.0
    git tag v8.10.1-rc.9
    git tag v9.0.0.rc2
    git tag v99.0.0-preview
    assert_equal 8.10.0 "$(get_latest_final_version)" 'latest final ignores both RC formats and uses version order'
    assert_equal 9.0.0 "$(get_next_version major)" 'major bumps reset minor and patch'
    assert_equal 8.11.0 "$(get_next_version minor)" 'minor bumps reset patch'
    assert_equal 8.10.1 "$(get_next_version patch)" 'patch bumps increment patch'
    git tag v8.10.4
    assert_equal 9.0.0 "$(get_next_version major)" 'major bumps reset a nonzero patch'
    assert_equal 8.11.0 "$(get_next_version minor)" 'minor bumps reset a nonzero patch'
    assert_equal 8.10.5 "$(get_next_version patch)" 'patch bumps retain major and minor'
    assert_rejected 'bump_type must be' get_next_version invalid

    git tag -d v8.9.0 v8.10.0 v8.10.4 >/dev/null
    assert_rejected 'No final release tags found' get_latest_final_version
    assert_rejected 'No final release tags found' get_next_version minor
}

test_final_tags() {
    assert_equal 8.4.0 "$(parse_branch_version releases/v8.4.0)" 'version comes from the release branch'
    assert_rejected 'Invalid branch name format' parse_branch_version releases/v8.4.0-extra
    bash "$SCRIPT_DIR/check-finalized.sh" releases/v8.4.0
    git tag -a v8.4.0 -m 'Final release'
    bash "$SCRIPT_DIR/create-tag.sh" releases/v8.4.0 final
    assert_equal v8.4.0 "$(cat "$RUNNER_TEMP/release_tag")" 'final script reuses an annotated tag at HEAD without pushing'
    assert_rejected 'Final release tag v8.4.0 already exists' bash "$SCRIPT_DIR/check-finalized.sh" releases/v8.4.0
    git commit --allow-empty -qm 'After final release'
    assert_rejected 'not HEAD' bash "$SCRIPT_DIR/create-tag.sh" releases/v8.4.0 final
}

test_start_release() (
    local remote="$TEMP_DIR/start-origin.git"
    git init -q --bare "$remote"
    git init -q -b main "$TEMP_DIR/start-repo"
    cd "$TEMP_DIR/start-repo"
    configure_git
    git commit --allow-empty -qm 'Initial release base'
    git tag v8.4.0
    git tag v8.4.1.rc2
    git remote add origin "$remote"
    git push -q origin main main:refs/heads/releases/v8.4.0 --tags
    local base_sha
    base_sha=$(git rev-parse HEAD)

    bash "$SCRIPT_DIR/start-release.sh" minor
    assert_equal v8.5.0-rc.1 "$(cat "$RUNNER_TEMP/release_tag")" 'a fresh release starts at RC one'
    assert_equal "$base_sha" "$(git --git-dir="$remote" rev-parse refs/heads/releases/v8.5.0)" 'the release branch is published at its base'
    assert_equal "$base_sha" "$(git --git-dir="$remote" rev-parse 'refs/tags/v8.5.0-rc.1^{}')" 'the first RC is published at the release head'
    assert_equal tag "$(git --git-dir="$remote" cat-file -t refs/tags/v8.5.0-rc.1)" 'the first RC is annotated'

    bash "$SCRIPT_DIR/start-release.sh" patch
    assert_equal v8.4.1-rc.3 "$(cat "$RUNNER_TEMP/release_tag")" 'the first RC continues legacy numbering'
    assert_equal "$base_sha" "$(git --git-dir="$remote" rev-parse 'refs/tags/v8.4.1-rc.3^{}')" 'a patch RC is published from a branch without release tools'

    tee "$remote/hooks/update" >/dev/null <<'EOF'
#!/usr/bin/env bash
if [[ "$1" == refs/tags/* ]]; then
    echo 'Tag rejected' >&2
    exit 1
fi
EOF
    chmod +x "$remote/hooks/update"
    assert_rejected 'Tag rejected' bash "$SCRIPT_DIR/start-release.sh" major
    assert_equal '' "$(git ls-remote origin refs/heads/releases/v9.0.0 refs/tags/v9.0.0-rc.1)" 'a rejected atomic push publishes neither the branch nor its first RC'
)

case "${1:-all}" in
    rc) test_rc_numbering ;;
    version) test_version_bumps ;;
    start) test_start_release ;;
    all)
        test_version_bumps
        test_rc_numbering
        test_final_tags
        test_start_release
        ;;
    *) echo "Usage: $0 [rc|version|start|all]" >&2; exit 1 ;;
esac

echo 'Release tests passed.'
