use extism_pdk::*;
use serde::{Deserialize, Serialize};
//use std::process::Command;

// start with something simple
#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    //let out = Command::new("echo").arg("HWE").output().expect("Failed");
    Ok(format!("Hello, {}!, here's the results", name))
}

// use json data for inputs and outputs
#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
struct Add {
    left: i32,
    right: i32,
}
#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
struct Sum {
    value: i32,
}

#[plugin_fn]
pub fn add(input: Add) -> FnResult<Sum> {
    Ok(Sum {
        value: input.left + input.right,
    })
}
