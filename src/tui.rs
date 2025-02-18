//! Utilities for terminal output during scanning.

/// Terminal User Interface Module for RustScan
/// Defines macros to use
#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        println!("{} {}", ansi_term::Colour::Red.bold().paint("[!]"), $name);
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!("{} {}", ansi_term::Colour::Red.bold().paint("[!]"), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        println!("{} {}", ansi_term::Colour::Blue.bold().paint("[~]"), $name);
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!("{} {}", ansi_term::Colour::Blue.bold().paint("[~]"), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr) => {
        println!(
            "{} {}",
            RGansi_term::Colour::RGB(0, 255, 9).bold().paint("[>]"),
            $name
        );
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!(
                    "{} {}",
                    ansi_term::Colour::RGB(0, 255, 9).bold().paint("[>]"),
                    $name
                );
            }
        }
    };
}
