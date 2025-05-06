// Copyright (c) The Hummanta Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod args;

use std::{fs, process};

use anyhow::{anyhow, Context as _, Result};
use clap::Parser as _;
use cranelift::{
    codegen::{print_errors::pretty_error, Context},
    module::{default_libcall_names, Linkage, Module},
    native::builder,
    object::{ObjectBuilder, ObjectModule},
    prelude::settings,
};
use cranelift_reader::{parse_test, ParseOptions};

use self::args::Args;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    let flag = settings::Flags::new(settings::builder());
    let isa = builder().unwrap().finish(flag).unwrap();

    let name = args.output.file_name().and_then(|s| s.to_str()).unwrap_or("a.out");
    let builder = ObjectBuilder::new(isa.clone(), name, default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);

    let name = String::from(args.input.as_os_str().to_string_lossy());
    let source = fs::read_to_string(&args.input)
        .context(format!("Failed to read input file: {}", args.input.display()))?;
    let test_file = parse_test(&source, ParseOptions::default())
        .with_context(|| format!("failed to parse {name}"))?;

    for (func, _) in test_file.functions {
        let mut context = Context::for_function(func);

        // Compile
        let _ = context
            .compile(isa.as_ref(), &mut Default::default())
            .map_err(|err| anyhow!("{}", pretty_error(err.func, err.inner)))?;

        let name = context.func.name.to_string();
        let fid = module.declare_function(&name, Linkage::Export, &context.func.signature)?;
        module.define_function_with_control_plane(fid, &mut context, &mut Default::default())?;
    }

    let bytes = module.finish().emit()?;
    fs::write(args.output, bytes)?;

    Ok(())
}
