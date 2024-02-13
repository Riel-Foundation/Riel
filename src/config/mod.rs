pub mod global;

pub struct Config {
    pub username: String,
    pub email: String
}
pub fn empty_config() -> Config {
    Config {
        username: String::from(""),
        email: String::from("")
    }
}
impl Config {
    pub fn from_string(string: String) -> Config {
        let lines = string.split("\n");
        let username =
            lines.clone().filter(|x| x.starts_with("username:")).collect::<Vec<&str>>()[0]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        let email =
            lines.clone().filter(|x| x.starts_with("email:")).collect::<Vec<&str>>()[0]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        Config {
            username,
            email
        }
    }
}