use crate::error::{ErrorLevel, LightError};
use crate::util::hyperstr::{GRAY, GREEN, RED, RESET};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::{self, current_exe};
use std::fmt::Write;
use std::process::exit;
use std::rc::Rc;

type Action = Rc<RefCell<dyn FnMut(Vec<String>)>>;
pub type ArgumentID = usize;

#[derive(Clone, Debug)]
pub struct ParsedArgument {
    values: Vec<String>,
}

impl ParsedArgument {
    fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn val(&self) -> &Vec<String> {
        &self.values
    }
}

#[derive(Clone, Debug)]
pub struct ParsedArgs {
    hm: HashMap<String, ParsedArgument>,
}

impl ParsedArgs {
    pub fn get(&self, key: impl AsRef<str>) -> Option<&ParsedArgument> {
        let key = format!("0#{}", key.as_ref());
        self.hm.get(&key)
    }

    pub fn has(&self, key: impl AsRef<str>) -> bool {
        self.get(key).is_some()
    }
}

#[derive(Default, Clone)]
pub struct Argument {
    matches: Vec<String>,
    numbers: usize,
    help: String,
    sub_arg: Option<usize>,
    id: ArgumentID,
    action: Option<Action>,
}

impl Argument {
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
            numbers: 0,
            help: "".into(),
            sub_arg: None,
            id: 0,
            action: None,
        }
    }

    pub fn alias<S: AsRef<str>>(mut self, name: S) -> Self {
        let name = name.as_ref().to_string();
        if !self.matches.contains(&name) {
            self.matches.push(name)
        }

        self
    }

    pub fn count(mut self, nv: usize) -> Self {
        self.numbers = nv;
        self
    }

    pub fn help<S: AsRef<str>>(mut self, help_string: S) -> Self {
        self.help = help_string.as_ref().into();
        self
    }

    pub(crate) fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

#[derive(Default, Clone)]
pub struct Args {
    stored: HashMap<String, Argument>,
    order: Vec<String>,
    last_id: usize,
    exit_on_help: bool,
    author: String,
    desc: String,
    name: String,
    help: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        let exe_name = match current_exe() {
            Ok(pb) => {
                if let Some(name) = pb.file_name() {
                    name.to_str().unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            }
            Err(_) => "".to_string(),
        };
        Self {
            stored: HashMap::new(),
            order: Vec::new(),
            last_id: 0,
            exit_on_help: true,
            author: "".to_string(),
            desc: "".to_string(),
            name: exe_name,
            help: None,
        }
    }
    pub fn set_author<S: AsRef<str>>(mut self, v: S) -> Self {
        self.author = v.as_ref().to_string();
        self
    }

    pub fn set_desc<S: AsRef<str>>(mut self, v: S) -> Self {
        self.desc = v.as_ref().to_string();
        self
    }
    pub fn set_name<S: AsRef<str>>(mut self, v: S) -> Self {
        self.name = v.as_ref().to_string();
        self
    }

    fn generate_id(&mut self) -> usize {
        self.last_id += 1;
        self.last_id
    }

    fn add_to_map(&mut self, mut argument: Argument) -> ArgumentID {
        let argument_id = self.generate_id();
        argument.set_id(argument_id);
        let matches = argument.matches.clone();
        for name in matches.iter() {
            let sub_arg = argument.sub_arg.unwrap_or(0);
            let new_name = format!("{}#{}", sub_arg, name.clone());
            let _ = self.stored.insert(new_name.clone(), argument.clone());
            self.order.push(name.clone())
        }

        argument_id
    }

    pub fn add<S: AsRef<str>>(
        &mut self,
        matches: S,
        num_values: usize,
        help_message: S,
    ) -> ArgumentID {
        let argument = Argument::new()
            .alias(matches)
            .count(num_values)
            .help(help_message);
        self.add_to_map(argument)
    }

    pub fn add_arg(&mut self, argument: Argument) -> ArgumentID {
        self.add_to_map(argument)
    }

