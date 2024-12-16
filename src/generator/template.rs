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

{% for doc in descriptions %}/// {{ doc }}
{% endfor %}/// [Git doc]({{ doc_url }})
pub fn {{ command_name }}(current_dir: Option<&str>) -> WrapCommand {
    git("{{ git_command }}", current_dir)
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
    optionarg::simple("{{ git_option }}")
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    optionarg::equal_no_optional("{{ git_option }}", {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    optionarg::equal_optional("{{ git_option }}", {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    optionarg::with_parameter("{{ git_option }}", {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    optionarg::with_optional_parameter("{{ git_option }}", {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_VALUE_PARAMETER,
        r#"pub fn {{ method_name }}({{ value_parameter }}: &str) -> FnOptionArg {
    optionarg::value_parameter({{ value_parameter }})
}"#
    ),
    (
        TEMPLATE_GIT_COMMAND_FILE,
        r#"use crate::wrap_command::WrapCommand;

pub fn git(cmd: &str, current_dir: Option<&str>) -> WrapCommand {
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
            let mut command = git("{{ git_command }}", $path);
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


