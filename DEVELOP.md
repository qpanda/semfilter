Developer Guide
================
## Branching Model
**semfilter** follows the [Gitflow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) branching model. All changes should be made on `feature` branches, `develop` will act as the **stable** branch, and `master` will contain only release versions.

`feature` branches should follow the naming convention `<initials>/<feature>` where `<initials>` is the initials of the developer making the change and `<feature>` is a meaningful short description of the feature. Before changes are merged to `develop` a pull request should be raised and at least one approval should be obtained.

## Git Usage
Git [rebase is preferred over merge](https://www.atlassian.com/git/tutorials/merging-vs-rebasing) and should be used whenever possible. Strive for a clean commit history with [meaningful commit messages](https://chris.beams.io/posts/git-commit/) according to the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## Versioning
**semfilter** uses [Semantic Versioning](https://semver.org/)

## Rust
**semfilter**  is written in Rust. It uses [cargo](https://doc.rust-lang.org/cargo/index.html) as the build tool and [Rustfmt](https://github.com/rust-lang/rustfmt) to keep the coding sytle consistent.

### Error Handling
**semfilter** uses [Anyhow](https://crates.io/crates/anyhow) for error handling. All fallible functions return `Result<T, anyhow::Error>` with additional context added when appropriate.

> **Note**: For more details on error handling in Rust see [Error Handling In Rust - A Deep Dive](https://www.lpalmieri.com/posts/error-handling-rust/), [Rust: Structuring and handling errors in 2020](https://nick.groenen.me/posts/rust-error-handling/), [Beginner's guide to Error Handling in Rust](https://www.sheshbabu.com/posts/rust-error-handling/), the [Error Handling](https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html) chapter in [A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html), the [Error Handling](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html) chapter in [The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html), and [Error handling in Rust](https://fettblog.eu/rust-error-handling/).

## Outstanding Issues
There are still many [outstanding issues](https://github.com/qpanda/semfilter/issues) that need to be addressed before **semfilter** reaches version `1.0.0`.