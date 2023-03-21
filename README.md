# Bazel alias issue

Duplicated alias with renamed dependency, e.g.,

```
[dependencies]
merk_v1 = { package = "merk", git = "https://github.com/liftedinit/merk.git", rev = "857bf81963d9282ab03438da5013e1f816bd9da1" }
merk_v2 = { package = "merk", git = "https://github.com/MavenRain/merk.git", branch = "merk-hash-migration" }
```

# Build

```
$ CARGO_BAZEL_REPIN=workspace bazel sync --only=crate_index
ERROR: Traceback (most recent call last):
        File "/home/fmorency/.cache/bazel/_bazel_fmorency/5137f5256acc616f8d8003b1ed264a13/external/crate_index/defs.bzl", line 306, column 46, in <toplevel>
                "@crate_index__merk-2.0.0//:merk": "merk_v2",
Error: dictionary expression has duplicate key: "@crate_index__merk-2.0.0//:merk"
ERROR: error loading package '': initialization of module 'defs.bzl' failed
 checking cached actions
```
