# Release Process

`atext` is in an early bootstrap phase and currently uses a straightforward manual release process.

## Release Contract

- Versioning follows [Semantic Versioning](https://semver.org/).
- `Cargo.toml` is the canonical version source.
- Git tags should use the `vX.Y.Z` form.
- Until the crate has a meaningful external API, releases should stay conservative and explicit.

## Release Checklist

### 1. Finalize the version

Update the crate version in `Cargo.toml`.

### 2. Review the public contract

If the release changes behavior or positioning, review and update:

- [README.md](README.md)
- [CONSTITUTION.md](CONSTITUTION.md)
- [ARCHITECTURE.md](ARCHITECTURE.md)
- [GUIDE.md](GUIDE.md)
- [CONFIGURATION.md](CONFIGURATION.md)

### 3. Run the checks

```bash
just check
just flake-check
```

If flake inputs changed during the release:

```bash
nix flake lock
```

### 4. Commit the release

Create an atomic release commit once version and docs are in place.

### 5. Tag the release

```bash
git tag v0.1.0
git push origin main --tags
```

### 6. Decide publication explicitly

The crate is currently marked `publish = false`. If that changes, update `Cargo.toml` deliberately before attempting a crates.io release.

## Post-release Checks

After tagging:

1. Verify the tag matches `Cargo.toml`.
2. Verify the foundational docs still describe the released state accurately.
3. Verify the flake and local command surface still work from a clean checkout.

## Notes

- Keep release commits atomic.
- Keep docs and version changes in the same release slice.
- If release automation is added later, replace this document rather than letting it silently rot.
