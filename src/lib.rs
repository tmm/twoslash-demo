//! # Twoslash Rustdoc Demo
//!
//! This crate demonstrates **twoslash-style type annotations** in rustdoc.
//! Hover over identifiers in the code examples below to see their inferred types.
//!
//! ## Quick Example
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut scores = HashMap::new();
//! scores.insert("Alice", 95);
//! scores.insert("Bob", 87);
//!
//! let total: i32 = scores.values().sum();
//! let avg = total as f64 / scores.len() as f64;
//! ```
//!
//! ## How This Works
//!
//! The type annotations you see on hover are produced by a
//! [modified version of rustdoc](https://github.com/tmm/rustdoc-twoslash)
//! that integrates [`twoslash-rust`](https://github.com/wevm/vocs/tree/next/twoslash-rust),
//! a library that extracts type information from Rust code using rust-analyzer.
//!
//! ```text
//! Doc comment ````rust code blocks
//!         │
//!         ▼
//!    pulldown-cmark (markdown.rs)
//!         │
//!         ▼
//!    CodeBlocks iterator extracts code text
//!         │
//!         ▼
//!    twoslash.rs ──► twoslash-rust ──► rust-analyzer
//!         │              │                  │
//!         │              │    scaffolds a temp Cargo project,
//!         │              │    runs rust-analyzer, returns
//!         │              │    type info per identifier
//!         │              ◄──────────────────┘
//!         ▼
//!    highlight.rs merges annotations into DecorationInfo
//!         │
//!         ▼
//!    push_token() emits <span data-type="..."> attributes
//!         │
//!         ▼
//!    rustdoc.css renders hover popovers via CSS + JS
//! ```
//!
//! ### Source
//!
//! - **Rustdoc fork**: [`tmm/rustdoc-twoslash`](https://github.com/tmm/rustdoc-twoslash) — the modified `src/librustdoc/` files and a patch
//! - **twoslash-rust**: [`wevm/vocs/twoslash-rust`](https://github.com/wevm/vocs/tree/next/twoslash-rust) — rust-analyzer integration library
//! - **This demo**: [`tmm/twoslash-demo`](https://github.com/tmm/twoslash-demo)
//!
//! ### Build
//!
//! ```bash
//! RUSTDOC_TWOSLASH=1 \
//!   RUSTDOC=.../build/host/stage1/bin/rustdoc \
//!   RUSTC=.../build/host/stage1/bin/rustc \
//!   cargo doc --no-deps
//! ```

use std::collections::HashMap;

/// A strongly-typed configuration builder.
///
/// # Examples
///
/// ```rust
/// let config = twoslash_demo::Config::new("my-app")
///     .with_port(8080)
///     .with_debug(true);
///
/// let name = config.name();
/// let addr = config.address();
/// ```
pub struct Config {
    name: String,
    port: u16,
    debug: bool,
}

impl Config {
    /// Create a new configuration with the given application name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            port: 3000,
            debug: false,
        }
    }

    /// Set the port number.
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Enable or disable debug mode.
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Get the application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the full address string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = twoslash_demo::Config::new("app").with_port(9090);
    /// let addr = config.address();
    /// assert_eq!(addr, "0.0.0.0:9090");
    /// ```
    pub fn address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

/// Process a list of items, filtering and transforming them.
///
/// # Examples
///
/// ```rust
/// let words = vec!["hello", "world", "rust", "is", "great"];
/// let result = twoslash_demo::process_items(&words, 3);
///
/// let first = result.first();
/// let count = result.len();
/// ```
pub fn process_items(items: &[&str], min_len: usize) -> Vec<String> {
    items
        .iter()
        .filter(|s| s.len() >= min_len)
        .map(|s| s.to_uppercase())
        .collect()
}

/// A generic key-value store.
///
/// # Examples
///
/// ```rust
/// let mut store = twoslash_demo::Store::<String, Vec<i32>>::new();
/// store.set("scores".into(), vec![100, 95, 87]);
///
/// let scores = store.get("scores");
/// let has_key = store.contains("scores");
/// ```
pub struct Store<K, V> {
    data: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq, V> Store<K, V> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Insert a key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    /// Get a reference to a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
}

