fn file<P: AsRef<Path>>

Returns a Data provider that sources its values by parsing the file at
path as format F. If path is relative, the file is searched for in
the current working directory and all parent directories until the root,
and the first hit is used.

Nesting is not enabled by default; use [Data::nested()] to enable
nesting.

use serde::Deserialize;
use figment::{Figment, Jail, providers::{Format, Toml}, value::Map};

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    numbers: Vec<usize>,
    untyped: Map<String, usize>,
}

Jail::expect_with(|jail| {
    jail.create_file("Config.toml", r#"
        numbers = [1, 2, 3]
Truncated...
