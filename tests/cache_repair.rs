use anyhow::Result;

mod common;

use common::cache::*;
use common::common_args::*;
use common::input_arg::*;
use common::output_option::*;
use common::program::*;
use common::target::*;
use common::test_dir::*;

//------------------------------------------

const USAGE: &str = 
"cache_repair 0.9.0
Repair binary cache metadata, and write it to a different device or file

USAGE:
    cache_repair [FLAGS] --input <FILE> --output <FILE>

FLAGS:
    -q, --quiet      Suppress output messages, return only exit code.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     Specify the input device
    -o, --output <FILE>    Specify the output device";

//-----------------------------------------

struct CacheRepair;

impl<'a> Program<'a> for CacheRepair {
    fn name() -> &'a str {
        "cache_repair"
    }

    fn cmd<I>(args: I) -> duct::Expression
    where
        I: IntoIterator,
        I::Item: Into<std::ffi::OsString>,
    {
        cache_repair_cmd(args)
    }

    fn usage() -> &'a str {
        USAGE
    }

    fn arg_type() -> ArgType {
        ArgType::IoOptions
    }

    fn bad_option_hint(option: &str) -> String {
        msg::bad_option_hint(option)
    }
}

impl<'a> InputProgram<'a> for CacheRepair {
    fn mk_valid_input(td: &mut TestDir) -> Result<std::path::PathBuf> {
        mk_valid_md(td)
    }

    fn file_not_found() -> &'a str {
        msg::FILE_NOT_FOUND
    }

    fn missing_input_arg() -> &'a str {
        msg::MISSING_INPUT_ARG
    }

    fn corrupted_input() -> &'a str {
        "bad checksum in superblock"
    }
}

impl<'a> OutputProgram<'a> for CacheRepair {
    fn missing_output_arg() -> &'a str {
        msg::MISSING_OUTPUT_ARG
    }
}

impl<'a> MetadataWriter<'a> for CacheRepair {
    fn file_not_found() -> &'a str {
        msg::FILE_NOT_FOUND
    }
}

//-----------------------------------------

test_accepts_help!(CacheRepair);
test_accepts_version!(CacheRepair);
test_rejects_bad_option!(CacheRepair);

test_input_file_not_found!(CacheRepair);
test_input_cannot_be_a_directory!(CacheRepair);
test_corrupted_input_data!(CacheRepair);

test_missing_output_option!(CacheRepair);

//-----------------------------------------
