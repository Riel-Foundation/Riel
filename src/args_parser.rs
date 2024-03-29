#[derive(Clone)]
pub struct ParsedArgsObject {
    command: String,
    subcommands: Vec<String>,
    options: Vec<String>,
}
impl ParsedArgsObject {
    pub fn command(&self) -> String {
        self.command.clone()
    }
    pub fn subcommands(&self) -> Vec<String> {
        self.subcommands.clone()
    }
    pub fn options(&self) -> Vec<String> {
        self.options.clone()
    }
    // Does cloning matter? Maybe check it, TODO
}

pub(crate) fn parse_args(args: Vec<String>) -> Option<ParsedArgsObject> {
    let command: String;
    let options: Vec<String> = args
        .iter()
        .filter(|x| x.starts_with("-"))
        .map(|x| x.to_string())
        .collect();
    let possible_commands: Vec<String> = args
        .iter()
        .filter(|x| !x.starts_with("-"))
        .map(|x| x.to_string())
        .collect();
    let coincidences: i16 = possible_commands.len() as i16;
    command = if coincidences == 0 {
        String::new()
    } else {
        possible_commands[0].clone()
    };
    let subcommands: Vec<String> = possible_commands
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect();
    return Some(ParsedArgsObject {
        command,
        subcommands,
        options,
    });
}
