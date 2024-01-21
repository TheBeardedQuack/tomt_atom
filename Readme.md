[![MIT/Apache 2.0][icon-license]][link-license]
[![Crate][icon-crates.io]][link-crates.io]
[![Realease Doc][icon-docs.rs]][link-docs.rs]

# Brief
A basic Atom (string) registry for use with ID strings.

For when an application contains and passes around many constant strings (mainly de/serialized strings), this should reduce the overall memory footprint and slightly increase cache usage

> **Warning**: The performance implications of this crate should be tested to ensure it's a good fit for your use-case. There are many sitatuations where such a caching mechanism is simply unnecessary and this crate may easily harm performance due to the memory allocation required to register an atom, as well as the hash-lookup.

## Inspiration for this project:
- [Use Arc instead of Vec](https://www.youtube.com/watch?v=A4cKi7PTJSs&ab_channel=LoganSmith) - Logan Smith (YouTube/[@_noisecode]((https://www.youtube.com/@_noisecode)))
- [Atom Tables](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables) - Microsoft Windows

# Usage

Atoms can be created directly and either passed by ref into functions, or cloned to avoid Rust's ownership semantics.

```rust
use tomt_atom::Atom;

fn construct_example()
{
    // Create atoms directly with `new()` constructor
    let atom = Atom::new("My &'static str example");

    // `new()` accepts any type that implements `AsRef<str>`
    let atom = Atom::new(String::new("My owned String example"));
}

fn convert_example()
{
    // Atom supports `From<&str>` and `From<String>``
    let atom: Atom = "Another &'static str example".into();

    // This should help if you need pass atoms directly
    let result = function_accepting_atom("Quick convert example".into())
}

```

Ideally atoms should be registered into a global table. This will provide lookups to return an existing atom if present, or create one if not. Atoms on there own may reduce the overall memory footprint, but with a registry the same string will always yield the same atom.

```rust
use tomt_atom::*;

fn global_registry()
{
    // Use built-in global registry
    let accept = AtomRegistry::global().register("application/json");
    
    // Create your own registry to pass via a service
    let mimes = AtomRegistry::new();
    let accept = mimes.register("application/json");

    // Unregistering will remove the entry in the lookup table, but all existing
    // Atoms will remain valid, and may continue to be cloned and passed around
    mimes.unregister("application/json");
}
```

Registries are indirect, support `Send`/`Sync` and can be cheaply cloned.
Cloned registries point to the same atom table and updates in one will be reflected in both.

# Changelog

| Version | Changes         |
|:-------:|-----------------|
|  0.1.0  | Initial-release |
|  0.1.1  | Fixed potential hash-collision bug |
|  0.1.2  | Updated readme usage. Improved construct semantics |
|  0.1.3  | Added `Display` trait to `Atom` |
|  0.1.4  | Added `PartialEq` and `Eq` traits to `Atom` |
|  0.1.5  | Added feature "serde" to implement `Deserialize`/`Serialize` |
|  0.1.6  | Added `Default` trait to `Atom` and `AtomRegistry` |
|  0.1.7  | Added infallible `FromStr` trait to `Atom` (I need this for use in higher projects, so it's via feature "from_str"). Added `Hash` trait to `Atom` |

# Contributing

Got some idea, feedback, question or found a bug? Feel free to open an issue at any time!

# License

TOMT_Atom is dual-licensed under either:

* MIT License (in [repository][license-MIT-local] or from [source][licence-MIT-remote])
* Apache License, Version 2.0 (in [repository][license-Apache-local] or from [source][license-MIT-remote])

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are good reasons to include both.

<!-- Icons -->
[icon-license]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg 
[icon-crates.io]: https://img.shields.io/crates/v/tomt_atom.svg
[icon-docs.rs]: https://docs.rs/tomt_atom/badge.svg

<!-- Licenses -->
[license-MIT-local]: LICENSE-MIT
[license-MIT-remote]: https://opensource.org/licenses/MIT
[license-Apache-local]: LICENSE-APACHE
[license-Apache-remote]: https://www.apache.org/licenses.LICENSE-2.0

<!-- Links -->
[link-license]: https://github.com/TheBeardedQuack/tomt_bevycss#license
[link-crates.io]: https://crates.io/crates/tomt_atom
[link-docs.rs]: https://docs.rs/tomt_atom
