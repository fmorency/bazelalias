# Bazel alias issue

Duplicated alias with renamed dependency, e.g.,

```
[dependencies]
rand = "0.8"
rand-07 = { package = "rand", version = "0.7" }
```

# Build

```
$ bazel build //...
```
