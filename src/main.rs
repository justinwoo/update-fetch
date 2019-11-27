use rayon::prelude::*;
use std::{env, fs, io, path::Path, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.is_empty() {
        println!("{}", EXPECT_FILE_PATH_ARG_MSG.trim());
        process::exit(1);
    }

    args.par_iter()
        .map(|x| {
            let path = Path::new(x);
            let result = main1(path);
            match result {
                Ok(_) => println!("Finished {}", x),
                Err(e) => {
                    println!("Error on {}: {}", x, e);
                }
            }
        })
        .collect::<()>();
}

const EXPECT_FILE_PATH_ARG_MSG: &str = r#"
Need arguments for what in_files to process.
Usage Examples:
    # update a single in_file
    update-fetch-derivation my-in_file.nix
    # Using fd (sequential)
    fd -e nix -x update-fetch-derivation {}
    # Multiple files
    update-fetch-derivation *.nix
"#;

fn main1(file: &Path) -> io::Result<()> {
    let input = fs::read_to_string(file)?;

    let ast = rnix::parse(&input);
    let root_node = ast.node();
    let result = update_fetch::format(&root_node).to_string();

    let output = nixpkgs_fmt::reformat_string(&result);
    if input != output {
        fs::write(file, &output)?;
    }
    Ok(())
}
