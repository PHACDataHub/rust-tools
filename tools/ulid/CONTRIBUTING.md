# Contributing to Our Project

We're excited that you're interested in contributing to our project! This document outlines the process for contributing code and explains our release strategy and branch naming conventions.

## Branch Naming Conventions

To keep our repository organized and to automate our deployment processes, we have specific naming conventions for our branches:

### Production Branches

- **Naming**: Branches intended for production releases should contain the keyword `prod`. For example:
  - `release-prod`
  - `feature-X-prod`

### Beta Branches

- **Naming**: Branches intended for beta testing should contain the keyword `beta`. Examples include:
  - `beta`
  - `feature-X-beta`
  - `beta-Q1-2024`

Using these keywords in your branch names will trigger specific workflows and treat releases appropriately according to the branch type.

## Release Process

Our release process is automated through GitHub Actions and depends on the type of branch:

### Production Releases

- **Trigger**: Merging into a production branch (any branch containing the keyword `prod`).
- **Behavior**: The merge triggers a GitHub Action that builds the project, runs tests, and creates a release without the `-beta` suffix. These releases are considered stable and are not marked as pre-releases.

### Beta Releases

- **Trigger**: Merging into a beta branch (any branch containing the keyword `beta`).
- **Behavior**: Similar to production, but the release is tagged with `-beta` and marked as a pre-release in GitHub. This indicates that the release is not yet considered stable and is meant for testing purposes.

### General Guidelines for Releases

1. **Versioning**: Use semantic versioning. For production releases, increment the major or minor version as appropriate. For beta releases, increment the patch level or use identifiers like `beta1`, `beta2`, etc.
2. **Tagging**: Tags should follow the format `ulid-vX.Y.Z` for production and `ulid-vX.Y.Z-beta` for beta releases.
3. **Draft Releases**: If a release requires manual approval, create a draft release first, review it, then publish it.

## How to Contribute

1. **Fork the Repository**: Start by forking the repository to your own GitHub account.
2. **Create a Branch**: Based on the guidelines above, create a branch in your fork.
3. **Make Changes**: Implement your changes, ensuring you follow our coding standards.
4. **Write Tests**: Add tests that cover your changes.
5. **Run Tests**: Ensure all tests pass.
6. **Open a Pull Request**: Submit a pull request to the main repository. Ensure your PR clearly describes the changes and reference any related issues.
7. **Review**: Wait for review and address any comments raised by reviewers.

Thank you for contributing to our project! We look forward to reviewing your contributions and engaging with you in our community.
