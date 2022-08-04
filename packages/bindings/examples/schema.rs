use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use neutron_bindings::msg::NeutronMsg;
use neutron_bindings::query::InterchainQueries;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
    export_schema(&schema_for!(NeutronMsg), &out_dir);
    export_schema(&schema_for!(InterchainQueries), &out_dir);
}
