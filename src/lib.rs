#![doc(
    html_logo_url = "https://github.com/dr-montasir/gprs/raw/HEAD/logo.svg?sanitize=true",
    html_root_url = "https://docs.rs/gprs/latest/gprs"
)]

/// ### asyncore crate
/// Asyncore
/// ### version
/// 0.2.0
/// ### Modules
/// - async_std (crate): Asynchronous
/// ### Links
/// - [asyncore](https://crates.io/crates/asyncore)
/// - [documentation](https://docs.rs/asyncore/latest/asyncore)
/// - [on github.com](https://github.com/dr-montasir/asyncore)
pub mod asyncore;

/// ### cans crate
/// An Elegant Rust-based Literal Template Engine
/// ### version
/// 0.4.0
/// ### Modules
/// - html
/// - json
/// - mime
/// - rules
/// ### Links
/// - [cans](https://crates.io/crates/cans)
/// - [documentation](https://docs.rs/cans/latest/cans)
/// - [on github.com](https://github.com/dr-montasir/cans)
pub mod cans;

/// ### chief crate
/// Chief Development Tools provides versatile functionalities for managing web applications.
/// Depending on your requirements, you can choose between different installation methods.
/// Whether you want to utilize the command-line interface (CLI) for seamless application management
/// or integrate logging and environment variable handling into your project,
/// Chief offers flexible options tailored to your development needs.
/// This empowers developers to enhance their workflow and streamline application processes efficiently.
/// ### version
/// 0.5.0
/// ### Crates
/// - log
/// - simplelog
/// ### Modules
/// - env
/// ### Macros
/// - env
/// ### Functions
/// - chief_cli
/// - dotenv
/// ### Links
/// - [chief](https://crates.io/crates/chief)
/// - [documentation](https://docs.rs/chief/latest/chief)
/// - [on github.com](https://github.com/dr-montasir/chief)
pub mod chief;

/// ### ghttp crate
/// GHTTP (Global HTTP)
/// ### version
/// 0.1.0
/// ### Modules
/// - http (crate)
/// - status (module)
/// ### Links
/// - [ghttp](https://crates.io/crates/ghttp)
/// - [documentation](https://docs.rs/ghttp/latest/ghttp)
/// - [on github.com](https://github.com/dr-montasir/ghttp)
pub mod ghttp;

/// ### mathlab crate (math module)
/// A Powerful Math Library for Rust
/// ### version
/// 1.2.0
/// ### Modules
/// - math
/// ### Links
/// - [mathlab](https://crates.io/crates/mathlab)
/// - [documentation](https://docs.rs/mathlab/latest/mathlab)
/// - [on github.com](https://github.com/dr-montasir/mathlab)
pub mod math;

/// ### Parser Module
pub mod parser;

/// ### Runtime Module
pub mod runtime;
pub use runtime::*;

/// ### regexy crate
/// **REGEXY** provides a simple and lightweight Rust library for working with regular expressions.
/// The `regexy` crate provides an easy-to-use interface for matching patterns in strings using regex.
/// ### version
/// 0.2.0
/// ### Crates
/// - regex
/// ### Modules
/// - utils (regexy::utils)
/// ### Functions
/// - is_match
/// ### Links
/// - [regexy](https://crates.io/crates/regexy)
/// - [documentation](https://docs.rs/regexy/latest/regexy)
/// - [on github.com](https://github.com/dr-montasir/regexy)
pub mod regexy;

/// ### wtime crate
/// WTIME provides a variety of functions for obtaining the current UTC and local times, as well as generating customizable timestamps to suit your needs.
/// ### version
/// 0.6.0
/// ### Modules
/// - calc
/// - local
/// - tz
/// - utc
/// ### Links
/// - [wtime](https://crates.io/crates/wtime)
/// - [documentation](https://docs.rs/wtime/latest/wtime)
/// - [on github.com](https://github.com/dr-montasir/wtime)
pub mod wtime;
