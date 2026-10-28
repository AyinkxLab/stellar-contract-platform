//! Generate the deterministic WASM fixtures used by `demo/`.
//!
//! Run from the repository root:
//!
//! ```bash
//! cargo run -p observatory-testutil --example make_demo_fixtures
//! ```
//!
//! The fixtures are checked in so the demo does not need to rebuild them.

use std::fs;

use observatory_testutil::module_with_spec;
use observatory_testutil::spec::{spec_function, spec_struct, ty_udt};
use stellar_xdr::ScSpecTypeDef;

fn main() {
    let old = vec![
        spec_function(
            "transfer",
            &[("to", ty_udt("Recipient")), ("amount", ScSpecTypeDef::I128)],
            Some(ScSpecTypeDef::Bool),
        ),
        spec_function(
            "balance",
            &[("owner", ty_udt("Recipient"))],
            Some(ScSpecTypeDef::I128),
        ),
        spec_struct("Recipient", &[("address", ScSpecTypeDef::Address)]),
    ];

    let new = vec![
        spec_function(
            "transfer",
            &[("to", ty_udt("Recipient")), ("amount", ScSpecTypeDef::I128)],
            Some(ScSpecTypeDef::I128),
        ),
        spec_function(
            "mint",
            &[("to", ty_udt("Recipient")), ("amount", ScSpecTypeDef::I128)],
            None,
        ),
        spec_struct("Recipient", &[("address", ScSpecTypeDef::Address)]),
    ];

    fs::create_dir_all("demo/fixtures").expect("create demo/fixtures");
    fs::write("demo/fixtures/old.wasm", module_with_spec(&old)).expect("write old.wasm");
    fs::write("demo/fixtures/new.wasm", module_with_spec(&new)).expect("write new.wasm");
    println!("wrote demo/fixtures/old.wasm and demo/fixtures/new.wasm");
}
