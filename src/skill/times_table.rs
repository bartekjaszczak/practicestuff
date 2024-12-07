use std::iter;
use std::ops::Mul;

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
        let (_, overflow) = u64::from(lower_boundary).overflowing_mul(u64::from(upper_boundary));
        if overflow {
            let max_factor = Self::calculate_max_factor(lower_boundary, upper_boundary);
            return Err(Self::build_err_message(Some(
                        format!("{lower_boundary}*{upper_boundary} exceeds maximum allowed value. Maximum factor that works with {lower_boundary} is {max_factor}")
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
                .description(vec!["Display help for times table command.".to_string()])
                .kind(ArgKind::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            Arg::builder()
                .id(ARG_ID_LOWER_BOUNDARY)
                .short_name('l')
                .long_name("lower-boundary")
                .description(vec![
                    "Set the minimum factor to use in questions (default: 1).".to_string(),
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
                    "Set the maximum factor to use in questions (default: 10).".to_string(),
                ])
                .kind(ArgKind::Value(ValueKind::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(10))
                .build(),
        ]
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... timestable [timestable_option]...")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME}' timestable --help for more information.")
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

        format!("Practice multiplication table with a customisable factor range. By default, the factor range mimics the normal times table ({default_lower_boundary}-{default_upper_boundary}).")
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
        let result = u64::from(first).mul(u64::from(second));

        Question::builder()
            .question(&format!("{first}*{second}"))
            .answer(&result.to_string())
            .build()
    }

    fn calculate_max_factor(lower_factor: u32, chosen_factor: u32) -> u32 {
        let mut low = 0;
        let mut high = chosen_factor;
        let mut max_factor = 0;
        while low <= high {
            let mid = low + (high - low) / 2;
            let (_, overflow) = u64::from(lower_factor).overflowing_mul(u64::from(mid));
            if overflow {
                high = mid - 1;
            } else {
                max_factor = mid;
                low = mid + 1;
            }
        }
        max_factor
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
