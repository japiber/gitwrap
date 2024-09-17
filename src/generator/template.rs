use upon::Engine;


pub const TEMPLATE_MOD_RS: &'static str = "mod_rs";
pub const TEMPLATE_OPTION_SIMPLE: &'static str = "simple";
pub const TEMPLATE_OPTION_DOC_COMMENTS: &'static str = "doc_comments";
pub const TEMPLATE_OPTION_EQUAL_NO_OPTIONAL: &'static str = "equal_no_optional";
pub const TEMPLATE_OPTION_EQUAL_OPTIONAL: &'static str = "equal_optional";
pub const TEMPLATE_OPTION_WITH_PARAMETER: &'static str = "with_parameter";
pub const TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER: &'static str = "with_optional_parameter";
pub const TEMPLATE_COMMAND : &'static str = "command";
pub const TEMPLATE_GIT_BASE_COMMAND: &'static str = "git_base_command";

pub fn command_templates() -> Engine<'static> {
    let mut engine = Engine::new();

    let _ = engine.add_template(
        TEMPLATE_MOD_RS,
        "mod {{ command_name }}_options;\npub use {{ command_name }}_options::*;\n"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_SIMPLE,
        "pub fn {{ option_name }}_option() -> CommandOption<'static> {\n    Box::new(|g: &mut CommandExecutor| g.add_option(\"{{ git_option }}\"))\n}"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_DOC_COMMENTS,
        "{% for doc in descriptions %}/// {{ doc }}\n{% endfor %}/// {{ arguments }}"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
        "pub fn {{ option_name }}_option({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}={}\", {{ option_argument }})))\n}"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_EQUAL_OPTIONAL,
        "pub fn {{ option_name }}_option({{ option_argument }} :&str) -> CommandOption {\n    if {{ option_argument }}.len() == 0 {\n        Box::new(|g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}\")))\n    } else {\n        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!(\"{{ git_option }}={}\", {{ option_argument }})))\n    }\n}\n"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_WITH_PARAMETER,
        "pub fn {{ option_name }}_option({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| {\n        g.add_option(\"{{ git_option }}\");\n        g.add_option_string(format!(\"{}\", {{ option_argument }} ));\n    })\n}\n"
    );
    let _ = engine.add_template(
        TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
        "pub fn {{ option_name }}_option({{ option_argument }} :&str) -> CommandOption {\n    Box::new(move |g: &mut CommandExecutor| {\n         g.add_option(\"{{ git_option }}\");\n        if {{ option_argument }}.len() > 0 {\n            g.add_option_string(format!(\"{}\", {{ option_argument }}));\n        }\n    })\n}\n"
    );
    let _ = engine.add_template(
        TEMPLATE_COMMAND,
        "pub fn {{ command_name }}<'a, I>(options: I) -> ExecResult\nwhere\n    I: IntoIterator<Item=CommandOption<'a>>\n{\n    git_command(\"{{ git_command }}\", options).exec()\n}\n");
    let _ = engine.add_template(
        TEMPLATE_GIT_BASE_COMMAND,
        "fn git_command<'a, I>(cmd: &str, options: I) -> CommandExecutor\nwhere\n    I: IntoIterator<Item=CommandOption<'a>>\n{\n    command(\"{{ git_cli_command }}\", cmd, options)\n}\n"
    );
    engine
}

