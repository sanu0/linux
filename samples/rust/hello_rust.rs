// SPDX-License-Identifier: GPL-2.0

//! My first Rust kernel module.


use kernel::prelude::*;

module! {
    type: HelloRust,
    name: "hello_rust",
    authors: ["Kumar Sanu"],
    description: "My First Rust kernel module",
    license: "GPL",
}

struct HelloRust;

impl kernel::Module for HelloRust {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, World! From Rust, in ring 0. \n");
        Ok(HelloRust)

    }
}
