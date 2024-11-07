use super::{QuestionAndAnswer, SkillBase};

pub const CMD_POWERS: &str = "powers";

#[derive(Debug)]
pub struct Powers {
    base: u32,
    lower_boundary: u32,
    upper_boundary: u32,
}

impl Powers {
    pub fn build(args: &[String]) -> Result<Self, String> {
        Ok(Powers {
            base: 2,
            lower_boundary: 3,
            upper_boundary: 4
        })
    }

    fn usage() -> String {
        todo!()
    }

    fn help_prompt() -> String {
        todo!()
    }

    fn print_help() {
        todo!()
    }
}

impl SkillBase for Powers {
    fn generate_questions_and_answers(&self, count: u32) -> Vec<QuestionAndAnswer> {
        todo!()
    }


}
