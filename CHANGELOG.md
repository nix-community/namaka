# Changelog

## v0.2.1 - 2024-11-16

### Fixes

- review: reset the terminal when interrupted

## v0.2.0 - 2023-06-02

### Breaking Changes

- `namaka review` now runs `nix eval .#checks` by default instead of `nix flake check`.
  You can override this behavior by editing `namaka.toml` or using the `--cmd` flag.
- When provided a directory, namaka now changes the working directory to it,
  instead of modifying the `nix` commands it runs.
  This can make a difference if you specify both the directory and `--cmd`.

### Features

- Local configuration with `namaka.toml`
- `namaka clean` to remove unused and pending snapshots
- Add `subflake` template for using namaka in a subflake

### Improvements

- `namaka review` now runs faster by default,
  since the default command has been changed to `nix eval .#checks`.

## v0.1.1 - 2023-05-02

### Features

- load: accept `src`
- cli: allow running commands other than `nix flake check`

### Changes

- load: deprecate `flake` and `dir`

### Fixes

- load: fix json format when values are implicitly casted
- canonicalize path before running `nix flake check`

## v0.1.0 - 2023-04-09

First release
