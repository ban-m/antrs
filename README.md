
# Antrs: Read H5AD with your favorite struct

[![CI](https://github.com/ban-m/antrs/actions/workflows/ci.yml/badge.svg)](https://github.com/ban-m/antrs/actions/workflows/ci.yml)

**antrs** is a Rust library for reading [AnnData](https://anndata.readthedocs.io/en/latest/) files (HDF5 format) used in single-cell RNA-sequencing analysis.

It provides a flexible, trait-based API for mapping AnnData fields to user-defined Rust structs.

## Features

- Read AnnData `.h5ad` files using [hdf5-metno](https://crates.io/crates/hdf5-metno)
- Trait-based design: map obs, var, layers, etc. to your own Rust types
- Type-safe and extensible
- Handles dynamic, DataFrame-like AnnData fields

## Example Usage

```rust
use antrs::{AnnField, AnnRecord};
use std::collections::HashMap;

// Define your own struct for obs
struct MyObs {
    cell_id: String,
    batch: i32,
}

// Implement the AnnRecord trait
impl AnnRecord for MyObs {
    fn from_map(fields: &HashMap<String, AnnField>) -> Self {
        let cell_id = match fields.get("cell_id") {
            Some(AnnField::String(v)) => v[0].clone(),
            _ => "".to_string(),
        };
        let batch = match fields.get("batch") {
            Some(AnnField::Int(v)) => v[0],
            _ => 0,
        };
        MyObs { cell_id, batch }
    }

    fn to_map(&self) -> HashMap<String, AnnField> {
        let mut map = HashMap::new();
        map.insert("cell_id".to_string(), AnnField::String(vec![self.cell_id.clone()]));
        map.insert("batch".to_string(), AnnField::Int(vec![self.batch]));
        map
    }
}
```

## Getting Started

1. Add to your `Cargo.toml`:
    ```toml
    hdf5-metno = "0.10.1"
    ```

2. Implement your own structs and the `AnnRecord` trait.

3. Use the library to read AnnData files and map fields to your types.

## License

MIT

## Acknowledgements
