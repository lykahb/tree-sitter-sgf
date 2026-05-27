# Release

This project publishes the same version to npm, crates.io, and PyPI. Publish
manually and keep the registries in sync.

## Update versions

Choose the next version and update it in all package manifests. For an exact
version:

```sh
npm version X.Y.Z --no-git-tag-version
cargo set-version X.Y.Z
uv version X.Y.Z
```

For a patch bump:

```sh
npm version patch --no-git-tag-version
cargo set-version --bump patch
uv version --bump patch
```

`npm version` updates `package.json` and `package-lock.json`. `cargo
set-version` is provided by `cargo-edit`; install it once with:

```sh
cargo install cargo-edit
```

Review that all registries now have the same version:

```sh
rg '"version":|^version =' package.json Cargo.toml pyproject.toml
```

Refresh lockfiles and regenerate the parser if dependencies or the grammar
changed:

```sh
npm_config_cache=.npm-cache npm install --package-lock-only --ignore-scripts
cargo update
npm run generate
```

If the Tree-sitter CLI changes, rebuild the local CLI binary before generating:

```sh
npm_config_cache=.npm-cache npm update tree-sitter-cli --ignore-scripts
npm_config_cache=.npm-cache npm rebuild tree-sitter-cli
npm run generate
```

## Validate

Run the grammar, binding, editor, and packaging checks before publishing:

```sh
npm run check
npm run build:parser
npm run check:bindings
npm run smoke:nvim
npm run health:helix
npm_config_cache=.npm-cache npm pack --dry-run
cargo package --allow-dirty --list
cargo publish --dry-run --allow-dirty
UV_CACHE_DIR=.uv-cache uv run --no-project --with build --with twine python -m build
UV_CACHE_DIR=.uv-cache uv run --no-project --with twine python -m twine check dist/*
```

Review `cargo package --allow-dirty --list` carefully. It should not include
local caches, virtual environments, `node_modules`, `build`, `dist`, or
`target`.

## Publish

Make sure each registry account is logged in before publishing:

```sh
npm_config_cache=.npm-cache npm whoami
cargo login
```

Publish npm first:

```sh
npm_config_cache=.npm-cache npm publish --access public
npm view tree-sitter-sgf version
```

Publish crates.io:

```sh
cargo publish --allow-dirty
cargo info tree-sitter-sgf
```

Publish PyPI:

```sh
UV_CACHE_DIR=.uv-cache uv run --no-project --with build --with twine python -m build
UV_CACHE_DIR=.uv-cache uv run --no-project --with twine python -m twine upload dist/*
pip index versions tree-sitter-sgf
```

## After publishing

Commit generated artifacts, lockfile changes, and version updates together:

```sh
git status --short
git add package.json package-lock.json Cargo.toml Cargo.lock pyproject.toml src
git commit -m "Release vX.Y.Z"
```

Tag the release after all registries show the new version:

```sh
git tag vX.Y.Z
git push origin vX.Y.Z
```
