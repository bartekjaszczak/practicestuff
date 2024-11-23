use crate::args::prelude::*;
use std::cmp;

const INDENT_WIDTH: usize = 2; // indent of 2 spaces before the option/command
const LONG_NAME_PREFIX: usize = 2; // the "--" before the long name
const SHORT_NAME_WIDTH: usize = 4; // "-x, "
const GAP_WIDTH: usize = 3; // gap of 3 spaces between option/command name and description

pub struct Command<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> Command<'a> {
    pub const fn new(name: &'a str, description: &'a str) -> Self {
        Command { name, description }
    }
}

pub fn build(usage: &str, options: &[ArgDefinition], commands: &[Command]) -> String {
    let mut help = String::new();

    help.push_str(usage);

    let first_column_width = calculate_first_column_width(options, commands);

    if !options.is_empty() {
        help.push_str("\n\nOptions:\n");
        help.push_str(&build_options(options, first_column_width));
    }

    if !commands.is_empty() {
        help.push_str("\n\nCommands:\n");
        help.push_str(&build_commands(commands, first_column_width));
    }

    help
}

fn build_options(options: &[ArgDefinition], column_width: usize) -> String {
    let mut result = String::new();
    let long_name_width = column_width - INDENT_WIDTH - SHORT_NAME_WIDTH - GAP_WIDTH;

    for option in options {
        let short_name = match option.short_name() {
            Some(name) => format!("-{name}, "),
            None => " ".repeat(SHORT_NAME_WIDTH),
        };

        let mut long_name = match option.long_name() {
            Some(name) => format!("--{name}"),
            None => String::new(),
        };

        let gap_to_fill = long_name_width - long_name.len();
        long_name.push_str(&" ".repeat(gap_to_fill));

        result.push_str(&format!(
            "{}{}{}{}{}\n",
            " ".repeat(INDENT_WIDTH),
            short_name,
            long_name,
            " ".repeat(GAP_WIDTH),
            option.description().first().unwrap_or(&String::new()),
        ));

        let rest_of_description = option
            .description()
            .iter()
            .skip(1)
            .map(|description_line| format!("{}{}", " ".repeat(column_width), description_line))
            .collect::<Vec<String>>()
            .join("\n");

        result.push_str(&rest_of_description);
    }

    result
}

fn build_commands(commands: &[Command], column_width: usize) -> String {
    let mut result = String::new();

    for command in commands {
        let gap_to_fill = column_width - command.name.len() - INDENT_WIDTH;
        result.push_str(&format!(
            "{}{}{}{}\n",
            " ".repeat(INDENT_WIDTH),
            command.name,
            " ".repeat(gap_to_fill),
            command.description
        ));
    }

    result
}

fn calculate_first_column_width(options: &[ArgDefinition], commands: &[Command]) -> usize {
    let max_option_width = options
        .iter()
        .map(|option| option.long_name().unwrap_or("").len() + LONG_NAME_PREFIX)
        .max()
        .unwrap_or(0)
        + INDENT_WIDTH
        + SHORT_NAME_WIDTH
        + GAP_WIDTH;

    let max_command_width = commands
        .iter()
        .map(|command| command.name.len())
        .max()
        .unwrap_or(0)
        + INDENT_WIDTH
        + GAP_WIDTH;

    cmp::max(max_option_width, max_command_width)
}
