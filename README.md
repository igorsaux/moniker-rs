# Moniker-rs

Moniker-rs - is a port of [.NET library](https://github.com/alexmg/Moniker) for generating names with no dependencies.

At the moment there is two types of monikers: `Moby` and `Animal`.

- Animal: generates names like: `dapper-ladybird`, `callous-bear`.
- Moby: generates names like: `focused-lovelace`, `determined-curie`.

## Examples

### Libray

```rust
extern crate moniker_rs;
use moniker_rs::{Moby, Animal, Moniker};

// Returns something like "brave-archimedes"
Moby::get_random();
// Returns an array of names ["aardvark", "aardwolf", ...]
Animal::get_names();
```

See more examples [here](./examples/).

### CLI

Creates a random moniker.
To use the bin crate - pass one of a monikers type as an argument (case insensitive):

```sh
$ moniker-cli animal
mollified-camel
```

or

```sh
$ moniker-cli moby
hardcore-rosalind
```
