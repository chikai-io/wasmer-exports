use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Path to wasm file.
    /// If the `recursive` flag is set, must be a directory path.
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Whether to recursively search and show functions from wasm files
    /// found in the `path`.
    #[structopt(short, long)]
    pub recursive: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;
    if args.recursive {
        let path = args.path.canonicalize()?.join("**").join("*.wasm");
        for entry in glob::glob(path.to_string_lossy().as_ref())? {
            let entry = entry?;
            show_wasm_functions(entry.as_ref());
        }
    } else {
        show_wasm_functions(args.path.as_ref());
    }

    Ok(())
}

fn show_wasm_functions(path: &std::path::Path) {
    println!("file: {}", path.to_string_lossy());
    for function in wasmer::Module::from_file(&wasmer::Store::default(), path)
        .unwrap()
        .exports()
        .filter(|e| matches!(e.ty(), wasmer::ExternType::Function(_fty)))
    {
        println!("{}", function.name());
    }
    println!();
}
