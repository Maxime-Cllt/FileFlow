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
                safe_headers.push(Self::sanitize_column(trimmed_column_name));
            }
        }
        safe_headers
    }

    /// Sanitize a value for safe insertion into the database
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
    pub fn sanitize_column(value: &str) -> String {
        let trimmed = value.trim();
        let mut result = String::with_capacity(trimmed.len()); // Pre-allocate

        for c in trimmed.chars() {
            // Filter out unwanted characters first (early return)
            if (c.is_control() && c != '\n' && c != '\t')
                || c == '\u{feff}'  // BOM
                || c == '\u{200b}'  // Zero-width space
                || c == '\u{200c}'  // Zero-width non-joiner
                || c == '\u{200d}'
            // Zero-width joiner
            {
                continue;
            }

            match c {
                '\'' | '\\' | '\"' => continue, // Skip these
                ' ' => result.push('_'),
                _ => result.push(c.to_ascii_lowercase()),
            }
        }

        result
    }

    /// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
    pub fn escaped_record(values: StringRecord) -> String {
        let vec: Vec<String> = values
            .iter()
            .map(|v| format!("'{}'", Self::sanitize_value(v)))
            .collect();
        vec.join(", ")
    }
}
