pub struct Config;

// --number-of-questions -n
// --show-live-statistics -s
// --behavior-on-error -b -> repeat, continue, showcorrect
//
// COMMAND: times_table, powers, doomsday

impl Config {
    pub fn build(_args: &[String]) -> Result<Config, &'static str> {
        // 1) Parse general args

        // 2) Determine specific mode/command

        // 3) Parse command arguments

        Ok(Config)
    }
}
