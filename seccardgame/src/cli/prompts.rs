use dialoguer::Input;

pub fn prompt<T: std::str::FromStr + Default>(
    prompt_msg: &str,
    validator: Option<Box<dyn Fn(&T) -> Result<(), String>>>,
) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug + ToString,
    T: Clone + ToString,
{
    let mut input = Input::<T>::new().with_prompt(prompt_msg);
    if let Some(validator) = validator {
        input = input.validate_with(validator);
    }
    input.interact_text().unwrap()
}

pub fn prompt_allow_empty(prompt_msg: &str) -> String {
    let default = "";

    Input::<String>::new()
        .with_prompt(prompt_msg)
        .allow_empty(true)
        .default(default.into())
        .show_default(false)
        .interact_text()
        .unwrap()
}
