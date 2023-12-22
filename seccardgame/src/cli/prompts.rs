use dialoguer::Input;

pub fn prompt<T: std::str::FromStr + Default>(
    prompt_msg: &str,
    validator: Option<Box<dyn Fn(&T) -> Result<(), String>>>,
) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug + ToString, T: Clone + ToString
{
    let mut input = Input::<T>::new().with_prompt(prompt_msg);
    if let Some(validator) = validator {
        input = input.validate_with(validator);
    }
    input.interact_text().unwrap()
}
