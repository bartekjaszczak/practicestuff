use std::iter;

use rand::Rng;

use super::Base;
use crate::application::APP_NAME;
use crate::args::prelude::*;
use crate::question::Question;

pub const CMD: &str = "times_table";

const ARG_ID_HELP: &str = "help";
const ARG_ID_LOWER_BOUNDARY_1: &str = "lower_boundary_1";
const ARG_ID_UPPER_BOUNDARY_1: &str = "upper_boundary_1";
const ARG_ID_LOWER_BOUNDARY_2: &str = "lower_boundary_2";
const ARG_ID_UPPER_BOUNDARY_2: &str = "upper_boundary_2";

const DEFAULT_LOWER_BOUNDARY: u32 = 1;
const DEFAULT_UPPER_BOUNDARY: u32 = 10;

#[derive(Debug)]
pub struct TimesTable {
    arg_definitions: Vec<Arg>,
    show_help: bool,

    lower_boundary_1: u32,
    upper_boundary_1: u32,
    lower_boundary_2: u32,
    upper_boundary_2: u32,
}

impl TimesTable {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::build_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)
            .map_err(|err| Self::build_err_message(Some(err)))?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let lower_boundary_1 = u32::set_value_from_arg_or_default(
            ARG_ID_LOWER_BOUNDARY_1,
            &parsed_args,
            &arg_definitions,
        );
        let upper_boundary_1 = u32::set_value_from_arg_or_default(
            ARG_ID_UPPER_BOUNDARY_1,
            &parsed_args,
            &arg_definitions,
        );

        let lower_boundary_2 = u32::set_value_from_arg_or_default(
            ARG_ID_LOWER_BOUNDARY_2,
            &parsed_args,
            &arg_definitions,
        );
        let upper_boundary_2 = u32::set_value_from_arg_or_default(
            ARG_ID_UPPER_BOUNDARY_2,
            &parsed_args,
            &arg_definitions,
        );

        if lower_boundary_1 > upper_boundary_1 || lower_boundary_2 > upper_boundary_2 {
            return Err(Self::build_err_message(Some(
                "lower boundary must be less than or equal to upper boundary".to_string(),
            )));
        }

        Ok(Self {
            arg_definitions,
            show_help,
            lower_boundary_1,
            upper_boundary_1,
            lower_boundary_2,
            upper_boundary_2,
        })
    }

    fn build_arg_definitions() -> Vec<Arg> {
        vec![
            Arg::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display help for times_table command.".to_string()])
                .kind(ArgKind::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_LOWER_BOUNDARY_1)
                .long_name("lower-boundary-1")
                .description(vec![
                    "Set the minimum value".to_string(),
                    format!(
                        "for the first factor (default: {}).",
                        DEFAULT_LOWER_BOUNDARY
                    ),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(DEFAULT_LOWER_BOUNDARY))
                .build(),
            Arg::builder()
                .id(ARG_ID_UPPER_BOUNDARY_1)
                .long_name("upper-boundary-1")
                .description(vec![
                    "Set the maximum value".to_string(),
                    format!(
                        "for the first factor (default: {}).",
                        DEFAULT_UPPER_BOUNDARY
                    ),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(DEFAULT_UPPER_BOUNDARY))
                .build(),
            Arg::builder()
                .id(ARG_ID_LOWER_BOUNDARY_2)
                .long_name("lower-boundary-2")
                .description(vec![
                    "Set the minimum value".to_string(),
                    format!(
                        "for the second factor (default: {}).",
                        DEFAULT_LOWER_BOUNDARY
                    ),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(DEFAULT_LOWER_BOUNDARY))
                .build(),
            Arg::builder()
                .id(ARG_ID_UPPER_BOUNDARY_2)
                .long_name("upper-boundary-2")
                .description(vec![
                    "Set the maximum value".to_string(),
                    format!(
                        "for the second factor (default: {}).",
                        DEFAULT_UPPER_BOUNDARY
                    ),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(DEFAULT_UPPER_BOUNDARY))
                .build(),
        ]
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... times_table [times_table_option]...")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME}' times_table --help for more information.")
    }

    fn additional_info() -> String {
        format!("Practise multiplication with a customisable factors' range.\nBy default, the range of factors mimics the normal times table ({DEFAULT_LOWER_BOUNDARY}-{DEFAULT_UPPER_BOUNDARY}).\nThe range is set separately for both factors.")
    }

    fn build_err_message(msg: Option<String>) -> String {
        if let Some(msg) = msg {
            format!(
                "{}: {}: {}\n{}\n{}",
                APP_NAME,
                CMD,
                msg,
                Self::usage(),
                Self::help_prompt()
            )
        } else {
            format!("{}\n{}", Self::usage(), Self::help_prompt())
        }
    }

    fn generate_question(&self) -> Question {
        let mut rng = rand::thread_rng();
        let mut first = rng.gen_range(self.lower_boundary_1..=self.upper_boundary_1);
        let mut second = rng.gen_range(self.lower_boundary_2..=self.upper_boundary_2);
        let result = u64::from(first) * u64::from(second); // u32::MAX ^ 2 < u64::MAX

        if rng.gen_bool(0.5) {
            std::mem::swap(&mut first, &mut second);
        }

        Question::builder()
            .question(&format!("{first}*{second}"))
            .answer(&result.to_string())
            .build()
    }
}

impl Base for TimesTable {
    fn generate_questions(&self, count: u32) -> Vec<crate::question::Question> {
        iter::repeat_with(|| self.generate_question())
            .take(count as usize)
            .collect()
    }

    fn wants_to_print_help(&self) -> bool {
        self.show_help
    }

    fn get_help_text(&self) -> String {
        let definitions = &self.arg_definitions;
        let options = help::Options::new("Times table options", definitions);
        help::build(
            &Self::usage(),
            Some(&Self::additional_info()),
            &options,
            &[],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_times_table_defaults() {
        let args = [];
        let times_table = TimesTable::build(&args).expect("Should build correctly with no args");
        assert!(!times_table.show_help);
        assert_eq!(times_table.lower_boundary_1, 1);
        assert_eq!(times_table.upper_boundary_1, 10);
        assert_eq!(times_table.lower_boundary_2, 1);
        assert_eq!(times_table.upper_boundary_2, 10);
    }

    #[test]
    #[should_panic(expected = "invalid option argument")]
    fn build_times_table_incorrect_args() {
        let args = ["--upper-boundary-1=yay".to_string(), "hehe".to_string(), "-what".to_string()];
        TimesTable::build(&args).unwrap();
    }

    #[test]
    fn build_times_table_with_args() {
        let args = [
            "--lower-boundary-1=4".to_string(),
            "--upper-boundary-1=7".to_string(),
            "--lower-boundary-2=5".to_string(),
            "--upper-boundary-2=8".to_string(),
        ];
        let times_table = TimesTable::build(&args).expect("Should build correctly with args");
        assert!(!times_table.show_help);
        assert_eq!(times_table.lower_boundary_1, 4);
        assert_eq!(times_table.upper_boundary_1, 7);
        assert_eq!(times_table.lower_boundary_2, 5);
        assert_eq!(times_table.upper_boundary_2, 8);
    }

    #[test]
    #[should_panic(expected = "lower boundary must be less than or equal to upper boundary")]
    fn build_times_table_mismatched_first_boundary() {
        let args = [
            "--lower-boundary-1=5".to_string(),
            "--upper-boundary-1=4".to_string(),
        ];
        TimesTable::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "lower boundary must be less than or equal to upper boundary")]
    fn build_times_table_mismatched_second_boundary() {
        let args = [
            "--lower-boundary-2=5".to_string(),
            "--upper-boundary-2=4".to_string(),
        ];
        TimesTable::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid option argument")]
    fn build_times_table_overflow() {
        let args = ["--upper-boundary-1=4294967296".to_string()]; // factors are u32 and the result is u64
                                                                  // parser won't accept u32::MAX + 1 and larger
                                                                  // no risk of overflowing
        TimesTable::build(&args).unwrap();
    }

    #[test]
    fn error_message() {
        let err = Some("something extraordinarily wrong happened".to_string());
        let message = TimesTable::build_err_message(err);
        assert!(message.contains(APP_NAME));
        assert!(message.contains(CMD));
        assert!(message.contains("something extraordinarily wrong happened"));
        assert!(message.contains("Usage"));
        assert!(message.contains("for more information"));

        let err = None;
        let message = TimesTable::build_err_message(err);
        assert!(message.contains(APP_NAME));
        assert!(message.contains(CMD));
        assert!(message.contains("Usage"));
        assert!(message.contains("for more information"));
    }

    #[test]
    fn question_generation() {
        let args = [
            "--upper-boundary-1=1".to_string(),
            "--upper-boundary-2=1".to_string(),
        ];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        let question = times_table.generate_question();
        assert_eq!(question.prompt(), "1*1");
        assert_eq!(question.correct_answer(), "1");
        assert!(question.is_answer_correct("1"));
    }

    #[test]
    fn multiple_question_generation() {
        let args = [
            "--lower-boundary-1=3".to_string(),
            "--upper-boundary-1=3".to_string(),
            "--lower-boundary-2=3".to_string(),
            "--upper-boundary-2=3".to_string(),
        ];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        let questions = times_table.generate_questions(10);
        assert_eq!(questions.len(), 10);
        assert!(questions
            .iter()
            .all(|question| question.prompt().contains("3*3")));
    }

    #[test]
    fn print_help_only() {
        let args = [];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        assert!(!times_table.wants_to_print_help());

        let args = [
            "--lower-boundary-1=4".to_string(),
            "-h".to_string(),
            "--upper-boundary-2=10".to_string(),
        ];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        assert!(times_table.wants_to_print_help());
    }

    #[test]
    fn help_text() {
        let args = ["-h".to_string()];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        let help_text = times_table.get_help_text();
        assert!(help_text.contains("Times table options"));
        assert!(help_text.contains("Usage"));

        // Ensure all flags are included
        assert!(help_text.contains("-h, --help"));
        assert!(help_text.contains("--lower-boundary-1"));
        assert!(help_text.contains("--upper-boundary-1"));
        assert!(help_text.contains("--lower-boundary-2"));
        assert!(help_text.contains("--upper-boundary-2"));
    }
}
