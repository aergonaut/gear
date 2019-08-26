# `gear`

`gear` is a CLI tool to improver developer QOL.

## Installation

### From source

1. Clone the repository.
2. Build with `cargo build --release`
3. Move `target/release/gear` somewhere in your path

## Configuration

`gear` will search for a `gear.toml` file, starting in the current directory and
walking back to the root directory, to read configuration values. The first
file that is found is the one that will be used.

Additionally, a global configuration file is used to store other properties. It
is stored in a canonical location, according to the platform:

- on Linux: `$XDG_CONFIG_HOME/gear/config.toml` or `$HOME/.config/gear/config.toml`
- on Windows: `{FOLDERID_RoamingAppData}/gear/config/config.toml`
- on macOS: `$HOME/Library/Preferences/gear/config.toml`

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.

![gears](gears.jpg)
