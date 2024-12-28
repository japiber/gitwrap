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
pub const TEMPLATE_OPTION_NAME_CONSTANT: &str = "option_name_constant";



static GIT_TEMPLATES_COMMON: &[(&str, &str)] = &[
    (
        TEMPLATE_MOD_RS,
        r#"mod options;
pub use options::*;

{% if is_command %}use crate::wrap_command::WrapCommand;
use crate::git;

pub const GIT_COMMAND: &str = "{{ git_command }}";

{% for doc in descriptions %}/// {{ doc }}
{% endfor %}/// [Git doc]({{ doc_url }})
pub fn {{ command_name }}() -> WrapCommand {
    git(GIT_COMMAND)
}{% endif %}
"#
    ),
    (
        TEMPLATE_OPTION_NAME_CONSTANT,
        r#"pub const {{ constant_name }}: &str = "{{ git_option }}";"#
    ),
    (
        TEMPLATE_OPTION_DOC_COMMENTS,
        r#"{% for doc in descriptions %}/// {{ doc }}
{% endfor %}/// {{ arguments }}"#
    ),
    (
        TEMPLATE_OPTION_SIMPLE,
        r#"pub fn {{ method_name }}() -> FnOptionArg {
    option_arg::simple({{ constant_name }})
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    option_arg::equal_no_optional({{ constant_name }}, {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_EQUAL_OPTIONAL,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    option_arg::equal_optional({{ constant_name }}, {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    option_arg::with_parameter({{ constant_name }}, {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
        r#"pub fn {{ method_name }}({{ option_argument }}: &str) -> FnOptionArg {
    option_arg::with_optional_parameter({{ constant_name }}, {{ option_argument }})
}"#
    ),
    (
        TEMPLATE_OPTION_VALUE_PARAMETER,
        r#"pub fn {{ method_name }}({{ value_parameter }}: &str) -> FnOptionArg {
    option_arg::value_parameter({{ value_parameter }})
}"#
    ),
    (
        TEMPLATE_GIT_COMMAND_FILE,
        r#"use std::sync::Arc;
use crate::wrap_command::{FnOptionArg, WrapCommand};

pub fn git(cmd: &str) -> WrapCommand {
    let l_cmd = String::from(cmd);
    WrapCommand::new("git")
        .add_option(FnOptionArg(Arc::new(move || vec![String::from(l_cmd.as_str())])))
}
"#
    ),
    (
        TEMPLATE_GIT_COMMAND_MACRO,
        r#"#[macro_export]
macro_rules! {{ command_name }} {
    () => (
        {
            $crate::git({{ command_name }}::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git({{ command_name }}::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git({{ command_name }}::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git({{ command_name }}::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
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


