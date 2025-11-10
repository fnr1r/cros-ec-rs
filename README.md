# cros-ec-rs

Rust crate for interfacing with the ChromeOS Embedded Controller (EC),
primarily targeting the Framework Laptop.

NOTE: This crate remains unversioned until I decide it has developed enough.

## Focus

Make a safe wrapper for the EC, roughly functionally equivalent to `ectool` as
a crate. I'll be focusing on the DEV V2 implementation, since it's what
Framework uses (and I can't check it with anything else).

## Usage

1. Add it to your Cargo.toml

    ```toml
    [dependencies.cros-ec]
    git = "https://github.com/fnr1r/cros-ec-rs"
    rev = "<REVISION_HERE>"
    ```

2. For examples, see the [examples](./examples) directory

## References

* DHowett's fork of [ectool](https://gitlab.howett.net/DHowett/ectool)
* The official
    [framework-system](https://github.com/FrameworkComputer/framework-system)
    repo  
    (which takes a different approach to the EC)
    (mainly using `nix` instead of `rustix`)

## Disclaimer

I'm not responsible for any thermo-nuclear war.

<!--
I bought a Framework Laptop before Framework endorsed Omarchy and DHH by proxy.
-->
