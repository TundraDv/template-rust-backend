use crate::utils::error::AppError;
use validator::Validate;

pub fn validate_request<T: Validate>(payload: T) -> Result<T, AppError> {
    payload.validate().map_err(|errors| {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    format!(
                        "{}: {}",
                        field,
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("{:?}", e.code))
                    )
                })
            })
            .collect();

        AppError::Validation(error_messages.join(", "))
    })?;

    Ok(payload)
}