    fn create_help(&self) -> String {
        let mut result_string = String::new();

        let longest = self
            .stored
            .values()
            .into_iter()
            .map(|t| {
                let mut temp = t.matches.join(" | ");
                if t.numbers > 0 {
                    write!(temp, " [{} argument/s]", t.numbers).unwrap();
                }

                temp.len()
            })
            .max();

        write!(result_string, "{}", self.name).unwrap_or(());
        writeln!(result_string, " \x1b[90m~\x1b[0m {}", self.desc).unwrap_or(());
        writeln!(
            result_string,
            "\x1b[0mAuthor \x1b[90m~ \x1b[0m{}",
            self.author
        )
        .unwrap_or(());
        writeln!(
            result_string,
            "\x1b[0mUsage \x1b[90m~\n    \x1b[32m$ \x1b[0m{} -[cmd] [arg/s...]",
            self.name
        )
        .unwrap_or(());

        let longest_value_len = match longest {
            Some(l) => l + 4,
            None => 4,
        };
        let mut max_level = 0;

        writeln!(result_string, "\x1b[0mArguments \x1b[90m~").unwrap_or(());

        let mut argument_vec: Vec<(&Argument, usize)> = Vec::new();
        for name in self.order.iter() {
            let each = self
                .stored
                .values()
                .find(|temp| temp.matches.contains(name))
                .unwrap();
            if !argument_vec
                .iter()
                .any(|(argument, _)| argument.id == each.id)
            {
                if let Some(sub_arg_of) = each.sub_arg {
                    if let Some((index, (_, level))) = argument_vec
                        .iter()
                        .enumerate()
                        .find(|(_, (t, _))| t.id == sub_arg_of)
                    {
                        if level + 1 > max_level {
                            max_level = level + 1;
                        }
                        argument_vec.insert(index + 1, (each, level + 1));
                    }
                } else {
                    argument_vec.push((each, 0))
                }
            }
        }

        for (argument, level) in argument_vec.iter() {
            let mut lvl = String::new();
            (0..(level * 4)).for_each(|_| lvl.push(' '));

            let mut matches = argument.matches.join(" | ");
            if argument.numbers > 0 {
                write!(matches, " [{} argument/s]", argument.numbers).unwrap();
            }

            while matches.len() != longest_value_len + (max_level * 4) - lvl.len() {
                matches.push(' ');
            }

            writeln!(result_string, "    {lvl}{matches} {}", argument.help).unwrap_or(());
        }

        let mut help = String::from("--help, -h");
        while help.len() != longest_value_len + max_level * 4 {
            help.push(' ');
        }

        write!(result_string, "    {help} Prints this help message.").unwrap_or(());

        result_string
    }

    fn help_and_exit(&self) {
        if let Some(help) = &self.help {
            println!("{help}");
        } else {
            let help_string = self.create_help();

            println!("{help_string}");
        }

        if self.exit_on_help {
            exit(0);
        }
    }

    pub fn parse(&mut self, from: Option<Vec<&str>>) -> Result<ParsedArgs, LightError> {
        let args: Vec<String>;
        if let Some(from_vec) = from {
            args = from_vec.iter().map(|each| each.to_string()).collect();
        } else {
            args = env::args().collect();
        }

        let mut hm = HashMap::new();
        let mut idem = HashMap::new();

        let mut ctx = 0;

        for (index, arg) in args.iter().enumerate() {
            if arg == "--help" || arg == "-h" {
                self.help_and_exit()
            }
            let query = format!("{ctx}#{arg}");
            let query2 = format!("0#{arg}");

            if self.stored.get(&query).is_some() {
                if let Some(argument) = self.stored.get(&query) {
                    ctx = argument.id;
                    let mut i = index;
                    let mut count = 0;
                    let mut values: Vec<String> = Vec::new();

                    while count < argument.numbers {
                        i += 1;
                        if i >= args.len() {
                            break;
                        }
                        if args[i].starts_with('-') {
                            break;
                        }
                        values.push(args[i].to_string());
                        count += 1;
                    }

                    if count < argument.numbers {
                        return Err(LightError::new(
                            format!(
                                "Argument {GRAY}'{GREEN}{}{GRAY}'{RESET} requires {GRAY}'{GREEN}{}{GRAY}'{RESET} parameter(s), but only {GRAY}'{RED}{count}{GRAY}'{RESET} were provided.",
                                argument.matches[0], argument.numbers
                            ),
                            ErrorLevel::Error,
                        ));
                    }

                    if let Some(action) = &argument.action {
                        action.borrow_mut()(values.clone());
                    }

                    let pa = ParsedArgument::new(values);
                    hm.insert(query, pa.clone());
                    idem.insert(argument.id, pa);
                }
            } else if let Some(argument) = self.stored.get(&query2) {
                ctx = argument.id;
                let mut i = index;
                let mut count = 0;
                let mut values: Vec<String> = Vec::new();

                while i < index + argument.numbers {
                    i += 1;
                    if i == args.len() {
                        break;
                    }
                    let value = &args[i];

                    let q1 = format!("{ctx}#{value}");
                    let q2 = format!("0#{value}");

                    if self.stored.get(&q1).is_some() || self.stored.get(&q2).is_some() {
                        break;
                    } else {
                        values.push(value.to_string());
                        count += 1;
                    }
                }

                if count < argument.numbers {
                    return Err(LightError::new(
                        format!(
                            "Argument {GRAY}'{GREEN}{}{GRAY}'{RESET} requires {GRAY}'{GREEN}{}{GRAY}'{RESET} parameter(s), but only {GRAY}'{RED}{count}{GRAY}'{RESET} were provided.",
                            argument.matches[0], argument.numbers
                        ),
                        ErrorLevel::Error,
                    ));
                }

                if let Some(action) = &argument.action {
                    action.borrow_mut()(values.clone());
                }

                let pa = ParsedArgument::new(values);
                hm.insert(query2, pa.clone());
                idem.insert(argument.id, pa);
            } else if let Some(argument) = self.stored.values().find(|t| t.matches.contains(arg)) {
                if let Some(parent) = argument.sub_arg {
                    let parent = self.stored.values().find(|t| t.id == parent).unwrap();
                    let parent_match = &parent.matches[0];
                    return Err(LightError::new(format!(
                        "Argument {GREEN}{parent_match}{RESET} requires a sub argument, but {RED}{arg}{RESET} was provided."
                    ), ErrorLevel::Error));
                }
            }
        }

        Ok(ParsedArgs { hm })
    }
}
