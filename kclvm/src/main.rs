//! The `kclvm` command-line interface.

#[macro_use]
extern crate clap;
use std::{path::Path};
use clap::ArgMatches;
use kclvm_config::settings::{load_file, merge_settings, SettingsFile};
use kclvm_parser::{load_program, parse_file};
use kclvm_runner::{command::Command, libgen::DyLibGenerator};


fn main() {
    let matches = clap_app!(kclx =>
        (@subcommand run =>
            (@arg INPUT: ... "Sets the input file to use")
            (@arg OUTPUT: -o --output +takes_value "Sets the LLVM IR/BC output file path")
            (@arg SETTING: ... -Y --setting "Sets the input file to use")
            (@arg EMIT_TYPE: --emit +takes_value "Sets the emit type, expect (ast)")
            (@arg BC_PATH: --bc +takes_value "Sets the linked LLVM bitcode file path")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@arg disable_none: -n --disable-none "Disable dumping None values")
            (@arg debug: -d --debug "Run in debug mode (for developers only)")
            (@arg sort_key: -k --sort "Sort result keys")
            (@arg ARGUMENT: ... -D --argument "Specify the top-level argument")
        )
    )
    .get_matches();
    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(files) = matches.values_of("INPUT") {
            let files: Vec<&str> = files.into_iter().collect::<Vec<&str>>();
            if let Some(emit_ty) = matches.value_of("EMIT_TYPE") {
                if emit_ty == "ast" {
                    let module = parse_file(files[0], None);
                    println!("{}", serde_json::to_string(&module).unwrap())
                }
            } else {
                // load ast
                let program = load_program(&files, None);
                let dylib_paths = DyLibGenerator::gen_and_run_dylib_from_ast(program);
                // let dylib_paths = dylib_gen.gen
                let mut cmd = Command::new(0);
                // link all dylibs
                let dylib_path = cmd.link_dylibs(&dylib_paths, "");
                // Config build
                let settings = build_settings(&matches);
                cmd.run_dylib_with_settings(&dylib_path, settings).unwrap();
                for dylib_path in dylib_paths {
                    if dylib_path.contains(kclvm_ast::MAIN_PKG) && Path::new(&dylib_path).exists() {
                        std::fs::remove_file(&dylib_path).unwrap();
                    }
                }
            }
        } else {
            println!("{}", matches.usage());
        }
    } else {
        println!("{}", matches.usage());
    }
}

/// Build settings from arg matches.
fn build_settings(matches: &ArgMatches) -> SettingsFile {
    let debug_mode = matches.occurrences_of("debug") > 0;
    let disable_none = matches.occurrences_of("disable_none") > 0;

    let mut settings = if let Some(files) = matches.values_of("SETTING") {
        let files: Vec<&str> = files.into_iter().collect::<Vec<&str>>();
        merge_settings(
            &files
                .iter()
                .map(|f| load_file(f))
                .collect::<Vec<SettingsFile>>(),
        )
    } else {
        SettingsFile::new()
    };
    if let Some(config) = &mut settings.kcl_cli_configs {
        config.debug = Some(debug_mode);
        config.disable_none = Some(disable_none);
    }
    settings
}
