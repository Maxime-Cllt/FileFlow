use csv::StringRecord;

pub struct StringFormatter;

impl StringFormatter {
    /// This function is used to generate the column names for a CSV file.
    pub fn get_formated_column_names(headers: &[String]) -> Vec<String> {
        const COLUMN_PREFIX: &str = "column_";
        let mut safe_headers: Vec<String> = Vec::with_capacity(headers.len());

        for (index, item) in headers.iter().enumerate() {
            let trimmed_column_name: &str = item.trim();
            if trimmed_column_name.is_empty() {
                safe_headers.push(format!("{COLUMN_PREFIX}{}", index + 1));
            } else {
                let sanitized_column_name: String = Self::sanitize_column(trimmed_column_name);
                safe_headers.push(if sanitized_column_name.is_empty() {
                    format!("{COLUMN_PREFIX}{}", index + 1)
                } else {
                    sanitized_column_name
                });
            }
        }
        safe_headers
    }

    /// Sanitize a value for safe insertion into the database
    #[inline]
    pub fn sanitize_value(value: &str) -> String {
        let mut sanitized: String = String::with_capacity(value.len());

        for c in value.trim().chars() {
            match c {
                '\'' => sanitized.push_str("''"),   // Escape single quotes
                '\\' => sanitized.push_str("\\\\"), // Escape backslashes
                '\"' | '\0' => {}                   // Remove double quotes and null characters
                '\r' | '\n' => sanitized.push(' '), // Normalize newlines
                _ => sanitized.push(c),
            }
        }

        sanitized
    }

    /// Sanitize a column name for safe insertion into the database
    #[inline]
    pub fn sanitize_column(value: &str) -> String {
        let trimmed: &str = value.trim();
        let mut result: String = String::with_capacity(trimmed.len());

        for char in trimmed.chars() {
            // Filter out unwanted characters first (early return)
            if (char.is_control() && char != '\n' && char != '\t')
                || char == '\u{feff}'  // BOM
                || char == '\u{200b}'  // Zero-width space
                || char == '\u{200c}'  // Zero-width non-joiner
                || char == '\u{200d}'  // Zero-width joiner
                || !char.is_ascii()
            // Filter out non-ASCII characters
            {
                continue;
            }

            match char {
                '\'' | '\\' | '\"' | '.' | '/' => continue, // Skip
                ' ' => result.push('_'),
                _ => {
                    if char.is_ascii() {
                        result.push(char.to_ascii_lowercase());
                    } else {
                        continue;
                    }
                }
            }
        }

        result
    }

    /// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
    #[inline]
    pub fn escape_record(values: StringRecord) -> String {
        if values.is_empty() {
            return String::new();
        }

        // Estimate capacity: each value + 2 quotes + 2 chars for ", "
        let estimated_capacity: usize = values.iter().map(|v| v.len() + 4).sum::<usize>();

        let mut result: String = String::with_capacity(estimated_capacity);

        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push('\'');
            result.push_str(&Self::sanitize_value(value));
            result.push('\'');
        }

        result
    }
}
