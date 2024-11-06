use crate::Config;

pub struct Application;

impl Application {
    pub fn run(_config: &Config) -> Result<(), &'static str> {
        println!("Your knowledge is improving!");

        Ok(())
    }
}
