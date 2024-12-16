mod template;
mod options;

use crate::options::{normalize, CmdOptionKind};
use crate::template::{command_templates, TEMPLATE_GIT_COMMAND_FILE, TEMPLATE_GIT_COMMAND_MACRO, TEMPLATE_MOD_RS, TEMPLATE_OPTION_DOC_COMMENTS, TEMPLATE_OPTION_EQUAL_NO_OPTIONAL, TEMPLATE_OPTION_EQUAL_OPTIONAL, TEMPLATE_OPTION_SIMPLE, TEMPLATE_OPTION_VALUE_PARAMETER, TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER, TEMPLATE_OPTION_WITH_PARAMETER};
use serde_json::{from_str, Value};
use std::string::String;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
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
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
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
            command_generator(output_dir.as_str(), &engine, command_name, options, description, doc_url);
            git_command_file_append_command(&engine, command_name, &git_command_file);
        }
    }
}

fn command_generator(output_dir: &str, engine: &Engine, command_name: &str, options: &Vec<Value>, description: &str, doc_url: &str) {
    let normalized_command_name = normalize(command_name);
    let command_path = format!("{output_dir}/{normalized_command_name}");
    fs::create_dir_all(command_path.as_str()).expect("could not create dir");
    command_mod_file_generator(engine, command_name, format!("{command_path}/{MOD_RS_FILENAME}").as_str(), description, doc_url);
    command_options_file_generator(engine, options, format!("{command_path}/options.rs").as_str());
    println!("command {command_name} generated");
}

fn command_mod_file_generator(engine: &Engine, cmd: &str, mod_file_path: &str, description: &str, doc_url: &str) {
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
        .render(upon::value!{})
        .to_string()
        .expect("could not render template git_command_file");

    fs::write(file_path, &git_command_content).expect("Unable to write git command file");
}

fn git_command_file_append_command(engine: &Engine, cmd: &str, file_path: &str) {
    let tpl = engine.template(TEMPLATE_GIT_COMMAND_MACRO);
    let git_macro_content = tpl
        .render(upon::value!{command_name: normalize(cmd), git_command: cmd})
        .to_string()
        .expect("could not render template git_command_macro");

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("cannot open git_command file");

    data_file.write_fmt(format_args!("\n{git_macro_content}\n")).unwrap();
}

fn command_options_file_generator(engine: &Engine, options: &Vec<Value>, options_file_path: &str) {
    let mut options_content: Vec<String> = Vec::new();
    options_content.push(String::from("// Warning!! Code generated automatically: this file must not be edited by hand"));
    options_content.push(String::from("use crate::optionarg;"));
    options_content.push(String::from("use crate::wrap_command::FnOptionArg;"));

    for opt in options {
        let argument = opt.get("argument").unwrap().as_str().unwrap();
        let arguments = opt.get("arguments").unwrap().as_str().unwrap();
        let method_name = opt.get("method_name");
        let descriptions: Vec<&str> = opt.get("description").unwrap().as_str().unwrap().lines().collect();

        let function_desc = engine.template(TEMPLATE_OPTION_DOC_COMMENTS)
            .render(upon::value!{descriptions: descriptions, arguments: arguments})
            .to_string()
            .expect("could not render template simple_option");

        let function = match options::option_kind(argument) {
            CmdOptionKind::Simple(git_option, option_name) =>
                render(
                    engine,
                    TEMPLATE_OPTION_SIMPLE,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option},
                ),

            CmdOptionKind::EqualNoOptional(git_option, option_name, argument) =>
                render(
                    engine,
                    TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: argument},
                ),

            CmdOptionKind::EqualOptionalWithoutName(git_option, option_name) =>
                render(
                    engine,
                    TEMPLATE_OPTION_EQUAL_OPTIONAL,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: String::from("value")},
                ),

            CmdOptionKind::EqualWithoutName(git_option, option_name) =>
                render(
                    engine,
                    TEMPLATE_OPTION_EQUAL_OPTIONAL,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: String::from("value")},
                ),

            CmdOptionKind::EqualOptionalWithName(git_option, option_name, argument) =>
                render(
                    engine,
                    TEMPLATE_OPTION_EQUAL_NO_OPTIONAL,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: argument},
                ),

            CmdOptionKind::WithParameter(git_option, option_name, argument) =>
                render(
                    engine,
                    TEMPLATE_OPTION_WITH_PARAMETER,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: argument},
                ),

            CmdOptionKind::WithOptionalParameter(git_option, option_name, argument) =>
                render(
                    engine,
                    TEMPLATE_OPTION_WITH_OPTIONAL_PARAMETER,
                    upon::value!{method_name: option_method_name(method_name, &option_name), git_option: git_option, option_argument: argument},
                ),

            CmdOptionKind::ValueParameter(value_parameter) => 
                render(
                    engine,
                    TEMPLATE_OPTION_VALUE_PARAMETER,
                    upon::value!{method_name: option_method_name(method_name, &value_parameter), value_parameter: value_parameter},
                ),
            
            CmdOptionKind::None => String::from("")
        };

        options_content.push(format!("{}\n{}\n", function_desc, function));
    }

    fs::write(options_file_path, options_content.join("\n")).expect("Unable to write command options file");
}

fn option_method_name(method_name: Option<&Value>, option_name: &String) -> String {
    normalize(match method_name {
        None => option_name.as_str(),
        Some(v) => v.as_str().unwrap()
    })
}

fn render(engine: &Engine, template_name: &str, template_values: upon::Value) -> String {
    engine.template(template_name)
        .render(template_values)
        .to_string()
        .expect("could not render template")
}

fn read_descriptions(file_name: &str) -> Value {
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");
    from_str(contents.as_str())
        .expect("file should be proper JSON")
}
