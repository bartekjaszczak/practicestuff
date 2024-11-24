use super::definition::ArgDefinition;
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
        help.push_str("\nCommands:\n");
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

        let mut rest_of_description = option
            .description()
            .iter()
            .skip(1)
            .map(|description_line| format!("{}{}", " ".repeat(column_width), description_line))
            .collect::<Vec<String>>()
            .join("\n");
        if !rest_of_description.is_empty() {
            rest_of_description.push('\n');
        }

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
        .map(|option| match option.long_name() {
            Some(name) => name.len() + LONG_NAME_PREFIX,
            None => 0,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::prelude::*;

    #[test]
    fn option_with_short_name_only() {
        let options = [ArgDefinition::builder()
            .id("arg1")
            .short_name('s')
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        assert_eq!(
            calculate_first_column_width(&options, &[]),
            INDENT_WIDTH + SHORT_NAME_WIDTH + GAP_WIDTH
        );
    }

    #[test]
    fn option_with_long_name_only() {
        let name = "long-name";
        let options = [ArgDefinition::builder()
            .id("arg1")
            .long_name(name)
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        assert_eq!(
            calculate_first_column_width(&options, &[]),
            INDENT_WIDTH + SHORT_NAME_WIDTH + LONG_NAME_PREFIX + name.len() + GAP_WIDTH
        ); // Space for short name is always taken into consideration
    }

    #[test]
    fn option_with_short_and_long_names() {
        let name = "long-name";
        let options = [ArgDefinition::builder()
            .id("arg1")
            .short_name('s')
            .long_name(name)
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        assert_eq!(
            calculate_first_column_width(&options, &[]),
            INDENT_WIDTH + SHORT_NAME_WIDTH + LONG_NAME_PREFIX + name.len() + GAP_WIDTH
        );
    }

    #[test]
    fn multiple_options_with_short_names_only() {
        let options = [
            ArgDefinition::builder()
                .id("arg1")
                .short_name('s')
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("arg2")
                .short_name('x')
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        assert_eq!(
            calculate_first_column_width(&options, &[]),
            INDENT_WIDTH + SHORT_NAME_WIDTH + GAP_WIDTH
        );
    }

    #[test]
    fn multiple_options() {
        let name1 = "long-name";
        let name2 = "even-longer-name";
        let name3 = "the-loooooooongest-name";
        let options = [
            ArgDefinition::builder()
                .id("arg1")
                .short_name('s')
                .long_name(name1)
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("arg2")
                .short_name('s')
                .long_name(name2)
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id("arg3")
                .short_name('s')
                .long_name(name3)
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .build(),
        ];

        assert_eq!(
            calculate_first_column_width(&options, &[]),
            INDENT_WIDTH + SHORT_NAME_WIDTH + LONG_NAME_PREFIX + name3.len() + GAP_WIDTH
        );
    }

    #[test]
    fn multiple_commands() {
        let name1 = "shortest";
        let name2 = "looooongest-command";
        let name3 = "in-between";
        let commands = [
            Command::new(name1, "desc1"),
            Command::new(name2, "desc2"),
            Command::new(name3, "desc3"),
        ];

        assert_eq!(
            calculate_first_column_width(&[], &commands),
            INDENT_WIDTH + name2.len() + GAP_WIDTH
        );
    }

    #[test]
    fn mix_command_longer_than_option() {
        let command_name = "the-looooongest-command-you-can-imagine";
        let commands = [Command::new(command_name, "desc")];
        let long_name = "opt-long-name";
        let options = [ArgDefinition::builder()
            .id("arg")
            .long_name(long_name)
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        assert_eq!(
            calculate_first_column_width(&options, &commands),
            INDENT_WIDTH + command_name.len() + GAP_WIDTH
        );
    }

    #[test]
    fn mix_command_shorter_than_option() {
        let command_name = "cmd";
        let commands = [Command::new(command_name, "desc")];
        let long_name = "very-long-command-name";
        let options = [ArgDefinition::builder()
            .id("arg")
            .long_name(long_name)
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        assert_eq!(
            calculate_first_column_width(&options, &commands),
            INDENT_WIDTH + SHORT_NAME_WIDTH + LONG_NAME_PREFIX + long_name.len() + GAP_WIDTH
        );
    }

    #[test]
    fn only_options() {
        let options = [ArgDefinition::builder()
            .id("arg")
            .long_name("some-name")
            .kind(ArgKindDefinition::Flag)
            .default_value(ArgValue::Bool(false))
            .build()];

        let result = super::build("Usage: something something", &options, &[]);
        assert!(result.contains("Usage: something something"));
        assert!(result.contains("Options:"));
        assert!(result.contains("--some-name"));
        assert!(!result.contains("Commands:"));
    }

    #[test]
    fn only_commands() {
        let commands = [
            Command::new("cmd", "desc")
        ];

        let result = super::build("Usage: something something", &[], &commands);
        assert!(result.contains("Usage: something something"));
        assert!(!result.contains("Options:"));
        assert!(result.contains("Commands:"));
        assert!(result.contains("cmd"));
        assert!(result.contains("desc"));
    }

    #[test]
    fn full_help() {
        let usage = "Usage: some text";

        let options = [
            ArgDefinition::builder()
                .id("arg1")
                .short_name('a')
                .long_name("first-name")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .description(vec!["one-line description".to_string()])
                .build(),
            ArgDefinition::builder()
                .id("arg2")
                .short_name('b')
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                // no description
                .build(),
            ArgDefinition::builder()
                .id("arg3")
                .long_name("third-name")
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .description(vec![
                    "first description line".to_string(),
                    "second line".to_string(),
                ])
                .build(),
            ArgDefinition::builder()
                .id("arg4")
                .short_name('d')
                .kind(ArgKindDefinition::Flag)
                .default_value(ArgValue::Bool(false))
                .description(vec!["another short description".to_string()])
                .build(),
        ];

        let commands = [
            Command::new("first-command", ""),
            Command::new("second", "This one has a description."),
        ];

        let mut expected = String::new();
        expected.push_str("Usage: some text\n");
        expected.push('\n');
        expected.push_str("Options:\n");
        expected.push_str("  -a, --first-name   one-line description\n");
        expected.push_str("  -b,                \n");
        expected.push_str("      --third-name   first description line\n");
        expected.push_str("                     second line\n");
        expected.push_str("  -d,                another short description\n");
        expected.push('\n');
        expected.push_str("Commands:\n");
        expected.push_str("  first-command      \n");
        expected.push_str("  second             This one has a description.\n");

        assert_eq!(super::build(usage, &options, &commands), expected);
    }
}