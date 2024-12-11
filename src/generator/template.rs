use upon::Engine;


pub const TEMPLATE_MOD_RS: &str = "mod_rs";
pub const TEMPLATE_OPTION_SIMPLE: &str = "simple";
pub const TEMPLATE_OPTION_DOC_COMMENTS: &str = "doc_comments";
pub const TEMPLATE_OPTION_EQUAL_NO_OPTIONAL: &str = "equal_no_optional";
pub const TEMPLATE_OPTION_EQUAL_OPTIONAL: &str = "equal_optional";
pub const TEMPLATE_OPTION_WITH_PARAMETER: &str = "with_parameter";
pub const TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER: &str = "with_optional_parameter";
pub const TEMPLATE_OPTION_VALUE_PARAMETER: &str = "value_parameter";
pub const TEMPLATE_GIT_COMMAND_FILE: &str = "git_command_file";
pub const TEMPLATE_GIT_COMMAND_MACRO: &str = "git_command_macro";



static GIT_TEMPLATES_COMMON: &[(&str, &str)] = &[
    (
        TEMPLATE_MOD_RS,
        r#"use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn {{ command_name }}(current_dir: &str) -> WrapCommand {
    git(current_dir, "{{ git_command }}")
}
"#
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
    ),
    (
        TEMPLATE_GIT_COMMAND_FILE,
        r#"use crate::wrap_command::WrapCommand;

pub fn git(current_dir: &str, cmd: &str) -> WrapCommand {
    let mut command = WrapCommand::new("git", current_dir);
    let l_cmd = String::from(cmd);
    command.option(Box::new(move |c: &mut  std::process::Command| { c.arg(l_cmd.as_str()); }));
    command
}
"#
    ),
    (
        TEMPLATE_GIT_COMMAND_MACRO,
        r#"#[macro_export]
macro_rules! {{ command_name }} {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git($path, "{{ git_command }}");
            $(
                command.option($options);
            )*
            command
        }
     }
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

