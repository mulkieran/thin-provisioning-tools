extern crate clap;

use clap::Arg;
use std::path::Path;

use crate::commands::utils::*;
use crate::commands::Command;
use crate::report::*;

pub struct ThinMetadataPackCommand;

impl ThinMetadataPackCommand {
    fn cli<'a>(&self) -> clap::Command<'a> {
        clap::Command::new(self.name())
            .color(clap::ColorChoice::Never)
            .version(crate::version::tools_version())
            .about("Produces a compressed file of thin metadata.  Only packs metadata blocks that are actually used.")
            .arg(Arg::new("INPUT")
                .help("Specify thinp metadata binary device/file")
                .required(true)
                .short('i')
                .value_name("DEV")
                .takes_value(true))
            .arg(Arg::new("OUTPUT")
                .help("Specify packed output file")
                .required(true)
                .short('o')
                .value_name("FILE")
                .takes_value(true))
    }
}

impl<'a> Command<'a> for ThinMetadataPackCommand {
    fn name(&self) -> &'a str {
        "thin_metadata_pack"
    }

    fn run(&self, args: &mut dyn Iterator<Item = std::ffi::OsString>) -> std::io::Result<()> {
        let matches = self.cli().get_matches_from(args);

        let input_file = Path::new(matches.value_of("INPUT").unwrap());
        let output_file = Path::new(matches.value_of("OUTPUT").unwrap());

        let report = mk_simple_report();
        check_input_file(input_file, &report);

        crate::pack::toplevel::pack(input_file, output_file).map_err(|reason| {
            report.fatal(&format!("Application error: {}\n", reason));
            std::io::Error::from_raw_os_error(libc::EPERM)
        })
    }
}