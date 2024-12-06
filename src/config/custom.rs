use std::process::Command;
use crate::wrap_command::FnOptionArg;

/// Adds a configuration entry
pub fn entry(key: &str, value: &str) -> FnOptionArg {
    let l_key = String::from(key);
    let l_value = String::from(value);
    Box::new(move |c: &mut Command| {
        c.arg(l_key.as_str());
        c.arg(l_value.as_str());
    })
}

/// Adds a new line to the option without altering any existing values. This is the same as providing ^$ as the value_regex in --replace-all.
/// --add
pub fn add(name: &str, value: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value = String::from(value);
    Box::new(move |c: &mut Command| {
        c.arg("--add");
        c.arg(l_name.as_str());
        c.arg(l_value.as_str());
    })
}

/// Default behavior is to replace at most one line. This replaces all lines matching the key (and optionally the value_regex).
/// --replace-all
pub fn replace_all(name:&str, value: &str, value_regex: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value = String::from(value);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--replace-all");
        c.arg(l_name.as_str());
        c.arg(l_value.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

/// Get the value for a given key (optionally filtered by a regex matching the value). Returns error code 1 if the key was not found and the last value if multiple key values were found.
/// --get
pub fn get(name: &str, value_regex: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--get");
        c.arg(l_name.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

/// Like get, but returns all values for a multi-valued key.
/// get-all
pub fn get_all(name: &str, value_regex: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--get-all");
        c.arg(l_name.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

// Like --get-all, but interprets the name as a regular expression and writes out the key names. Regular expression matching is currently case-sensitive and done against a canonicalized version of the key in which section and variable names are lowercased, but subsection names are not.
// --get-regexp
pub fn get_regexp(name_regex: &str, value_regex: &str) -> FnOptionArg {
    let l_name_regex = String::from(name_regex);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--get-regexp");
        c.arg(l_name_regex.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

/// When given a two-part name section.key, the value for section.<url>.key whose <url> part matches the best to the given URL is returned (if no such key exists, the value for section.key is used as a fallback). When given just the section as name, do so for all the keys in the section and list them. Returns error code 1 if no value is found.
/// --get-urlmatch name URL
pub fn get_urlmatch(name: &str, url: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_url = String::from(url);
    Box::new(move |c: &mut Command| {
        c.arg("--get-urlmatch");
        c.arg(l_name.as_str());
        c.arg(l_url.as_str());
    })
}

/// Remove the line matching the key from config file.
/// --unset
pub fn unset(name: &str, value_regex: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--unset");
        c.arg(l_name.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

/// Remove all lines matching the key from config file.
/// --unset-all
pub fn unset_all(name: &str, value_regex: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_value_regex = String::from(value_regex);
    Box::new(move |c: &mut Command| {
        c.arg("--unset-all");
        c.arg(l_name.as_str());
        if !l_value_regex.is_empty() {
            c.arg(l_value_regex.as_str());
        }
    })
}

/// Rename the given section to a new name.
/// --rename-section
pub fn rename_section(old_name: &str, new_name: &str) -> FnOptionArg {
    let l_old_name = String::from(old_name);
    let l_new_name = String::from(new_name);
    Box::new(move |c: &mut Command| {
        c.arg("--rename-section");
        c.arg(l_old_name.as_str());
        c.arg(l_new_name.as_str());
    })
}

/// Remove the given section from the configuration file.
/// --remove-section
pub fn remove_section(name: &str) -> FnOptionArg {
    let l_name = String::from(name);
    Box::new(move |c: &mut Command| {
        c.arg("--remove-section");
        c.arg(l_name.as_str());
    })
}

/// Find the color configured for name (e.g.  color.diff.new) and output it as the ANSI color escape sequence to the standard output. The optional default parameter is used instead, if there is no color configured for name.
/// --get-color name [default]
pub fn get_color(name: &str, default_value: &str) -> FnOptionArg {
    let l_name = String::from(name);
    let l_default_value = String::from(default_value);
    Box::new(move |c: &mut Command| {
        c.arg("--get-color");
        c.arg(l_name.as_str());
        if !l_default_value.is_empty() {
            c.arg(l_default_value.as_str());
        }
    })
}

/// Find the color setting for name (e.g.  color.diff) and output "true" or "false".  stdout-is-tty should be either "true" or "false", and is taken into account when configuration says "auto". If stdout-is-tty is missing, then checks the standard output of the command itself, and exits with status 0 if color is to be used, or exits with status 1 otherwise. When the color setting for name is undefined, the command uses color.ui as fallback.
/// --get-colorbool name [stdout-is-tty]
pub fn get_colorbool(name: &str, stdout_is_tty: bool) -> FnOptionArg {
    let l_name = String::from(name);
    Box::new(move |c: &mut Command| {
        c.arg("--get-colorbool");
        c.arg(l_name.as_str());
        c.arg(&stdout_is_tty.to_string());
    })
}
