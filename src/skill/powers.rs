use std::iter;

use rand::Rng;

use super::SkillBase;
use crate::application::APP_NAME;
use crate::args::prelude::*;
use crate::question::Question;

pub const CMD: &str = "powers";

const ARG_ID_HELP: &str = "help";
const ARG_ID_BASE: &str = "base";
const ARG_ID_LOWER_BOUNDARY: &str = "lower_boundary";
const ARG_ID_UPPER_BOUNDARY: &str = "upper_boundary";

#[derive(Debug)]
pub struct Powers {
    show_help: bool,

    base: u32,
    lower_boundary: u32,
    upper_boundary: u32,
}

impl Powers {
    pub fn build(args: &[String]) -> Result<Self, String> {
        let arg_definitions = Self::get_arg_definitions();
        let parsed_args = parser::parse_and_validate_arg_list(args, &arg_definitions)
            .map_err(|err| Self::build_err_message(Some(err)))?;

        let show_help =
            bool::set_value_from_arg_or_default(ARG_ID_HELP, &parsed_args, &arg_definitions);
        let base = u32::set_value_from_arg_or_default(ARG_ID_BASE, &parsed_args, &arg_definitions);
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

        // check for power overflow
        let (_, overflow) = u64::from(base).overflowing_pow(upper_boundary);
        if overflow {
            let max_exp = Self::calculate_max_exponent(base, upper_boundary);
            return Err(Self::build_err_message(Some(
                        format!("{base}^{upper_boundary} exceeds maximum allowed value. Maximum exponent for base {base} is {max_exp}")
            )));
        }

        Ok(Self {
            show_help,
            base,
            lower_boundary,
            upper_boundary,
        })
    }

    fn usage() -> String {
        format!("Usage: {APP_NAME} [option]... powers [powers_option]...")
    }

    fn help_prompt() -> String {
        format!("Try '{APP_NAME} powers --help' for more information.")
    }

    fn additional_info() -> String {
        let default_base = Self::get_arg_definitions()
            .iter()
            .find(|def| def.id() == ARG_ID_BASE)
            .expect("base argument definition not found")
            .default_value()
            .to_string();
        let default_lower_boundary = Self::get_arg_definitions()
            .iter()
            .find(|def| def.id() == ARG_ID_LOWER_BOUNDARY)
            .expect("lower boundary argument definition not found")
            .default_value()
            .to_string();
        let default_upper_boundary = Self::get_arg_definitions()
            .iter()
            .find(|def| def.id() == ARG_ID_UPPER_BOUNDARY)
            .expect("upper boundary argument definition not found")
            .default_value()
            .to_string();

        format!("Practice powers with a customizable base and exponent range. By default, the base is {default_base}, with exponents ranging from {default_lower_boundary} to {default_upper_boundary}.")
    }

    fn get_arg_definitions() -> Vec<ArgDefinition> {
        vec![
            ArgDefinition::builder()
                .id(ARG_ID_HELP)
                .short_name('h')
                .long_name("help")
                .description(vec!["Display help for powers command.".to_string()])
                .kind(ArgKindDefinition::Flag)
                .stop_parsing(true)
                .default_value(ArgValue::Bool(false))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_BASE)
                .short_name('b')
                .long_name("base")
                .description(vec!["Set the base for powers (default: 2).".to_string()])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(2))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_LOWER_BOUNDARY)
                .short_name('l')
                .long_name("lower-boundary")
                .description(vec![
                    "Set the minimum exponent to use in questions (default: 1).".to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(1))
                .build(),
            ArgDefinition::builder()
                .id(ARG_ID_UPPER_BOUNDARY)
                .short_name('u')
                .long_name("upper-boundary")
                .description(vec![
                    "Set the maximum exponent to use in questions (default: 16).".to_string(),
                ])
                .kind(ArgKindDefinition::Value(ValueKindDefinition::UnsignedInt))
                .stop_parsing(false)
                .default_value(ArgValue::UnsignedInt(16))
                .build(),
        ]
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
        let exp = rng.gen_range(self.lower_boundary..=self.upper_boundary);
        let result = u64::from(self.base).pow(exp); // Won't overflow, checked during Powers construction
        Question::builder()
            .question(&format!("{base}^{exp}", base = self.base))
            .answer(&result.to_string())
            .build()
    }

    fn calculate_max_exponent(base: u32, chosen_exponent: u32) -> u32 {
        let mut low = 0;
        let mut high = chosen_exponent;
        let mut max_exp = 0;
        while low <= high {
            let mid = low + (high - low) / 2;
            let (_, overflow) = u64::from(base).overflowing_pow(mid);
            if overflow {
                high = mid - 1;
            } else {
                max_exp = mid;
                low = mid + 1;
            }
        }
        max_exp
    }
}

