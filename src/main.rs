mod argparse;
mod log;
mod substring;

use std::error::Error;
use std::ops::Index;
use std::process::Command;
use std::{process, time};

use argparse::ArgParseError;

use crate::log::format_duration;
use crate::{
    argparse::{Argument, ArgumentParser},
    log::log_with_header,
};

fn create_command_from_str(cmd_str: &str) -> Command {
    let tokens: Vec<&str> = cmd_str.split(" ").collect();
    let mut pre_cmd = Command::new(tokens[0]);
    pre_cmd.args(&tokens[1..tokens.len()]);
    pre_cmd
}

fn main() -> Result<(), Box<dyn Error>> {
    // Registering Ctrl+C handler
    // See: https://rust-cli.github.io/book/in-depth/signals.html
    ctrlc::set_handler(move || {
        println!("[INFO] Received termination signal, exiting...");
    })
    .expect("Error setting Ctrl+C handler");

    let mut parser = ArgumentParser::new();

    parser.add_argument(Argument {
        name: "pre".into(),
        description: "Command executed before the main command".into(),
        required: false,
        multiple: true,
    });

    parser.add_argument(Argument {
        name: "cmd".into(),
        description: "The command to execute".into(),
        required: true,
        multiple: false,
    });

    parser.add_argument(Argument {
        name: "post".into(),
        description: "Command to execute after the main command exited or failed".into(),
        required: false,
        multiple: true,
    });

    let args = match parser.parse() {
        Ok(hashmap) => hashmap,
        Err(ArgParseError { arg, reason }) => {
            println!(
                "[ERROR] Error when parsing arguments: argument \"{}\" {}",
                arg, reason
            );
            process::exit(1);
        }
    };

    // Get arguments
    let pre = args.get("pre".into());
    let cmd = args.get("cmd".into()).unwrap();
    let post = args.get("post".into());

    // Execute pre commands if any
    match pre {
        Some(pre_cmds) => {
            for (i, pre_cmd) in pre_cmds.iter().enumerate() {
                let mut pre_cmd = create_command_from_str(pre_cmd);

                log_with_header(format!("PRE COMMAND {} BEGIN", i).as_str());
                let before_pre = time::Instant::now();

                let status = match pre_cmd.status() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("[ERROR] Error when launching pre-command: {:?}", err);
                        process::exit(1);
                    }
                };

                if !status.success() {
                    println!("[ERROR] Pre command failed with {}", status);
                    process::exit(status.code().unwrap_or(1));
                }

                log_with_header(
                    format!(
                        "PRE COMMAND {} END in {} ({})",
                        i,
                        format_duration(&before_pre.elapsed()),
                        status
                    )
                    .as_str(),
                );
                println!("");
            }
        }
        None => (),
    };

    // Execute the main command
    let mut cmd_instance = create_command_from_str(cmd.index(0));
    log_with_header("MAIN COMMAND BEGIN");
    let before_cmd = time::Instant::now();
    let cmd_status = match cmd_instance.status() {
        Ok(s) => s,
        Err(err) => {
            println!("[ERROR] Error when launching command: {:?}", err);
            process::exit(1);
        }
    };
    log_with_header(
        format!(
            "MAIN COMMAND END in {} ({})",
            format_duration(&before_cmd.elapsed()),
            cmd_status,
        )
        .as_str(),
    );
    println!("");

    // Execute post commands even if the main command failed
    match post {
        Some(post_cmds) => {
            for (i, post_cmd) in post_cmds.iter().enumerate() {
                let mut post_cmd = create_command_from_str(post_cmd);

                log_with_header(format!("POST COMMAND {} BEGIN", i).as_str());
                let before_post = time::Instant::now();

                let status = match post_cmd.status() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("[ERROR] Error when launching post-command: {:?}", err);
                        process::exit(1);
                    }
                };

                log_with_header(
                    format!(
                        "POST COMMAND {} END in {} ({})",
                        i,
                        format_duration(&before_post.elapsed()),
                        status
                    )
                    .as_str(),
                );
            }
        }
        None => (),
    }

    if !cmd_status.success() {
        process::exit(cmd_status.code().unwrap_or(1));
    }

    Ok(())
}
