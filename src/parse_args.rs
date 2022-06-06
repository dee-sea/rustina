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

pub fn usage(cmd: &str) {
    println!(
        "Usage: {} [Options]

Options:
        --server=interface  # Bind server to IP address of provided interface
                            # Default value \"lo\"
        --manual=ipadr      # Manual mode : Only generate docx and html files without binding a server
                            # Default value \"127.0.0.1\"
        --port=portnumber   # Bind server to provided port
                            # Default value \"8080\"
        --webroot=webpath   # Specify where files are generated
                            # Default value \"./www\"
        --html=filename     # Specify the name of the html generated file
                            # Default value \"exploit.html\"
        --docx=filename     # Specify the name of the docx generated file
                            # Default value \"document.docx\"
        --binary=binarypath # Make a payload to execue binarypath on the victime computer
                            # Default value \"\\\\\\\\windows\\\\\\\\system32\\\\\\\\calc\"
                            # Binary path should not include the file extention e.g. .exe
                            # On linux binarypath should be double excaped:
                            # e.g. \\\\\\\\windows\\\\\\\\system32\\\\\\\\calc
                            # On windows binarypath should be excaped:
                            # e.g. \\\\windows\\\\system32\\\\calc
        -h or --help        # print this message ", cmd);
}

pub fn parse_args(args: &Vec<String>, params: &[&str]) -> (Vec<Arg>, Vec<String>) {
    let mut flag_vec: Vec<Arg> = vec![];
    let mut arg_vec: Vec<String> = vec![];
    let mut count = 0;
    for arg in args {
        if arg.starts_with("-") {
            if arg.starts_with("--") {
                let pos = arg.find('=');
                let pos = match pos {
                    Some(value) => value,
                    None => {
                        println!("Must be in the form --name=value");
                        exit(1);
                    }
                };
                if !params.contains(&&arg[..pos]) {
                    println!("Unknown parameter: {}\n", arg);
                    usage(&args[0]);
                    exit(1);
                }
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
