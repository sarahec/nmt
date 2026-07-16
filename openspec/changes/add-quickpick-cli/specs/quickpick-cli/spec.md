## ADDED Requirements

### Requirement: quickpick accepts a package attribute path
The system SHALL accept a positional argument representing a Nixpkgs attribute path, such as `python3Packages.openai`.

#### Scenario: Invoked with a valid attribute path
- **WHEN** the user runs `quickpick python3Packages.openai`
- **THEN** quickpick SHALL parse the attribute path as the target package.

#### Scenario: Invoked without an attribute path
- **WHEN** the user runs `quickpick` with no positional argument
- **THEN** quickpick SHALL exit with a non-zero status and print usage instructions.

### Requirement: quickpick resolves the Nixpkgs repository
The system SHALL operate on a local Nixpkgs Git repository. It SHALL accept an optional `--repo PATH` argument and a `NIXPKGS_REPO` environment variable, with the CLI flag taking precedence. When neither is provided, it SHALL default to the current working directory. It SHALL validate that the repository contains the configured source and release branches.

#### Scenario: Repository path provided explicitly
- **WHEN** the user runs `quickpick --repo /path/to/nixpkgs python3Packages.openai`
- **THEN** quickpick SHALL use `/path/to/nixpkgs` as the repository.

#### Scenario: Repository path provided via environment variable
- **WHEN** `NIXPKGS_REPO=/path/to/nixpkgs` is set and no `--repo` flag is given
- **THEN** quickpick SHALL use `/path/to/nixpkgs` as the repository.

#### Scenario: Repository path omitted and current directory is valid
- **WHEN** the user runs `quickpick python3Packages.openai` from inside a nixpkgs clone
- **THEN** quickpick SHALL use the current working directory as the repository.

#### Scenario: Repository is missing a required branch
- **WHEN** the configured source or release branch does not exist in the repository
- **THEN** quickpick SHALL exit with a non-zero status and report which branch is missing.

### Requirement: quickpick supports environment variables for branch and repository configuration
The system SHALL read environment variables `NIXPKGS_REPO`, `NIXPKGS_SOURCE_BRANCH`, and `NIXPKGS_RELEASE_BRANCH` as defaults for the corresponding CLI options. CLI flags SHALL take precedence over environment variables, and environment variables SHALL take precedence over built-in defaults.

#### Scenario: Source branch set via environment variable
- **WHEN** `NIXPKGS_SOURCE_BRANCH=main` is set and no `--source-branch` flag is given
- **THEN** quickpick SHALL use `main` as the source branch.

#### Scenario: CLI flag overrides environment variable
- **WHEN** `NIXPKGS_SOURCE_BRANCH=main` is set and the user passes `--source-branch master`
- **THEN** quickpick SHALL use `master` as the source branch.

### Requirement: quickpick resolves the package expression file
The system SHALL determine the file path in the repository that corresponds to the given attribute path.

#### Scenario: Attribute resolves to a known expression file
- **WHEN** the user provides `python3Packages.openai`
- **THEN** quickpick SHALL resolve it to the expression file path (e.g. `pkgs/development/python-modules/openai/default.nix`) using the Nix evaluator.

#### Scenario: Attribute cannot be resolved
- **WHEN** the provided attribute path does not exist or cannot be mapped to a file
- **THEN** quickpick SHALL exit with a non-zero status and report the failure.

### Requirement: quickpick finds candidate commits on the source branch
The system SHALL identify commits on the configured source branch that modify the resolved package expression file.

#### Scenario: Source branch has package updates
- **WHEN** the source branch contains commits that touch the resolved expression file
- **THEN** quickpick SHALL collect those commits as candidates.

#### Scenario: Source branch has no matching commits
- **WHEN** the source branch has no commits modifying the resolved expression file
- **THEN** quickpick SHALL produce no candidate commits and exit successfully.

### Requirement: quickpick excludes commits already in the release branch
The system SHALL exclude candidate commits that are already reachable from the configured release branch.

#### Scenario: Candidate commit exists only on source branch
- **WHEN** a candidate commit is not an ancestor of the release branch HEAD
- **THEN** quickpick SHALL include it in the output.

#### Scenario: Candidate commit is already in release
- **WHEN** a candidate commit is an ancestor of the release branch HEAD
- **THEN** quickpick SHALL exclude it from the output.

### Requirement: quickpick emits cherry-pick commands
The system SHALL print one `git cherry-pick -x <commit-sha>` command per selected commit, in chronological order from oldest to newest.

#### Scenario: Multiple missing commits
- **WHEN** quickpick finds three commits on the source branch not present in the release branch
- **THEN** it SHALL print three lines, each formatted as `git cherry-pick -x <sha>`, ordered oldest first.

#### Scenario: No missing commits
- **WHEN** quickpick finds no commits to backport
- **THEN** it SHALL print nothing and exit successfully.

### Requirement: quickpick supports a dry-run mode
The system SHALL accept a `--dry-run` flag. When provided, it SHALL print a human-readable summary of the selected commits (SHA, date, and summary) instead of executable `git cherry-pick -x` commands.

#### Scenario: Dry-run with missing commits
- **WHEN** the user runs `quickpick --dry-run python3Packages.openai`
- **THEN** quickpick SHALL print one line per selected commit containing the SHA, date, and summary, and SHALL NOT print `git cherry-pick -x` commands.

#### Scenario: Dry-run with no missing commits
- **WHEN** the user runs `quickpick --dry-run python3Packages.openai` and no commits are selected
- **THEN** quickpick SHALL print nothing and exit successfully.

### Requirement: quickpick supports optional version filters
The system SHALL accept optional `--from VERSION` and `--to VERSION` arguments and filter candidate commits to those that change the package version within the specified range.

#### Scenario: Filter from a minimum version
- **WHEN** the user runs `quickpick --from 1.0.0 python3Packages.openai`
- **THEN** quickpick SHALL exclude candidate commits that introduce a version older than `1.0.0`.

#### Scenario: Filter to a maximum version
- **WHEN** the user runs `quickpick --to 2.0.0 python3Packages.openai`
- **THEN** quickpick SHALL exclude candidate commits that introduce a version newer than `2.0.0`.

#### Scenario: Filter from and to combined
- **WHEN** the user runs `quickpick --from 1.0.0 --to 2.0.0 python3Packages.openai`
- **THEN** quickpick SHALL include only candidate commits whose introduced version is greater than or equal to `1.0.0` and less than or equal to `2.0.0`.

### Requirement: quickpick reports errors clearly
The system SHALL produce human-readable error messages and exit with a non-zero status for invalid input, missing repositories, missing branches, or unresolvable attributes.

#### Scenario: Invalid attribute path
- **WHEN** the attribute path cannot be resolved
- **THEN** quickpick SHALL print an error message identifying the attribute path and exit with a non-zero status.

#### Scenario: Missing repository
- **WHEN** the specified `--repo` path is not a Git repository
- **THEN** quickpick SHALL print an error message indicating the path is not a valid repository and exit with a non-zero status.
