use csv::StringRecord;

/// This function is used to generate the column names for a CSV file.
pub fn get_formated_column_names(headers: &[String]) -> Vec<String> {
    const COLUMN_PREFIX: &str = "column_";
    let mut safe_headers: Vec<String> = Vec::with_capacity(headers.len());

    for (index, item) in headers.iter().enumerate() {
        let trimmed_column_name: &str = item.trim();
        if trimmed_column_name.is_empty() {
            safe_headers.push(format!("{COLUMN_PREFIX}{}", index + 1));
        } else {
            safe_headers.push(sanitize_column(trimmed_column_name));
        }
    }
    safe_headers
}

/// Sanitize a value for safe insertion into the database
pub fn sanitize_value(value: &str) -> String {
    let mut sanitized:String = String::with_capacity(value.len());

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
    value
        .trim()
        .chars()
        .filter_map(|c| match c {
            '\'' | '\\' | '\"' => None, // Remove these characters
            ' ' => Some('_'),           // Replace spaces with underscores
            _ => Some(c),               // Keep everything else
        })
        .collect()
}

/// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
pub fn escaped_record(values: StringRecord) -> String {
    let vec: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", sanitize_value(v)))
        .collect();
    vec.join(", ")
}
