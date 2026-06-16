use validator::ValidationError;

/// Removes formatting and allows only alphanumeric characters, returning an uppercase String.
pub fn sanitize_input(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_uppercase()
}

pub fn validate_cpf_cnpj(input: &str) -> Result<(), ValidationError> {
    // Clean the input and remove formatting, allowing only alphanumeric characters.
    let sanitized_input = sanitize_input(input);

    // Check if the sequence consists of the same character (e.g., '00...0', 'AA...A').
    if let Some(first_char) = sanitized_input.chars().next() {
        if sanitized_input.chars().all(|c| c == first_char) {
            let mut err = ValidationError::new("invalid_sequence");
            err.message = Some("Sequence cannot consist of the same character".into());

            return Err(err);
        }
    }

    // Check if CPF length is 11 and all chars are numbers, or if CNPJ length is 14 and the last 2 chars are numbers (we already confirmed that the rest of the sequence is alphanumeric).
    let length = sanitized_input.len();

    let is_valid = if length == 11 && sanitized_input.chars().all(|c| c.is_ascii_digit()) {
        validate_cpf(&sanitized_input)
    } else if length == 14 && sanitized_input.chars().skip(12).all(|c| c.is_ascii_digit()) {
        validate_cnpj(&sanitized_input)
    } else {
        false
    };

    if is_valid {
        return Ok(())
    } else {
        let (code, msg) = match length {
            11 => ("invalid_cpf", "CPF number is not valid"),
            14 => ("invalid_cnpj", "CNPJ number is not valid"),
            _ => ("invalid_format", "Sequence must be a valid CPF or CNPJ format"),
        };
        
        let mut err = ValidationError::new(code);
        err.message = Some(msg.into());

        return Err(err);
    }
}

fn validate_cpf(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

    let sum1: u32 = digits.iter().take(9).enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let check1 = (sum1 * 10) % 11 % 10;

    let sum2: u32 = digits.iter().take(10).enumerate().map(|(i, &d)| d * (11 - i as u32)).sum();
    let check2 = (sum2 * 10) % 11 % 10;

    return check1 == digits[9] && check2 == digits[10];
}

fn validate_cnpj(cnpj: &str) -> bool {
    // Convert chars to their numeric values (Unicode code - 48)
    let values: Vec<u32> = cnpj.chars().map(|c| c as u32 - 48).collect();

    let sum1: u32 = values
        .iter()
        .take(12)
        .enumerate()
        .map(|(i, &v)| v * if i < 4 { 5 - i as u32 } else { 13 - i as u32 })
        .sum();
    let remainder1 = sum1 % 11;
    let check1 = if remainder1 < 2 { 0 } else { 11 - remainder1 };

    let sum2: u32 = values
        .iter()
        .take(12)
        .chain(std::iter::once(&check1))
        .enumerate()
        .map(|(i, &v)| v * if i < 5 { 6 - i as u32 } else { 14 - i as u32 })
        .sum();
    let remainder2 = sum2 % 11;
    let check2 = if remainder2 < 2 { 0 } else { 11 - remainder2 };

    return check1 == values[12] && check2 == values[13];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cpf_cnpj() {
        assert!(validate_cpf_cnpj("314.733.270-59").is_ok());
        assert!(validate_cpf_cnpj("314.733.270-60").is_err());
        assert!(validate_cpf_cnpj("80.865.683/0001-89").is_ok());
        assert!(validate_cpf_cnpj("80.865.683/0001-90").is_err());
        assert!(validate_cpf_cnpj("C3.P45.166/A8H9-14").is_ok());
        assert!(validate_cpf_cnpj("C3.P45.166/A8H9-15").is_err());
        assert!(validate_cpf_cnpj("invalid").is_err());
    }
}
