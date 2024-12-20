mod options;
mod template;

use crate::options::{normalize, CmdOptionKind};
use crate::template::{
    command_templates, TEMPLATE_GIT_COMMAND_FILE, TEMPLATE_GIT_COMMAND_MACRO, TEMPLATE_MOD_RS,
    TEMPLATE_OPTION_DOC_COMMENTS, TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
    TEMPLATE_OPTION_EQUAL_OPTIONAL, TEMPLATE_OPTION_NAME_CONSTANT, TEMPLATE_OPTION_SIMPLE,
    TEMPLATE_OPTION_VALUE_PARAMETER, TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
    TEMPLATE_OPTION_WITH_PARAMETER,
};
use serde_json::{from_str, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};
use upon::Engine;

const PHG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION_FILENAME: &str = "description.json";
const MOD_RS_FILENAME: &str = "mod.rs";
const GIT_COMMAND_FILENAME: &str = "git_command.rs";

pub fn main() {
    println!("{PKG_NAME} v{PHG_VERSION}");
    let args: Vec<String> = env::args().collect();
    let desc_file = if args.len() <= 1 {
        DESCRIPTION_FILENAME
    } else {
        &args[1]
    };
    println!("reading git commands description file: {desc_file}");
    let json = read_descriptions(desc_file);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let output_dir = format!("output_{:?}", now.as_millis());
    let engine = command_templates();
    fs::create_dir_all(&output_dir).expect("could not create output dir");
    let git_command_file = format!("{output_dir}/{GIT_COMMAND_FILENAME}");
    git_command_file_create(&engine, &git_command_file);
    for desc in json.as_array().unwrap() {
        let enabled = desc.get("enabled").unwrap().as_bool().unwrap();
        if enabled {
            let command_name = desc.get("command_name").unwrap().as_str().unwrap();
            let options = desc.get("options").unwrap().as_array().unwrap();
            let description = desc.get("description").unwrap().as_str().unwrap();
            let doc_url = desc.get("doc-url").unwrap().as_str().unwrap();
            command_generator(
                output_dir.as_str(),
                &engine,
                command_name,
                options,
                description,
                doc_url,
            );
            git_command_file_append_command(&engine, command_name, &git_command_file);
        }
    }
}

fn command_generator(
    output_dir: &str,
    engine: &Engine,
    command_name: &str,
    options: &Vec<Value>,
    description: &str,
    doc_url: &str,
) {
    let normalized_command_name = normalize(command_name);
    let command_path = format!("{output_dir}/{normalized_command_name}");
    fs::create_dir_all(command_path.as_str()).expect("could not create dir");
    command_mod_file_generator(
        engine,
        command_name,
        format!("{command_path}/{MOD_RS_FILENAME}").as_str(),
        description,
        doc_url,
    );
    command_options_file_generator(
        engine,
        options,
        format!("{command_path}/options.rs").as_str(),
    );
    println!("command {command_name} generated");
}

fn command_mod_file_generator(
    engine: &Engine,
    cmd: &str,
    mod_file_path: &str,
    description: &str,
    doc_url: &str,
) {
    let tpl = engine.template(TEMPLATE_MOD_RS);
    let descriptions: Vec<&str> = description.lines().collect();
    let mod_rs_content = tpl
        .render(upon::value!{command_name: normalize(cmd), git_command: cmd, descriptions: descriptions, doc_url: doc_url})
        .to_string()
        .expect("could not render template mod_rs");

    fs::write(mod_file_path, mod_rs_content.as_str()).expect("Unable to write file module file");
}

fn git_command_file_create(engine: &Engine, file_path: &str) {
    let tpl = engine.template(TEMPLATE_GIT_COMMAND_FILE);
    let git_command_content = tpl
        .render(upon::value! {})
        .to_string()
        .expect("could not render template git_command_file");

    fs::write(file_path, &git_command_content).expect("Unable to write git command file");
}

fn git_command_file_append_command(engine: &Engine, cmd: &str, file_path: &str) {
    let tpl = engine.template(TEMPLATE_GIT_COMMAND_MACRO);
    let git_macro_content = tpl
        .render(upon::value! {command_name: normalize(cmd), git_command: cmd})
        .to_string()
        .expect("could not render template git_command_macro");

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("cannot open git_command file");

    data_file
        .write_fmt(format_args!("\n{git_macro_content}\n"))
        .unwrap();
}

fn command_options_file_generator(engine: &Engine, options: &Vec<Value>, options_file_path: &str) {
    let mut options_content: Vec<String> = vec![
        String::from("// Warning!! Code generated automatically: this file must not be edited by hand"),
        String::from("use crate::optionarg;"),
        String::from("use crate::wrap_command::FnOptionArg;"),
        String::from("")
    ];

    let mut functions: Vec<String> = Vec::new();
    let mut constants: Vec<String> = Vec::new();

    for opt in options {
        let (function_desc, constant, function) = option_render_match(engine, opt);
        if let Some(f_def) = function {
            if let Some(f_desc) = function_desc {
                functions.push(format!("{}\n{}\n", f_desc, f_def));
            }
            if let Some(c_def) = constant {
                constants.push(c_def);
            }
        }
    }

    options_content.append(&mut constants);
    options_content.push(String::from(""));
    options_content.append(&mut functions);

    fs::write(options_file_path, options_content.join("\n"))
        .expect("Unable to write command options file");
}

