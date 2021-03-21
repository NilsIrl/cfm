use regex::Regex;
use std::os::unix::process::CommandExt;
use std::{path::Path, process::Command};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(setting = AppSettings::TrailingVarArg)]
struct App {
    #[structopt(short, long, value_delimiter = ":", env = "CDPATH")]
    path: Vec<String>,

    #[structopt(allow_hyphen_values = true, use_delimiter = false, required = true)]
    cmd: Vec<String>,
}

fn main() {
    let rule_regex = Regex::new(r"\A(.+):(\d*)([-+]?)([t])\z").unwrap();
    let app = App::from_args();
    let mut commands = Command::new(&app.cmd[0]);
    let paths = app.path.iter().map(Path::new).collect::<Vec<_>>();
    'outer: for option in app.cmd.iter().skip(1) {
        if Path::new(option).exists() {
            commands.arg(option);
            continue 'outer;
        }

        for path in &paths {
            let fullpath = path.join(option);
            if fullpath.exists() {
                commands.arg(fullpath);
                continue 'outer;
            }
        }

        if let Some(caps) = rule_regex.captures(option) {
            let basepath = caps.get(1).unwrap().as_str();
            let count = {
                let count_capture = caps.get(2).unwrap();
                if count_capture.start() == count_capture.end() {
                    1
                } else {
                    count_capture.as_str().parse().unwrap()
                }
            };
            let reverse = {
                let reverse_capture = caps.get(3).unwrap();
                // "+" is true
                // true reverses
                // Defaults to false
                reverse_capture.start() != reverse_capture.end() && reverse_capture.as_str() == "+"
            };
            for path in &paths {
                let fullpath = path.join(basepath);
                if fullpath.exists() {
                    match caps.get(4).unwrap().as_str() {
                        "t" => {
                            if let Ok(dirlist) = fullpath.read_dir() {
                                let mut dirlist: Vec<_> =
                                    dirlist.map(|dir_entry| dir_entry.unwrap()).collect();
                                dirlist.sort_by_cached_key(|k| {
                                    k.metadata().unwrap().modified().unwrap()
                                });
                                if reverse {
                                    for dir in dirlist.iter().take(count) {
                                        commands.arg(dir.path());
                                    }
                                } else {
                                    for dir in dirlist.iter().rev().take(count) {
                                        commands.arg(dir.path());
                                    }
                                }
                                continue 'outer;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        commands.arg(option);
    }
    // TODO: print the command ran
    //dbg!(&commands);
    println!("{}", commands.exec());
}
