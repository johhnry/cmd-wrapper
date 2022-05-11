use std::{collections::HashMap, env};

use crate::substring::Substring;

#[derive(Debug, Clone)]
pub struct ArgParseError {
    pub arg: String,
    pub reason: String,
}

pub struct Argument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub multiple: bool,
}

pub struct ArgumentParser {
    arguments: Vec<Argument>,
}

type ParseResult = Result<HashMap<String, Vec<String>>, ArgParseError>;

impl ArgumentParser {
    pub fn new() -> ArgumentParser {
        ArgumentParser {
            arguments: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, arg: Argument) {
        self.arguments.push(arg);
    }

    pub fn parse_args(&self, args: Vec<String>) -> ParseResult {
        let mut parsed_arguments: HashMap<String, Vec<String>> = HashMap::new();

        for arg in args {
            if !arg.starts_with("--") {
                return Err(ArgParseError {
                    arg,
                    reason: "does not start with double dashes (--)".into(),
                });
            }

            let equal_index = match arg.find('=') {
                Some(i) => i,
                None => {
                    return Err(ArgParseError {
                        arg,
                        reason: "does not contain any equal sign (=)".into(),
                    })
                }
            };

            let chars_count = arg.chars().count();

            // Check if no argument name (--=) or no value (--a=)
            if equal_index == 2 || equal_index == chars_count - 1 {
                return Err(ArgParseError {
                    arg,
                    reason: "is incomplete or value is missing".into(),
                });
            }

            let key = arg.substring(2, equal_index);
            let value = arg.substring(equal_index + 1, chars_count);

            if parsed_arguments.contains_key(&key) {
                parsed_arguments.get_mut(&key).unwrap().push(value);
            } else {
                parsed_arguments.insert(key, vec![value]);
            }
        }

        // Check for required arguments
        for argument in &self.arguments {
            if argument.required && parsed_arguments.get(&argument.name).is_none() {
                return Err(ArgParseError {
                    arg: argument.name.clone(),
                    reason: "is required".into(),
                });
            }
        }

        Ok(parsed_arguments)
    }

    pub fn parse(&self) -> ParseResult {
        // Skip one argument since it's the executable itself
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse_args(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_arguments() {
        let parser = ArgumentParser::new();
        assert_eq!(parser.parse_args(vec![]).ok(), Some(HashMap::new()));
    }

    #[test]
    fn no_double_dashes() {
        assert!(ArgumentParser::new()
            .parse_args(vec!["-a=4".into()])
            .is_err());
    }

    #[test]
    fn no_arg_name() {
        assert!(ArgumentParser::new()
            .parse_args(vec!["--a=567".into(), "--=error".into()])
            .is_err());
    }

    #[test]
    fn no_value() {
        assert!(ArgumentParser::new()
            .parse_args(vec!["--pre=".into()])
            .is_err());
    }

    #[test]
    fn with_required_arguments() {
        let mut parser = ArgumentParser::new();

        parser.add_argument(Argument {
            name: "test".into(),
            description: "This is a test parameter".into(),
            required: true,
            multiple: false,
        });

        parser.add_argument(Argument {
            name: "other".into(),
            description: "Other argument".into(),
            required: false,
            multiple: false,
        });

        assert!(parser.parse_args(vec!["--test=ls -la".into()]).is_ok());
        assert!(parser.parse_args(vec!["--other=whoami".into()]).is_err());
        assert!(parser
            .parse_args(vec!["--test=whoami".into(), "--other=hello".into()])
            .is_ok());
    }

    #[test]
    fn with_multiple_arguments() {
        let mut parser = ArgumentParser::new();

        parser.add_argument(Argument {
            name: "mult".into(),
            description: "Multiple".into(),
            multiple: true,
            required: true,
        });

        let mut expected_args: HashMap<String, Vec<String>> = HashMap::new();
        expected_args.insert("mult".into(), vec!["1".into(), "2".into()]);

        assert_eq!(
            parser
                .parse_args(vec!["--mult=1".into(), "--mult=2".into()])
                .ok(),
            Some(expected_args)
        );
    }
}
