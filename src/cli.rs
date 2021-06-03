use std::path::PathBuf;
use structopt::StructOpt;

/// CLI for stress testing in competitive programming contest
#[derive(StructOpt, Debug)]
#[structopt(name = "quicktest")]
pub enum Opt {

    /// Check TLE
    #[structopt(help = "Check that <target-file> does not have the TLE status using random test cases generated by <gen-file>")]
    TLE {
        /// Target File
        #[structopt(short = "t", long = "target-file", parse(from_os_str))]
        target_file: PathBuf,

        /// Generator File
        #[structopt(short = "g", long = "gen-file", parse(from_os_str))]
        gen_file: PathBuf,

        /// Timeout TLE
        #[structopt(short = "o", long = "timeout", default_value = "2000")]
        timeout: i32,

        /// Number of test cases
        #[structopt(short = "n", long = "test-cases", default_value = "10000")]
        test_cases: i32,
    },
    /// Compare correctness with a slower program
    #[structopt(help = "Check the correctness of the <target-file> comparing it with <slow-file> with input test generated by <gen-file>")]
    Compare {
        /// Target File
        #[structopt(short = "t", long = "target-file", parse(from_os_str))]
        target_file: PathBuf,

        /// Slow File
        #[structopt(short = "s", long = "slow-file", parse(from_os_str))]
        slow_file: PathBuf,

        /// Generator File
        #[structopt(short = "g", long = "gen-file", parse(from_os_str))]
        gen_file: PathBuf,

        /// Timeout TLE
        #[structopt(short = "o", long = "timeout", default_value = "2000")]
        timeout: i32,

        /// Number of test cases
        #[structopt(short = "n", long = "test-cases", default_value = "10000")]
        test_cases: i32,
    },
}