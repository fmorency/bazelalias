load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "b4e622a36904b5dd2d2211e40008fc473421c8b51c9efca746ab2ecf11dca08e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.19.1/rules_rust-v0.19.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

RUST_VERSION = "nightly/2023-03-21"

rust_register_toolchains(
    edition = "2021",
    versions = [RUST_VERSION],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.Bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = [
        "//:Cargo.toml",
    ],
    rust_version = RUST_VERSION,
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
