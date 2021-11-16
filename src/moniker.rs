pub trait Moniker {
    /// Returns a random moniker.
    /// The random function is **not secure** nor cryptographically strong.
    fn get_random() -> String;
}
