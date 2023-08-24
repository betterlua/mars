use clap::{Arg, Command};
use mar::{BuildOptions, BuildOutputType, BuildType};

pub fn main() {
    let args = Command::new("mar")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Bundle a bunch of lua files into a single lua file")
        .arg(
            Arg::new("entry")
                .short('e')
                .long("entry")
                .value_name("entry")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("input_dir")
                .short('i')
                .long("input-dir")
                .value_name("input dir")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .num_args(1),
        )
        .get_matches();

    let entry_point = args.get_one::<String>("entry").unwrap().clone();
    let input_dir = args.get_one::<String>("input_dir").unwrap().clone();
    let output_path = args.get_one::<String>("output").unwrap().clone();
    let mut path = output_path.split("/").collect::<Vec<&str>>();
    let output_name = path.remove(path.len() - 1);
    let path = path.join("/");

    println!("entry_point: {}", entry_point);
    println!("input_dir: {}", input_dir);
    println!("output_path: {}", output_path);
    println!("output_name: {}", output_name);
    println!("path: {}", path);

    let build_opts = BuildOptions {
        entry_point,
        src_dir: input_dir,
        r#type: BuildType::Debug,
        output_type: BuildOutputType::Binary,
        output_path: path,
        output_name: output_name.to_string(),
    };

    mar::build(build_opts).unwrap();
}
