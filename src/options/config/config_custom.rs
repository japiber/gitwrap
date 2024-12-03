use crate::{CommandExecutor, CommandOption};

/// Adds a configuration entry
pub fn entry(key: &'static str, value: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option(key);
        g.add_option(value);
    })
}

/// Adds a new line to the option without altering any existing values. This is the same as providing ^$ as the value_regex in --replace-all.
/// --add
pub fn add(name: &'static str, value: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--add");
        g.add_option(name);
        g.add_option(value);
    })
}

/// Default behavior is to replace at most one line. This replaces all lines matching the key (and optionally the value_regex).
/// --replace-all
pub fn replace_all(name: &'static str, value: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--replace-all");
        g.add_option(name);
        g.add_option(value);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

/// Get the value for a given key (optionally filtered by a regex matching the value). Returns error code 1 if the key was not found and the last value if multiple key values were found.
/// --get
pub fn get(name: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get");
        g.add_option(name);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

/// Like get, but returns all values for a multi-valued key.
/// get-all
pub fn get_all(name: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get-all");
        g.add_option(name);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

// Like --get-all, but interprets the name as a regular expression and writes out the key names. Regular expression matching is currently case-sensitive and done against a canonicalized version of the key in which section and variable names are lowercased, but subsection names are not.
// --get-regexp
pub fn get_regexp(name_regex: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get-regexp");
        g.add_option(name_regex);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

/// When given a two-part name section.key, the value for section.<url>.key whose <url> part matches the best to the given URL is returned (if no such key exists, the value for section.key is used as a fallback). When given just the section as name, do so for all the keys in the section and list them. Returns error code 1 if no value is found.
/// --get-urlmatch name URL
pub fn get_urlmatch(name: &'static str, url: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get-urlmatch");
        g.add_option(name);
        g.add_option(url);
    })
}

/// Remove the line matching the key from config file.
/// --unset
pub fn unset(name: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--unset");
        g.add_option(name);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

/// Remove all lines matching the key from config file.
/// --unset-all
pub fn unset_all(name: &'static str, value_regex: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--unset-all");
        g.add_option(name);
        if !value_regex.is_empty() {
            g.add_option(value_regex)
        }
    })
}

/// Rename the given section to a new name.
/// --rename-section
pub fn rename_section(old_name: &'static str, new_name: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--rename-section");
        g.add_option(old_name);
        g.add_option(new_name);
    })
}

/// Remove the given section from the configuration file.
/// --remove-section
pub fn remove_section(name: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--remove-section");
        g.add_option(name);
    })
}

/// Find the color configured for name (e.g.  color.diff.new) and output it as the ANSI color escape sequence to the standard output. The optional default parameter is used instead, if there is no color configured for name.
/// --get-color name [default]
pub fn get_color(name: &'static str, default_value: &'static str) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get-color");
        g.add_option(name);
        if !default_value.is_empty() {
            g.add_option(default_value);
        }
    })
}

/// Find the color setting for name (e.g.  color.diff) and output "true" or "false".  stdout-is-tty should be either "true" or "false", and is taken into account when configuration says "auto". If stdout-is-tty is missing, then checks the standard output of the command itself, and exits with status 0 if color is to be used, or exits with status 1 otherwise. When the color setting for name is undefined, the command uses color.ui as fallback.
/// --get-colorbool name [stdout-is-tty]
pub fn get_colorbool(name: &'static str, stdout_is_tty: bool) -> CommandOption<'static> {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--get-colorbool");
        g.add_option(name);
        g.add_option(&stdout_is_tty.to_string())
    })
}
