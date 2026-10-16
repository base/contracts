# Release process

Release versions come from branch names, not a repository version file. Release
management creates branches, tags, and draft GitHub releases. It does not build
artifacts or change individual contracts' versions or semver snapshots.

## Start a release

Run **Start Release** in GitHub Actions from `main` and select a bump type:

- `minor` is the default. It increments the minor version and resets the patch.
- `patch` increments the patch version.
- `major` increments the major version and resets the minor and patch.

The workflow uses the highest final `vX.Y.Z` tag by version order. RC tags of
both formats are ignored. For example, `v8.10.0` comes after `v8.9.0`, even if a
`v9.0.0-rc.1` tag also exists.

Major and minor releases branch from `main`. Patch releases branch from the
highest existing `releases/vX.Y.*` branch for the latest final tag's major and
minor version. Starting a release fails if the new branch already exists or if
a patch release has no matching base branch.

Start Release pushes `releases/vX.Y.Z`, then creates the first RC at its head in
the same workflow. Branch pushes made with `GITHUB_TOKEN` do not trigger other
workflows, so it cannot rely on Create RC to make that first tag. There is no
version-sync step.

## Create release candidates

Every push to a `releases/v*` branch containing the automation runs **Create RC**.
It tags the triggering commit, even if newer commits arrive while it waits.
New tags use `vX.Y.Z-rc.N`.

Legacy tags use `vX.Y.Z.rcN`. Both formats reserve RC numbers for their exact
version. For example, with `v8.4.0.rc1` and `v8.4.0.rc2` present, the next RC is
`v8.4.0-rc.3`. Numbering is numeric, so `rc.10` follows `rc.9`. Tags for other
versions do not affect the count.

Tag allocation is serialized per release branch with `cancel-in-progress: false`
and `queue: max`. GitHub allows up to 100 pending jobs in that queue. A full
queue cancels additional jobs. Jobs enter the queue when they start waiting,
which need not match commit order. Each job still tags its own triggering commit.

### Retries

Rerun a failed Create RC workflow. If a new-format RC tag for that version already
points at the triggering commit, the script reuses the highest such tag instead
of allocating another number. Legacy tags at that commit count toward numbering
but are not reused.

Start Release refuses to reuse an existing release branch, including after a
partial failure. If the branch was pushed but the initial RC failed, check for
an RC tag at that branch head. To finish the tagging step without rerunning
Start Release, use an up-to-date local clone containing the release scripts:

```sh
git fetch origin --tags
git checkout --detach origin/releases/v8.5.0
./scripts/release/create-tag.sh releases/v8.5.0 rc
```

Substitute the release version. The script pushes a new tag if needed and reuses
a new-format tag already at HEAD. For a historical branch without the scripts,
keep a copy of `scripts/release/` outside the checkout before switching to it.
Do not move an existing tag to repair a failed run.

## Publish the final release

Run **Publish Release** from `main` and enter `X.Y.Z`, without a `v` or `releases/`
prefix. The workflow:

- Validates the version and checks that `releases/vX.Y.Z` exists.
- Checks out the release branch head and creates `vX.Y.Z` there.
- Creates a draft GitHub release with GitHub-generated "What's Changed" notes.

Review the notes and publish the draft on GitHub. There are no Docker images,
binaries, or other release assets to wait for.

Publishing is serialized per version. If the final tag already points at the
release branch head, a retry reuses it. A final tag at a different commit causes
a failure; the workflow never moves it. If a GitHub release already exists,
the workflow leaves its notes and draft or published state unchanged.

A final tag freezes the release branch. **Release Branch CI** rejects pull
requests targeting a `releases/vX.Y.Z` branch once `vX.Y.Z` exists, even if its
GitHub release is still a draft. Keep this check required in branch protection
to enforce the freeze on merges. The check does not prevent direct pushes.

## Existing release branches

Push-triggered workflows only run on release branches that contain their
workflow files. Updating `main` does not install automation on existing branches.
Branches such as `releases/v8.4.0` keep the manual release process unless the
release workflows and scripts are backported. Patch branches created from an
older release branch inherit that limitation, although Start Release still
creates their initial RC using the tools from `main`.

To adopt automatic RCs and the finalized-release check on an active branch,
backport the release workflows and `scripts/release/`. Existing queued or running
workflows keep the configuration they started with. Publish Release runs with
tools from `main`, so it can tag an existing branch without that backport.

## Quick reference

| Action | Workflow | Trigger | Output |
|--------|----------|---------|--------|
| Start a release | Start Release | Manual bump type | Release branch and first RC tag |
| Create an RC | Create RC | Push to `releases/v*` | RC tag at the triggering commit |
| Finalize a release | Publish Release | Manual version | Final tag and draft GitHub release |
| Guard a finalized branch | Release Branch CI | Pull request to `releases/v*` | Fails if the final tag exists |

Run `just test-release` to test version selection, legacy RC numbering, retry
behavior, and final-tag protection locally without network access or pushes.
