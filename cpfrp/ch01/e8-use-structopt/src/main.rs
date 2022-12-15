use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Activate verbose mode
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// File to generate
    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    /// Files to pracess
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    println!("{:#?}", Opt::from_args());
}

// cargo run input1.txt input2.txt -v --result res.xyz
// Opt {
//     verbose: true,
//     result_file: "res.xyz",
//     files: [
//         "input1.txt",
//         "input2.txt",
//     ],
// }
