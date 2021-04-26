//
// Copyright (c) 2019 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

fn main() {
    //println!("cargo:rustc-link-search=native=/opt/pmdk-1.6.1/lib");
    println!("cargo:rustc-link-search=native=/home/junghan/workspace/lib/pmdk/src/debug/");
    println!("cargo:rustc-link-lib=pmemobj");
}
