## 1. Project Setup

- [ ] 1.1 Add `clap` and `git2` dependencies to `Cargo.toml` using `cargo add`.
- [ ] 1.2 Reorganize `src/main.rs` to delegate to a new `src/lib.rs` so core logic is unit-testable.
- [ ] 1.3 Create empty module files for `cli`, `git`, `nix`, and `version` under `src/`.

## 2. CLI Argument Parsing

- [ ] 2.1 Define a `Cli` struct with `clap` derive supporting:
  - positional `attr_path` (required),
  - `--repo PATH` (env: `NIXPKGS`),
  - `--source-branch NAME` (env: `NIXPKGS_SOURCE_BRANCH`, default: `master`),
  - `--release-branch NAME` (env: `NIXPKGS_RELEASE_BRANCH`, default: `release-26.05`),
  - `--from VERSION`,
  - `--to VERSION`,
   `-n, --dry-run (also known as --dry-run)`,
- [ ] 2.2 Add unit tests covering minimal invocation, all flags, environment-variable defaults, precedence (flag over env), and missing attribute path.

## 3. Repository Validation

- [ ] 3.1 Open the local Git repository with `git2`, using `--repo` or the current working directory.
- [ ] 3.2 Resolve the configured source and release branch refs and report clear errors if either is missing.
- [ ] 3.3 Add unit tests using an in-memory `git2` repository with configurable source and release branches.

## 4. Attribute Path Resolution

- [ ] 4.1 Implement resolution via `nix eval` / `nix-instantiate --eval` reading `builtins.unsafeGetAttrPos` for the given attribute path.
- [ ] 4.2 Implement a heuristic fallback that maps common prefixes (e.g. `python3Packages.*`, `nodePackages.*`) to known `nixpkgs` directory layouts when the `nix` command is unavailable.
- [ ] 4.3 Add unit tests for both the Nix evaluator path and the heuristic fallback, plus error cases for invalid attributes.

## 5. Commit Discovery and Filtering

- [ ] 5.1 Walk the configured source branch with `git2::Revwalk`, filtering to commits that touch the resolved package file path.
- [ ] 5.2 Exclude any candidate commit that is an ancestor of the release branch HEAD.
- [ ] 5.3 Sort remaining commits oldest-first by commit timestamp.
- [ ] 5.4 Add unit tests with fixture repositories containing overlapping and non-overlapping commits between branches.

## 6. Version Filtering

- [ ] 6.1 Determine the version introduced by each candidate commit by evaluating the derivation at that commit or by inspecting the commit diff for `version` changes.
- [ ] 6.2 Apply `--from` as a lower bound and `--to` as an upper bound on the introduced version.
- [ ] 6.3 Add unit tests covering `--from`, `--to`, and combined range filters.

## 7. Output Formatting

- [ ] 7.1 Format each selected commit as `git cherry-pick -x <sha>` when `--dry-run` is not set.
- [ ] 7.2 When `--dry-run` is set, format each selected commit as a human-readable line containing SHA, date, and summary.
- [ ] 7.3 Ensure output is ordered oldest-first and that no output is produced when no commits are selected.
- [ ] 7.4 Add unit tests for command formatting, dry-run formatting, and ordering.

## 8. Error Handling and Integration

- [ ] 8.1 Wire the CLI, repository, resolution, filtering, and formatting modules together in `main`.
- [ ] 8.2 Return non-zero exit codes with clear error messages for invalid input, missing repository, missing branches, and unresolvable attributes.
- [ ] 8.3 Add a smoke test that invokes the binary with `--help` and an end-to-end test against a small fixture repository.

## 9. Verification

- [ ] 9.1 Run `cargo build` and fix any compilation errors.
- [ ] 9.2 Run `cargo test` and fix failing tests.
- [ ] 9.3 Run `cargo clippy -- -D warnings` and resolve all warnings.
- [ ] 9.4 Run `cargo fmt --check` and fix any formatting issues.
