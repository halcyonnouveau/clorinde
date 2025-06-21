### Before submitting the PR, please make sure you do the following

- [ ] Prefix your PR title with `feat:`, `fix:`, `chore:`, or `docs:`.
- [ ] This message body should clearly illustrate what problems it solves.
- [ ] If this PR changes code within `src`, run codegen for the repo `cargo run --package test_integration -- --apply-codegen`

### Tests and linting

- [ ] Run formatting with `cargo fmt`
- [ ] Run the tests with `cargo test --all` and lint the project with `cargo clippy`
