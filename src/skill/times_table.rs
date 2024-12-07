use std::iter;

use rand::Rng;

use super::Base;
use crate::application::APP_NAME;
use crate::args::prelude::*;
use crate::question::Question;

pub const CMD: &str = "times_table";

const ARG_ID_HELP: &str = "help";
const ARG_ID_LOWER_BOUNDARY: &str = "lower_boundary";
const ARG_ID_UPPER_BOUNDARY: &str = "upper_boundary";

#[derive(Debug)]
pub struct TimesTable {
    arg_definitions: Vec<Arg>,
    show_help: bool,

    lower_boundary: u32,
    upper_boundary: u32,
}

impl TimesTable {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::build_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)
            .map_err(|err| Self::build_err_message(Some(err)))?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let lower_boundary = u32::set_value_from_arg_or_default(
            ARG_ID_LOWER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );
        let upper_boundary = u32::set_value_from_arg_or_default(
            ARG_ID_UPPER_BOUNDARY,
            &parsed_args,
            &arg_definitions,
        );

        if lower_boundary > upper_boundary {
            return Err(Self::build_err_message(Some(
                "lower boundary must be less than or equal to upper boundary".to_string(),
            )));
        }

        Ok(Self {
            arg_definitions,
            show_help,
            lower_boundary,
            upper_boundary,
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
                .id(ARG_ID_LOWER_BOUNDARY)
                .short_name('l')
                .long_name("lower-boundary")
                .description(vec![
                    "Set the minimum factor (default: 1).".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(1))
                .build(),
            Arg::builder()
                .id(ARG_ID_UPPER_BOUNDARY)
                .short_name('u')
                .long_name("upper-boundary")
                .description(vec![
                    "Set the maximum factor (default: 10).".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(10))
                .build(),
        ]
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... times_table [times_table_option]...")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME}' times_table --help for more information.")
    }

    fn additional_info(&self) -> String {
        let default_lower_boundary = self
            .arg_definitions
            .iter()
            .find(|def| def.id() == ARG_ID_LOWER_BOUNDARY)
            .expect("lower boundary argument definition not found")
            .default_value()
            .to_string();
        let default_upper_boundary = self
            .arg_definitions
            .iter()
            .find(|def| def.id() == ARG_ID_UPPER_BOUNDARY)
            .expect("upper boundary argument definition not found")
            .default_value()
            .to_string();

        format!("Practise multiplication with a customisable factors' range.\nBy default, the range of factors mimics the normal times table ({default_lower_boundary}-{default_upper_boundary}).")
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
        let first = rng.gen_range(self.lower_boundary..=self.upper_boundary);
        let second = rng.gen_range(self.lower_boundary..=self.upper_boundary);
        let result = u64::from(first) * u64::from(second); // u32::MAX ^ 2 < u64::MAX

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
        help::build(&Self::usage(), Some(&self.additional_info()), &options, &[])
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
        assert_eq!(times_table.lower_boundary, 1);
        assert_eq!(times_table.upper_boundary, 10);
    }

    #[test]
    #[should_panic(expected = "invalid option argument")]
    fn build_times_table_incorrect_args() {
        let args = ["-l".to_string(), "hehe".to_string(), "-what".to_string()];
        TimesTable::build(&args).unwrap();
    }

    #[test]
    fn build_times_table_with_args() {
        let args = [
            "--lower-boundary=4".to_string(),
            "-u".to_string(),
            "7".to_string(),
        ];
        let times_table = TimesTable::build(&args).expect("Should build correctly with args");
        assert!(!times_table.show_help);
        assert_eq!(times_table.lower_boundary, 4);
        assert_eq!(times_table.upper_boundary, 7);
    }

    #[test]
    #[should_panic(expected = "lower boundary must be less than or equal to upper boundary")]
    fn build_times_table_mismatched_boundaries() {
        let args = [
            "-l".to_string(),
            "5".to_string(),
            "-u".to_string(),
            "4".to_string(),
        ];
        TimesTable::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid option argument")]
    fn build_times_table_overflow() {
        let args = [
            "-l".to_string(),
            "4294967296".to_string(),
            "-u".to_string(),
            "4294967296".to_string(),
        ]; // factors are u32 and the result is u64
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
        let args = ["-u".to_string(), "1".to_string()];
        let times_table = TimesTable::build(&args).expect("Should build correctly");
        let question = times_table.generate_question();
        assert_eq!(question.prompt(), "1*1");
        assert_eq!(question.correct_answer(), "1");
        assert!(question.is_answer_correct("1"));
    }

    #[test]
    fn multiple_question_generation() {
        let args = [
            "--lower-boundary=3".to_string(),
            "--upper-boundary=3".to_string(),
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
            "-l".to_string(),
            "4".to_string(),
            "-h".to_string(),
            "--upper-boundary=10".to_string(),
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
        assert!(help_text.contains("-l, --lower-boundary"));
        assert!(help_text.contains("-u, --upper-boundary"));
    }
}
