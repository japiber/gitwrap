use regex::{Regex, RegexSet, RegexSetBuilder};


const EXP_CMD_SIMPLE: &str = r#"^(-{1,2}([\w\-]+))$"#;

// --strategy=<strategy>
const EXP_CMD_EQUAL_NO_OPTIONAL: &str = r#"^(-{1,2}([\w\-]+))=<([\w\- ]+)>$"#;

// --no-recurse-submodules[=yes|on-demand|no]
const EXP_CMD_EQUAL_OPTIONAL_WITHOUT_NAME: &str = r#"^(-{1,2}([\w\-]+))\[=[\w\-()|]+]$"#;

// --recurse-submodules-default=[yes|on-demand]
// --sign=(true|false|if-asked)
const EXP_CMD_EQUAL_WITHOUT_NAME: &str = r#"^(-{1,2}([\w\-]+))=[\[(][\w\-|()]+[])]$"#;

// --log[=<n>]
const EXP_CMD_EQUAL_OPTIONAL_WITH_NAME: &str = r#"^(-{1,2}([\w\-]+))\[=<([\w\-)]+)>]$"#;

// --foo <bar>
const EXP_CMD_WITH_PARAMETER: &str = r#"^(-{1,2}([\w\-]+)) ?<([\w\-)]+)>$"#;

// --foo [bar]
const EXP_CMD_WITH_OPTIONAL_PARAMETER: &str = r#"^(-{1,2}([\w\-]+)) ?\[([\w\-)]+)]$"#;

// <branch_name>
const EXP_CMD_VALUE_PARAMETER: &str = r#"^<([\w\-)]+)>$"#;

const CMD_PATTERNS: [&str; 8] = [
        EXP_CMD_SIMPLE,
        EXP_CMD_EQUAL_NO_OPTIONAL,
        EXP_CMD_EQUAL_OPTIONAL_WITHOUT_NAME,
        EXP_CMD_EQUAL_WITHOUT_NAME,
        EXP_CMD_EQUAL_OPTIONAL_WITH_NAME,
        EXP_CMD_WITH_PARAMETER,
        EXP_CMD_WITH_OPTIONAL_PARAMETER,
        EXP_CMD_VALUE_PARAMETER,
];

pub enum CmdOptionKind {
    Simple(String, String),
    EqualNoOptional(String, String, String),
    EqualOptionalWithoutName(String, String),
    EqualWithoutName(String, String),
    EqualOptionalWithName(String, String, String),
    WithParameter(String, String, String),
    WithOptionalParameter(String, String, String),
    ValueParameter(String),
    None
}

pub(crate) fn option_kind(option: &str) -> CmdOptionKind {
    let options_set = options_regex_set();
    let match_set = options_set.matches(option);
    let matches: Vec<_> = match_set.clone().into_iter().collect();

    if matches.len() == 1 {
        match matches[0] {
            0 => match_cmd_simple(option),
            1 => match_cmd_equal_no_optional(option),
            2 => match_cmd_equal_optional_without_name(option),
            3 => match_cmd_equal_without_name(option),
            4 => match_cmd_equal_optional_with_name(option),
            5 => match_cmd_with_parameter(option),
            6 => match_cmd_with_optional_parameter(option),
            7 => match_cmd_value_parameter(option),
            _ => CmdOptionKind::None
        }
    } else {
        CmdOptionKind::None
    }
}

fn match_cmd_simple(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_SIMPLE).unwrap();
    if let Some((_, [f1, f2])) = re.captures_iter(option).map(|caps| caps.extract()).next() {
        return CmdOptionKind::Simple(String::from(f1), String::from(f2))
    }

    CmdOptionKind::None
}

fn match_cmd_equal_no_optional(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_NO_OPTIONAL).unwrap();
    if let Some((_, [f1, f2, f3])) = re.captures_iter(option).map(|caps| caps.extract()).next() {
        return CmdOptionKind::EqualNoOptional(String::from(f1), String::from(f2), normalize_argument(f3))
    }

    CmdOptionKind::None
}

fn match_cmd_equal_optional_without_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_OPTIONAL_WITHOUT_NAME).unwrap();
    if let Some((_, [f1, f2])) = re.captures_iter(option).map(|caps| caps.extract()).next() {
        return CmdOptionKind::EqualOptionalWithoutName(String::from(f1), String::from(f2))
    }

    CmdOptionKind::None
}

fn match_cmd_equal_without_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_WITHOUT_NAME).unwrap();
    if let Some((_, [f1, f2])) = re.captures_iter(option).map(|caps| caps.extract()).next() {
        return CmdOptionKind::EqualWithoutName(String::from(f1), String::from(f2))
    }

    CmdOptionKind::None
}

fn match_cmd_equal_optional_with_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_OPTIONAL_WITH_NAME).unwrap();
    if let Some((_, [f1, f2, f3])) = re.captures_iter(option).map(|caps| caps.extract()).next() {
        return CmdOptionKind::EqualOptionalWithName(String::from(f1), String::from(f2), normalize_argument(f3))
    }

    CmdOptionKind::None
}

fn match_cmd_with_parameter(parameter: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_WITH_PARAMETER).unwrap();
    if let Some((_, [f1, f2, f3])) = re.captures_iter(parameter).map(|caps| caps.extract()).next() {
        return CmdOptionKind::WithParameter(String::from(f1), String::from(f2), normalize_argument(f3))
    }

    CmdOptionKind::None
}

fn match_cmd_with_optional_parameter(parameter: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_WITH_OPTIONAL_PARAMETER).unwrap();
    if let Some((_, [f1, f2, f3])) = re.captures_iter(parameter).map(|caps| caps.extract()).next() {
        return CmdOptionKind::WithOptionalParameter(String::from(f1), String::from(f2), normalize_argument(f3))
    }

    CmdOptionKind::None
}

fn match_cmd_value_parameter(parameter: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_VALUE_PARAMETER).unwrap();
    if let Some((_, [f1])) = re.captures_iter(parameter).map(|caps| caps.extract()).next() {
        return CmdOptionKind::ValueParameter(String::from(f1))
    }

    CmdOptionKind::None    
}

pub fn normalize(cmd: &str) -> String {
    let re = Regex::new(r"\s|-").unwrap();
    String::from(re.replace_all(cmd, "_")).to_lowercase()
}

pub fn normalize_argument(arg: &str) -> String {
    format!("{}_arg", normalize(arg))
}


fn options_regex_set() -> RegexSet {
    RegexSetBuilder::new(CMD_PATTERNS).build().unwrap()
}