# Bazel alias issue

Duplicated alias with renamed dependency, e.g.,

```
[dependencies]
rand = "0.8"
rand-07 = { package = "rand", version = "0.7" }
```

# Build

```
$ cd bazelalias
$ cargo rake
$ bazel build :bazelalias
...
Error in alias: alias rule 'rand' in package 'cargo' conflicts with existing alias rule, defined at /bazelalias/cargo/BUILD.bazel:15:6
```
