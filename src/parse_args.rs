use std::process::exit;

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub value: String,
}

impl Arg {
    fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

pub fn parse_args(args: &Vec<String>) -> (Vec<Arg>, Vec<String>) {
    let mut flag_vec: Vec<Arg> = vec![];
    let mut arg_vec: Vec<String> = vec![];
    let mut count = 0;
    for arg in args {
        if arg.starts_with("-") {
            if arg.starts_with("--") {
                if arg == "--help" {
                    flag_vec.push(Arg::new(arg[2..].to_string(), "".to_string()));
                    count += 1;
                } else {
                    let pos = arg.find('=');
                    let pos = match pos {
                        Some(value) => value,
                        None => {
                            println!("Must be in the form --name=value");
                            exit(1);
                        }
                    };
                    flag_vec.push(Arg::new(
                        arg[2..pos].to_string(),
                        arg[pos + 1..].to_string(),
                    ));
                    count += 1;
                }
            } else {
                flag_vec.push(Arg::new(arg[1..].to_string(), "".to_string()));
                count += 1;
            }
        } else {
            if count > 0 {
                arg_vec.push(arg.to_string());
            }
            count += 1;
        }
    }
    (flag_vec, arg_vec)
}

pub fn get_flag_value(flag_name: &str, args: &Vec<Arg>) -> Option<String> {
    let name = flag_name.to_string();
    for arg in args {
        if arg.name == name {
            return Some(arg.value.clone());
        }
    }
    None
}
