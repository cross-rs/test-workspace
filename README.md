# `test-workspace`

> Minimal example of cargo workspaces

Run _anywhere_ inside the `workspace` directory:

```
cross run -p binary
```

This should also work from the root directory:

```
cross run --manifest-path ./workspace/Cargo.toml -p binary
```

## Specification

This aims to replicate most of the features present in
the reference [documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html).

AKA, we support:
- globs
- excludes

The glob syntax is described in detail [here](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html).  

In short:
- `?`: any single character.
- `*`: 0 or more characters.
- `**`: current directory and recursive subdirectories.
  - `**b` and `a**` are both invalid: it must be just `**`
- `[...]`: matches character in the set, such as `[0-9]`
- `[!...]`: matches character not in the set, such as `[!0-9]`

These syntaxes don't apply on top of each other, like more sophisticated
regular expressions: they are simply globs. For example, `[0-9]?` matches
`1f`, but not `1`.

Adding in a non-glob member, such as `"."` for the root overrides all
exclude patterns. For example, having a workspace like the following will
ignore the exclude filter.

```toml
[workspace]
members = ["folder", "folder/*"]
exclude = ["folder/lib1"]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
