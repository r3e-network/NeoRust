# NeoRust 2.1.0 Release Plan

> **For maintainers:** Execute each gate in order. Do not create or push the release tag until the release commit is on `master` and every required check passes.

**Goal:** Ship the completed SDK correctness, reliability, security, and developer-experience work as NeoRust `2.1.0`.

**Version rationale:** The change set adds backwards-compatible public SDK capabilities, strengthens provider and retry behavior, and raises the minimum supported Rust version from 1.83 to 1.91. Under the repository's SemVer policy this is a minor release, not a patch; existing 2.x entry points remain available.

**Release architecture:** The repository publishes from an annotated `vX.Y.Z` tag through `.github/workflows/release.yml`. The tag must match the root `neo3` package version. GitHub Actions builds cross-platform CLI artifacts, runs `cargo publish --dry-run --locked`, publishes `neo3` when the repository token is configured, and creates the GitHub release from the matching `CHANGELOG.md` section. `neo-cli` is versioned with the SDK for artifact clarity but is not published to crates.io.

**Constraints:** Preserve the existing hardening diff, add no dependencies, keep 2.x API compatibility, use Rust 1.91 as the MSRV, and never retry a crates.io publication blindly after an ambiguous failure. The known `RUSTSEC-2023-0071` exception remains limited to the transitive RSA feature that the SDK's HS256-only JWT API does not exercise.

---

## Task 1: Establish the release branch and baseline

**Files:**
- Create: `docs/superpowers/plans/2026-07-11-neo-rust-sdk-2.1.0-release.md`
- Inspect: `Cargo.toml`
- Inspect: `neo-cli/Cargo.toml`
- Inspect: `Cargo.lock`
- Inspect: `CHANGELOG.md`
- Inspect: `.github/workflows/release.yml`
- Inspect: `docs/RELEASE_PROCESS.md`

1. Confirm `v2.1.0` does not exist locally, on GitHub, or on crates.io.
2. Confirm GitHub authentication, repository permissions, default branch, and the most recent successful release convention.
3. Create `release/v2.1.0` from the current dirty `master` worktree without resetting or stashing the completed work.
4. Record the pre-release `master` commit and inspect all changed files before staging anything.

## Task 2: Update release metadata and notes

**Files:**
- Modify: `Cargo.toml`
- Modify: `neo-cli/Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `CHANGELOG.md`
- Modify: `.github/workflows/release.yml`
- Modify: `.cargo/config.toml`
- Modify: `examples/*/Cargo.toml` where publication is not already disabled
- Modify as factually required: `README.md`
- Modify as factually required: `docs/RELEASE_PROCESS.md`

1. Change the root `neo3` version from `2.0.1` to `2.1.0`.
2. Change `neo-cli` to `2.1.0` and its local `neo3` dependency requirement to `2.1.0`.
3. Regenerate the lockfile through Cargo and verify both workspace packages resolve to `2.1.0`.
4. Make publication intent explicit: only `neo3` publishes to crates.io; the CLI and examples remain applications/examples and cannot be published accidentally.
5. Make the tag workflow run release verification and fail when crates.io credentials are missing rather than producing a partial green release.
6. Replace the empty Unreleased placeholder with a dated `2.1.0` entry covering public API additions, correctness fixes, retry/provider hardening, secret redaction, contract/CLI fixes, Neo X compatibility, dependency policy, MSRV, and verification.
7. Restore an empty Unreleased section above the release entry and add/update changelog comparison links if the file uses them.
8. Audit release-facing documentation for stale version, MSRV, feature, and behavior claims. Make only factual updates supported by the code.

## Task 3: Run local release gates

Run from the repository root and retain terminal results:

1. `cargo fmt --all --check`
2. `git diff --check`
3. `cargo clippy --workspace --locked --all-features --all-targets -- -D warnings`
4. `cargo clippy --workspace --locked --no-default-features --all-targets -- -D warnings`
5. `cargo +1.91.0 check -p neo3 --lib --locked --all-features`
6. `cargo test --workspace --locked`
7. `RUSTDOCFLAGS="-D warnings" cargo doc -p neo3 --locked --all-features --no-deps`
8. `cargo audit`
9. `cargo deny check --hide-inclusion-graph`
10. `cargo publish --dry-run --locked --allow-dirty`
11. Inspect the generated package list and unpacked manifest to confirm required documentation, licenses, sources, and lockfile are present and no local-only paths leak into the published package.

If a gate fails, fix the underlying release or implementation issue and rerun that gate plus any downstream gate affected by the change.

## Task 4: Review and commit the release candidate

**Files:** all intentional implementation, test, policy, and release files in the branch.

1. Obtain an independent API/security/release review of the complete diff against `v2.0.1`.
2. Resolve every correctness, security, publication, and compatibility finding; document accepted low-risk limitations in release notes where user-visible.
3. Stage only the reviewed files by explicit path.
4. Create logical Lore-formatted commits with intent-first subjects and `Confidence`, `Scope-risk`, `Tested`, and `Not-tested` trailers.
5. Confirm the release branch is clean and the final commit still reports package version `2.1.0`.

## Task 5: Validate the release branch remotely

1. Push `release/v2.1.0` to `origin`.
2. Open a pull request to `master` with the release notes, risk disclosures, and exact local verification evidence.
3. Wait for all required GitHub checks and review the logs for skipped or best-effort failures, not only the aggregate status.
4. Merge only after the head SHA and passing checks match the reviewed release candidate.
5. Fetch `origin/master` and verify the merged release commit, version metadata, and changelog content.

## Task 6: Publish and verify `v2.1.0`

1. Create an annotated `v2.1.0` tag on the verified release commit and push only that tag.
2. Monitor the GitHub `Release` workflow through completion, including prepare, platform builds, crates.io publish, and GitHub release jobs.
3. If publication status is ambiguous, query crates.io before retrying. Crate versions are immutable.
4. Verify all of the following independently:
   - `v2.1.0` resolves to the intended commit locally and on GitHub.
   - The GitHub release is public, non-draft, and contains Linux, macOS, and Windows artifacts.
   - crates.io reports `neo3 = 2.1.0` and the downloadable package matches the reviewed metadata.
   - docs.rs has queued or completed documentation for `neo3 2.1.0`.
   - GitHub Actions has no failed or cancelled release job.
5. Report the release URLs, commit/tag SHA, verification evidence, changed files, simplifications delivered, and remaining risks.

## Rollback Boundaries

- Before the tag is pushed: amend the release branch and rerun affected gates.
- After the tag is pushed but before crates.io publication: fix forward with a new commit and replacement tag only if GitHub shows the tag has not triggered externally; otherwise use a new patch version.
- After crates.io publication: never delete or overwrite `2.1.0`; publish a corrective `2.1.1`.
