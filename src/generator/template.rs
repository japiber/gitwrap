use upon::Engine;


pub const TEMPLATE_MOD_RS: &str = "mod_rs";
pub const TEMPLATE_OPTION_SIMPLE: &str = "simple";
pub const TEMPLATE_OPTION_DOC_COMMENTS: &str = "doc_comments";
pub const TEMPLATE_OPTION_EQUAL_NO_OPTIONAL: &str = "equal_no_optional";
pub const TEMPLATE_OPTION_EQUAL_OPTIONAL: &str = "equal_optional";
pub const TEMPLATE_OPTION_WITH_PARAMETER: &str = "with_parameter";
pub const TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER: &str = "with_optional_parameter";
pub const TEMPLATE_OPTION_VALUE_PARAMETER: &str = "value_parameter";
pub const TEMPLATE_COMMAND : &str = "command";
pub const TEMPLATE_GIT_BASE_COMMAND: &str = "git_base_command";

static GIT_TEMPLATES_COMMON: &[(&str, &str)] = &[
    (
        TEMPLATE_MOD_RS,
        "mod {{ command_name }}_options;\npub use {{ command_name }}_options::*;\n"
    ),
    (
        TEMPLATE_OPTION_DOC_COMMENTS,
        "{% for doc in descriptions %}/// {{ doc }}\n{% endfor %}/// {{ arguments }}"
    ),
    (
        TEMPLATE_OPTION_SIMPLE,
        "pub fn {{ method_name }}() -> CommandOption<'static> {\n    Box::new(|g: &mut CommandExecutor| g.add_option(\"{{ git_option }}\"))\n}"
    ),
    (
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        "pub fn {{ method_name }}({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}={}\", {{ option_argument }})))\n}"
    ),
    (
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        "pub fn {{ method_name }}({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}={}\", {{ option_argument }})))\n}"
    ),
    (
        TEMPLATE_OPTION_EQUAL_OPTIONAL,
        "pub fn {{ method_name }}({{ option_argument }} :&str) -> CommandOption {\n    if {{ option_argument }}.is_empty() {\n        Box::new(|g: &mut CommandExecutor| g.add_option(\"{{ git_option }}\"))\n    } else {\n        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}={}\", {{ option_argument }})))\n    }\n}\n"
    ),
    (
        TEMPLATE_OPTION_WITH_PARAMETER,
        "pub fn {{ method_name }}({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| {\n        g.add_option(\"{{ git_option }}\");\n        g.add_option({{ option_argument }});\n    })\n}\n"
    ),
    (
        TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
        "pub fn {{ method_name }}({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| {\n         g.add_option(\"{{ git_option }}\");\n        if !{{ option_argument }}.is_empty() {\n            g.add_option({{ option_argument }});\n        }\n    })\n}\n"
    ),
    (
        TEMPLATE_OPTION_VALUE_PARAMETER,
        "pub fn {{ method_name }}(value :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| {\n         g.add_option(value);\n    })\n}\n"
    ),
    (
        TEMPLATE_COMMAND,
        "pub fn {{ command_name }}<'a, I>(options: I) -> ExecResult\nwhere\n    I: IntoIterator<Item=CommandOption<'a>>\n{\n    git_command(\"{{ git_command }}\", options).exec()\n}\n"
    ),
    (
        TEMPLATE_GIT_BASE_COMMAND,
        "fn git_command<'a, I>(cmd: &str, options: I) -> CommandExecutor\nwhere\n    I: IntoIterator<Item=CommandOption<'a>>\n{\n    command(\"{{ git_cli_command }}\", cmd, options)\n}\n"
    )
];

pub fn command_templates() -> Engine<'static> {
    let mut engine = Engine::new();
    
    for (name, source) in GIT_TEMPLATES_COMMON {
        let _ = engine.add_template(*name, *source);
    }
    
    engine
}

