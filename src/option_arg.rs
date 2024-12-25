use std::sync::Arc;
use crate::wrap_command::FnOptionArg;


pub fn simple(git_option: &str) -> FnOptionArg {
    let l_git_option = String::from(git_option);
    FnOptionArg(Arc::new(move || vec![String::from(l_git_option.as_str())]))
}

pub fn equal_no_optional(git_option: &str, option_argument: &str) -> FnOptionArg {
    let l_option_argument = format!("{git_option}={option_argument}");
    FnOptionArg(Arc::new(move || vec![String::from(l_option_argument.as_str())]))
}

pub fn equal_optional(git_option: &str, option_argument: &str) -> FnOptionArg {
    let l_option_argument = if option_argument.is_empty() {
        String::from(git_option)
    } else {
        format!("{git_option}={option_argument}")
    };
    FnOptionArg(Arc::new(move || vec![String::from(l_option_argument.as_str())]))
}

pub fn with_parameter(git_option: &str, arg1: &str) -> FnOptionArg {
    let l_git_option = git_option.to_string();
    let l_arg1 = arg1.to_string();
    FnOptionArg(Arc::new(move || vec![
        String::from(l_git_option.as_str()),
        String::from(l_arg1.as_str())
    ]))
}

pub fn with_second_parameter(git_option: &str, arg1: &str, arg2: &str) -> FnOptionArg {
    let l_git_option = git_option.to_string();
    let l_arg1 = arg1.to_string();
    let l_arg2 = arg2.to_string();
    FnOptionArg(Arc::new(move || vec![
        String::from(l_git_option.as_str()),
        String::from(l_arg1.as_str()),
        String::from(l_arg2.as_str())
    ]
    ))
}

pub fn with_optional_third_parameter(git_option: &str, arg1: &str, arg2: &str, arg3: &str) -> FnOptionArg {
    let l_git_option = git_option.to_string();
    let l_arg1 = arg1.to_string();
    let l_arg2 = arg2.to_string();
    let l_arg3 = if !arg3.is_empty() {
        Some(String::from(arg3))
    } else {
        None
    };
    if let Some(arg) = l_arg3 {
        FnOptionArg(Arc::new(move || vec![
            String::from(l_git_option.as_str()),
            String::from(l_arg1.as_str()),
            String::from(l_arg2.as_str()),
            String::from(arg.as_str()),
        ]))
    } else {
        FnOptionArg(Arc::new(move || vec![
            String::from(l_git_option.as_str()),
            String::from(l_arg1.as_str()),
            String::from(l_arg2.as_str())
        ]))
    }
}

pub fn with_optional_second_parameter(git_option: &str, arg1: &str, arg2: &str) -> FnOptionArg {
    let l_git_option = git_option.to_string();
    let l_arg1 = arg1.to_string();
    let l_arg2 = if !arg2.is_empty() {
        Some(String::from(arg2))
    } else {
        None
    };

    if let Some(arg) = l_arg2 {
        FnOptionArg(Arc::new(move || vec![
            String::from(l_git_option.as_str()),
            String::from(l_arg1.as_str()),
            String::from(arg.as_str())
        ]))
    } else {
        FnOptionArg(Arc::new(move || vec![
            String::from(l_git_option.as_str()),
            String::from(l_arg1.as_str())
        ]))
    }
}

pub fn with_optional_parameter(git_option: &str, arg1: &str) -> FnOptionArg {
    let l_git_option = git_option.to_string();
    let l_arg1 = if !arg1.is_empty() {
        Some(String::from(arg1))
    } else {
        None
    };
    if let Some(arg) = l_arg1 {
        FnOptionArg(Arc::new(move || vec![
            String::from(l_git_option.as_str()),
            String::from(arg.as_str())
        ]))
    } else {
        FnOptionArg(Arc::new(move || vec![ String::from(l_git_option.as_str())]))
    }
}

pub fn value_parameter(value: &str) -> FnOptionArg {
    let l_value = String::from(value);
    FnOptionArg(Arc::new(move || vec![String::from(l_value.as_str())]))
}

pub fn double_value_parameter(value1: &str, value2: &str) -> FnOptionArg {
    let l_value1 = String::from(value1);
    let l_value2 = String::from(value2);
    FnOptionArg(Arc::new(move || vec![
        String::from(l_value1.as_str()),
        String::from(l_value2.as_str())
    ]))
}
