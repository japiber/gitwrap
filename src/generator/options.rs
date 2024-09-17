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

pub enum CmdOptionKind {
    CmdSimple(String, String),
    CmdEqualNoOptional(String, String, String),
    CmdEqualOptionalWithoutName(String, String),
    CmdEqualWithoutName(String, String),
    CmdEqualOptionalWithName(String, String, String),
    CmdWithParameter(String, String, String),
    CmdWithOptionalParameter(String, String, String),
    CmdNone
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
            _ => CmdOptionKind::CmdNone
        }
    } else {
        CmdOptionKind::CmdNone
    }
}

fn match_cmd_simple(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_SIMPLE).unwrap();
    for (_, [f1, f2]) in re.captures_iter(option).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdSimple(String::from(f1), normalize(f2))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_equal_no_optional(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_NO_OPTIONAL).unwrap();
    for (_, [f1, f2, f3]) in re.captures_iter(option).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdEqualNoOptional(String::from(f1), normalize(f2), normalize_argument(f3))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_equal_optional_without_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_OPTIONAL_WITHOUT_NAME).unwrap();
    for (_, [f1, f2]) in re.captures_iter(option).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdEqualOptionalWithoutName(String::from(f1), normalize(f2))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_equal_without_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_WITHOUT_NAME).unwrap();
    for (_, [f1, f2]) in re.captures_iter(option).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdEqualWithoutName(String::from(f1), normalize(f2))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_equal_optional_with_name(option: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_EQUAL_OPTIONAL_WITH_NAME).unwrap();
    for (_, [f1, f2, f3]) in re.captures_iter(option).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdEqualOptionalWithName(String::from(f1), normalize(f2), normalize_argument(f3))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_with_parameter(parameter: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_WITH_PARAMETER).unwrap();
    for (_, [f1, f2, f3]) in re.captures_iter(parameter).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdWithParameter(String::from(f1), normalize(f2), normalize_argument(f3))
    }

    CmdOptionKind::CmdNone
}

fn match_cmd_with_optional_parameter(parameter: &str) -> CmdOptionKind {
    let re = Regex::new(EXP_CMD_WITH_OPTIONAL_PARAMETER).unwrap();
    for (_, [f1, f2, f3]) in re.captures_iter(parameter).map(|caps| caps.extract()) {
        return CmdOptionKind::CmdWithOptionalParameter(String::from(f1), normalize(f2), normalize_argument(f3))
    }

    CmdOptionKind::CmdNone
}

pub fn normalize(cmd: &str) -> String {
    let re = Regex::new(r"\s|-").unwrap();
    String::from(re.replace_all(cmd, "_")).to_lowercase()
}

pub fn normalize_argument(arg: &str) -> String {
    format!("{}_arg", normalize(arg))
}


fn options_regex_set() -> RegexSet {
    RegexSetBuilder::new(&[
        EXP_CMD_SIMPLE,
        EXP_CMD_EQUAL_NO_OPTIONAL,
        EXP_CMD_EQUAL_OPTIONAL_WITHOUT_NAME,
        EXP_CMD_EQUAL_WITHOUT_NAME,
        EXP_CMD_EQUAL_OPTIONAL_WITH_NAME,
        EXP_CMD_WITH_PARAMETER,
        EXP_CMD_WITH_OPTIONAL_PARAMETER
    ]).build().unwrap()
}