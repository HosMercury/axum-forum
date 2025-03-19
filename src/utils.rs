use validator::ValidationErrors;

pub fn validation_errors(errs: ValidationErrors) -> Vec<String> {
    errs.field_errors()
        .iter()
        .flat_map(|(_, errors)| {
            errors.iter().map(move |error| {
                format!(
                    "{}",
                    error.message.clone().unwrap_or_else(|| "Invalid".into())
                )
            })
        })
        .collect::<Vec<String>>()
}
