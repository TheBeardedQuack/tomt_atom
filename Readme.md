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

To use TOMT_BevyCSS just add a `StyleSheet` with a loaded `css` file to any entity and all style sheet rules will be applied to the entity and _all_ its [`descendants`][80] (children of children of children and so on).

```rust
use tomt_atom::Atom;

fn example_create()
{
    // Create atom directly with constructor
    let atom = Atom::new("From constructor");

    // Convert from any &str
    let atom: Atom = "From string".into();

    // Including String, or any &str-like object (impls AsRef<str>)
    let string = String::new("Example string");
    let atom = string.into();
}
```

# Changelog

| Version | Changes         |
|:-------:|-----------------|
|  0.1.0  | Initial-release |

# Contributing

Got some idea, feedback, question or found a bug? Feel free to open an issue at any time!

# License

TOMT_Atom is dual-licensed under either:

* MIT License (in [repository][license-MIT-local] or from [source][licence-MIT-remote])
* Apache License, Version 2.0 (in [repository][license-Apache-local] or from [source][license-MIT-remote])

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons][55] to include both.

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