fn option_render_match(engine: &Engine, opt: &Value) -> (Option<String>, Option<String>, Option<String>) {
    let argument = opt.get("argument").unwrap().as_str().unwrap();
    let arguments = opt.get("arguments").unwrap().as_str().unwrap();
    let method_name = opt.get("method_name");
    let descriptions: Vec<&str> = opt
        .get("description")
        .unwrap()
        .as_str()
        .unwrap()
        .lines()
        .collect();

    let function_desc = render(engine, TEMPLATE_OPTION_DOC_COMMENTS, upon::value! {descriptions: descriptions, arguments: arguments});

    let (constant_value, template, option_value) = match options::option_kind(argument) {
        CmdOptionKind::Simple(git_option, option_name) =>
            build_option(method_name, Some(TEMPLATE_OPTION_SIMPLE), option_name.as_str(), git_option.as_str(), None),

        CmdOptionKind::EqualNoOptional(git_option, option_name, argument) |
        CmdOptionKind::EqualOptionalWithName(git_option, option_name, argument) =>
            build_option(method_name, Some(TEMPLATE_OPTION_EQUAL_NO_OPTIONAL), option_name.as_str(), git_option.as_str(), Some(argument.as_str())),

        CmdOptionKind::EqualOptionalWithoutName(git_option, option_name) |
        CmdOptionKind::EqualWithoutName(git_option, option_name) =>
            build_option(method_name, Some(TEMPLATE_OPTION_EQUAL_OPTIONAL), option_name.as_str(), git_option.as_str(), Some("value")),

        CmdOptionKind::WithParameter(git_option, option_name, argument) =>
            build_option(method_name, Some(TEMPLATE_OPTION_WITH_PARAMETER), option_name.as_str(), git_option.as_str(), Some(argument.as_str())),

        CmdOptionKind::WithOptionalParameter(git_option, option_name, argument) =>
            build_option(method_name, Some(TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER), option_name.as_str(), git_option.as_str(), Some(argument.as_str())),

        CmdOptionKind::ValueParameter(value_parameter) =>
            build_value_parameter(method_name, TEMPLATE_OPTION_VALUE_PARAMETER, value_parameter.as_str()),

        CmdOptionKind::None =>
            (None, None, None)
    };

    if let Some(tpl) = template {
        option_render(engine, Some(function_desc), constant_value, tpl.as_str(), option_value.unwrap())
    } else {
        (None, None, None)
    }
}

fn build_option(method_name: Option<&Value>, template: Option<&str>, option_name: &str, git_option: &str, option_argument: Option<&str>) -> (Option<upon::Value>, Option<String>, Option<upon::Value>) {
    let name = option_method_name(method_name, option_name);
    let constant_name = name.to_uppercase();
    let constant_value = upon::value! {constant_name: String::from(constant_name.as_str()), git_option: String::from(git_option)};
    if let Some(tpl) = template {
        if let Some(argument) = option_argument {
            (Some(constant_value), Some(String::from(tpl)), Some(upon::value! {method_name: name, constant_name: String::from(constant_name.as_str()), option_argument: String::from(argument)}))
        } else {
            (Some(constant_value), Some(String::from(tpl)), Some(upon::value! {method_name: name, constant_name: String::from(constant_name.as_str())}))
        }
    } else {
        (Some(constant_value), None, None)
    }
}

fn build_value_parameter(method_name: Option<&Value>, template: &str, value_parameter: &str) -> (Option<upon::Value>, Option<String>, Option<upon::Value>) {
    let name = option_method_name(method_name, value_parameter);
    (None, Some(String::from(template)), Some(upon::value! {method_name: name, value_parameter: value_parameter}))
}

fn option_render(engine: &Engine, function_desc: Option<String>, constant_value: Option<upon::Value>, template: &str, option_value: upon::Value) -> (Option<String>, Option<String>, Option<String>) {
    if let Some(cv) = constant_value {
        (
            function_desc,
            Some(render(engine, TEMPLATE_OPTION_NAME_CONSTANT, cv)),
            Some(render(engine, template, option_value)),
        )
    } else {
        (
            function_desc,
            None,
            Some(render(engine, template, option_value)),
        )
    }
}

fn option_method_name(method_name: Option<&Value>, option_name: &str) -> String {
    normalize(match method_name {
        None => option_name,
        Some(v) => v.as_str().unwrap(),
    })
}

fn render(engine: &Engine, template_name: &str, template_values: upon::Value) -> String {
    engine
        .template(template_name)
        .render(template_values)
        .to_string()
        .expect("could not render template")
}

fn read_descriptions(file_name: &str) -> Value {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    from_str(contents.as_str()).expect("file should be proper JSON")
}
