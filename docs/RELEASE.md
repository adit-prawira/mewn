# Making a Release

## 1. Create release branch

```bash
git checkout -b release/0.1.0
```

## 2. Verify

```bash
cargo build --release
cargo test --all
cargo clippy --all-targets -- -D warnings
```

## 3. Bump version

Update the version string in both files:

- `Cargo.toml` → `version = "0.1.0"`
- `Formula/mewn.rb` → `version "0.1.0"` and both `url` fields

Commit the version bump.

## 4. Tag and push

```bash
git tag v0.1.0
git push origin release/0.1.0 --tags
```

## 5. CI publishes GitHub Release

`.github/workflows/release.yml` triggers on the `v0.1.0` tag. It builds two macOS binaries:

| Target | Runner |
|--------|--------|
| `aarch64-apple-darwin` | macos-latest (Apple Silicon) |
| `x86_64-apple-darwin` | macos-latest (Intel) |

Artifacts attached to the release:
- `mewn-v0.1.0-aarch64-apple-darwin.tar.gz` + `.sha256`
- `mewn-v0.1.0-x86_64-apple-darwin.tar.gz` + `.sha256`

## 6. Update formula SHA256

The formula ships with `"PLACEHOLDER_ARM64"` and `"PLACEHOLDER_X86_64"` — these block installs until replaced.

1. Go to the GitHub Release page for `v0.1.0`
2. Copy the SHA256 from each `.sha256` file
3. Paste into `Formula/mewn.rb` replacing the placeholders
4. Commit and push

```bash
# Verify the formula
brew tap adit-prawira/mewn
brew install mewn
mewn version
```

## 7. Merge release branch

```bash
git checkout main
git merge release/0.1.0
git push origin main
```

## One-time setup (first release only)

The first release also needs the workflow, formula, and README update committed to main before the release branch is cut. After that, future releases only need steps 1–7.
