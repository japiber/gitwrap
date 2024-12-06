use upon::Engine;


pub const TEMPLATE_MOD_RS: &str = "mod_rs";
pub const TEMPLATE_OPTION_SIMPLE: &str = "simple";
pub const TEMPLATE_OPTION_DOC_COMMENTS: &str = "doc_comments";
pub const TEMPLATE_OPTION_EQUAL_NO_OPTIONAL: &str = "equal_no_optional";
pub const TEMPLATE_OPTION_EQUAL_OPTIONAL: &str = "equal_optional";
pub const TEMPLATE_OPTION_WITH_PARAMETER: &str = "with_parameter";
pub const TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER: &str = "with_optional_parameter";
pub const TEMPLATE_OPTION_VALUE_PARAMETER: &str = "value_parameter";



static GIT_TEMPLATES_COMMON: &[(&str, &str)] = &[
    (
        TEMPLATE_MOD_RS,
        r#"use std::process::Command;
use crate::wrap_command::{git, WrapCommand};

mod options;
pub use options::*;

pub fn {{ command_name }}(current_dir: &str) -> WrapCommand {
    let mut command = git(current_dir);
    command.option(Box::new(move |cmd: &mut Command| { cmd.arg(String::from("{{ git_command }}")); }));
    command
}"#
    ),
    (
        TEMPLATE_OPTION_DOC_COMMENTS,
        r#"{% for doc in descriptions %}/// {{ doc }}
{% endfor %}/// {{ arguments }}"#
    ),
    (
        TEMPLATE_OPTION_SIMPLE,
        r#"pub fn {{ method_name }}() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("{{ git_option }}");
    })
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    let l_{{ option_argument }} = format!("{{ git_option }}={}", {{ option_argument }});
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_{{ option_argument }}.as_str());
    })
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    let l_{{ option_argument }} = if {{ option_argument }}.is_empty() {
        String::from("{{ git_option }}")
    } else {
        format!("{{ git_option }}={}", {{ option_argument }})
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_{{ option_argument }}.as_str());
    })
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    let l_{{ option_argument }} = String::from({{ option_argument }});
    Box::new(move |cmd: &mut Command| {
        cmd.arg("{{ git_option }}");
        cmd.arg(l_{{ option_argument }}.as_str());
    })
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    let l_{{ option_argument }} = if {{ option_argument }}.is_empty() {
        String::from("{{ git_option }}")
    } else {
        format!("{{ git_option }} {}", {{ option_argument }})
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_{{ option_argument }}.as_str());
    })
}"#
    ),
    (
        TEMPLATE_OPTION_VALUE_PARAMETER,
        r#"pub fn {{ method_name }}(value: &str) -> FnOptionArg {
    let l_value = String::from(value);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}"#
    )
];

pub fn command_templates() -> Engine<'static> {
    let mut engine = Engine::new();
    
    for (name, source) in GIT_TEMPLATES_COMMON {
        let _ = engine.add_template(*name, *source);
    }
    
    engine
}