/// Demonstrates iterator chains and type inference.
///
/// # Examples
///
/// ```rust
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///
/// let evens: Vec<_> = numbers.iter()
///     .filter(|n| *n % 2 == 0)
///     .collect();
///
/// let sum: i32 = numbers.iter().sum();
///
/// let pairs: Vec<_> = numbers.iter()
///     .zip(numbers.iter().rev())
///     .take(3)
///     .collect();
/// ```
pub fn iterators_demo() {}

/// Working with `Result` and `Option` types.
///
/// # Examples
///
/// ```rust
/// fn parse_and_double(input: &str) -> Result<i64, std::num::ParseIntError> {
///     let parsed = input.parse::<i64>()?;
///     Ok(parsed * 2)
/// }
///
/// let ok_result = parse_and_double("21");
/// let err_result = parse_and_double("not_a_number");
///
/// let value = ok_result.unwrap();
/// let fallback = err_result.unwrap_or(0);
/// ```
pub fn error_handling_demo() {}

/// Closures and higher-order functions.
///
/// # Examples
///
/// ```rust
/// let multiply = |a: i32, b: i32| a * b;
/// let result = multiply(6, 7);
///
/// let add_n = |n: i32| move |x: i32| x + n;
/// let add_ten = add_n(10);
/// let sum = add_ten(32);
///
/// let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![
///     Box::new(|x| x + 1),
///     Box::new(|x| x * 2),
///     Box::new(|x| x - 3),
/// ];
///
/// let mut val = 10;
/// for op in &ops {
///     val = op(val);
/// }
/// ```
pub fn closures_demo() {}

/// Pattern matching and enums.
///
/// # Examples
///
/// ```rust
/// #[derive(Debug)]
/// enum Shape {
///     Circle(f64),
///     Rectangle(f64, f64),
///     Triangle { base: f64, height: f64 },
/// }
///
/// fn area(shape: &Shape) -> f64 {
///     match shape {
///         Shape::Circle(r) => std::f64::consts::PI * r * r,
///         Shape::Rectangle(w, h) => w * h,
///         Shape::Triangle { base, height } => 0.5 * base * height,
///     }
/// }
///
/// let shapes = vec![
///     Shape::Circle(5.0),
///     Shape::Rectangle(4.0, 6.0),
///     Shape::Triangle { base: 3.0, height: 8.0 },
/// ];
///
/// let areas: Vec<f64> = shapes.iter().map(area).collect();
/// let total: f64 = areas.iter().sum();
/// ```
pub fn pattern_matching_demo() {}

/// Trait objects and dynamic dispatch.
///
/// # Examples
///
/// ```rust
/// trait Greet {
///     fn hello(&self) -> String;
/// }
///
/// struct English;
/// struct Spanish;
///
/// impl Greet for English {
///     fn hello(&self) -> String { "Hello!".into() }
/// }
///
/// impl Greet for Spanish {
///     fn hello(&self) -> String { "¡Hola!".into() }
/// }
///
/// let greeters: Vec<Box<dyn Greet>> = vec![
///     Box::new(English),
///     Box::new(Spanish),
/// ];
///
/// let messages: Vec<String> = greeters.iter().map(|g| g.hello()).collect();
/// ```
pub fn trait_objects_demo() {}

/// Hex encoding with the `const-hex` crate.
///
/// # Examples
///
/// ```rust
/// let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe];
/// let hex_string = const_hex::encode(bytes);
/// let prefixed = const_hex::encode_prefixed(bytes);
///
/// let decoded = const_hex::decode("48656c6c6f").unwrap();
/// let as_str = std::str::from_utf8(&decoded).unwrap();
///
/// let mut buf = [0u8; 4];
/// const_hex::decode_to_slice("cafebabe", &mut buf).unwrap();
/// ```
pub fn hex_demo() {}

/// Combining `const-hex` with standard library types.
///
/// # Examples
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// let entries: Vec<(&str, &[u8])> = vec![
///     ("tx_hash", &[0x1a, 0x2b, 0x3c, 0x4d]),
///     ("block", &[0xff, 0x00, 0xaa, 0x55]),
///     ("nonce", &[0x00, 0x00, 0x00, 0x01]),
/// ];
///
/// let encoded: BTreeMap<&str, String> = entries
///     .iter()
///     .map(|(k, v)| (*k, const_hex::encode_prefixed(v)))
///     .collect();
///
/// let tx = encoded.get("tx_hash");
/// let all_keys: Vec<&&str> = encoded.keys().collect();
/// ```
pub fn hex_collections_demo() {}