impl SkillBase for Powers {
    fn generate_questions(&self, count: u32) -> Vec<Question> {
        iter::repeat_with(|| self.generate_question())
            .take(count as usize)
            .collect()
    }

    fn wants_to_print_help(&self) -> bool {
        self.show_help
    }

    fn get_help_text(&self) -> String {
        let definitions = &Self::get_arg_definitions();
        let options = help::Options::new("Powers options", definitions);
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
    fn build_powers_defaults() {
        let args = [];
        let powers = Powers::build(&args).expect("Should build correctly with no args");
        assert!(!powers.show_help);
        assert_eq!(powers.base, 2);
        assert_eq!(powers.lower_boundary, 1);
        assert_eq!(powers.upper_boundary, 16);
    }

    #[test]
    #[should_panic(expected = "invalid option argument")]
    fn build_powers_incorrect_args() {
        let args = ["-b".to_string(), "hehe".to_string(), "-what".to_string()];
        Powers::build(&args).unwrap();
    }

    #[test]
    fn build_powers_with_args() {
        let args = [
            "-b".to_string(),
            "5".to_string(),
            "--lower-boundary=4".to_string(),
            "-u".to_string(),
            "7".to_string(),
        ];
        let powers = Powers::build(&args).expect("Should build correctly with args");
        assert!(!powers.show_help);
        assert_eq!(powers.base, 5);
        assert_eq!(powers.lower_boundary, 4);
        assert_eq!(powers.upper_boundary, 7);
    }

    #[test]
    #[should_panic(expected = "lower boundary must be less than or equal to upper boundary")]
    fn build_powers_mismatched_boundaries() {
        let args = [
            "-l".to_string(),
            "5".to_string(),
            "-u".to_string(),
            "4".to_string(),
        ];
        Powers::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "exceeds maximum allowed value")]
    fn build_powers_overflow() {
        let args = [
            "-b".to_string(),
            "2".to_string(),
            "-u".to_string(),
            "64".to_string(),
        ];
        Powers::build(&args).unwrap();
    }

    #[test]
    fn error_message() {
        let err = Some("something extraordinarily wrong happened".to_string());
        let message = Powers::build_err_message(err);
        assert!(message.contains(APP_NAME));
        assert!(message.contains(CMD));
        assert!(message.contains("something extraordinarily wrong happened"));
        assert!(message.contains("Usage"));
        assert!(message.contains("for more information"));

        let err = None;
        let message = Powers::build_err_message(err);
        assert!(message.contains(APP_NAME));
        assert!(message.contains(CMD));
        assert!(message.contains("Usage"));
        assert!(message.contains("for more information"));
    }

    #[test]
    fn max_exponent() {
        assert_eq!(Powers::calculate_max_exponent(2, 64), 63);
        assert_eq!(Powers::calculate_max_exponent(5, 40), 27);
        assert_eq!(Powers::calculate_max_exponent(17, 100), 15);
        assert_eq!(Powers::calculate_max_exponent(101, 21), 9);
    }

    #[test]
    fn question_generation() {
        let args = ["-u".to_string(), "1".to_string()];
        let powers = Powers::build(&args).expect("Should build correctly");
        let question = powers.generate_question();
        assert_eq!(question.question(), "2^1");
        assert_eq!(question.correct_answer(), "2");
        assert!(question.is_answer_correct("2"));
    }

    #[test]
    fn multiple_question_generation() {
        let args = ["--base=3".to_string()];
        let powers = Powers::build(&args).expect("Should build correctly");
        let questions = powers.generate_questions(10);
        assert_eq!(questions.len(), 10);
        assert!(questions
            .iter()
            .all(|question| question.question().starts_with("3^")));
    }

    #[test]
    fn print_help_only() {
        let args = [];
        let powers = Powers::build(&args).expect("Should build correctly");
        assert!(!powers.wants_to_print_help());

        let args = [
            "-b".to_string(),
            "4".to_string(),
            "-h".to_string(),
            "--upper-boundary=10".to_string(),
        ];
        let powers = Powers::build(&args).expect("Should build correctly");
        assert!(powers.wants_to_print_help());
    }

    #[test]
    fn help_text() {
        let args = ["-h".to_string()];
        let powers = Powers::build(&args).expect("Should build correctly");
        let help_text = powers.get_help_text();
        assert!(help_text.contains("Powers options"));
        assert!(help_text.contains("Usage"));

        // Ensure all flags are included
        assert!(help_text.contains("-h, --help"));
        assert!(help_text.contains("-b, --base"));
        assert!(help_text.contains("-l, --lower-boundary"));
        assert!(help_text.contains("-u, --upper-boundary"));
    }
}
