use csv::StringRecord;
use std::borrow::Cow;

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
    pub fn sanitize_value(value: &'_ str) -> Cow<'_, str> {
        let trimmed: &str = value.trim();

        let needs_sanitization: bool = trimmed
            .chars()
            .any(|c| matches!(c, '\'' | '\\' | '\"' | '\0' | '\r' | '\n'));

        if !needs_sanitization {
            return Cow::Borrowed(trimmed);
        }

        let mut sanitized = String::with_capacity(trimmed.len());

        for c in trimmed.chars() {
            match c {
                '\'' => sanitized.push_str("''"),   // Escape single quotes
                '\\' => sanitized.push_str("\\\\"), // Escape backslashes
                '\"' | '\0' => {}                   // Remove double quotes and null characters
                '\r' | '\n' => sanitized.push(' '), // Normalize newlines
                _ => sanitized.push(c),
            }
        }

        Cow::Owned(sanitized)
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

#[cfg(test)]
mod tests {
    use crate::fileflow::stuct::string_formater::StringFormatter;
    use csv::StringRecord;

    #[tokio::test]
    async fn test_escape_values() {
        let record: StringRecord = StringRecord::from(vec!["value1", "value2"]);
        let values: String = StringFormatter::escape_record(record);
        assert_eq!(values, "'value1', 'value2'");

        let record: StringRecord = StringRecord::from(vec![
            "\"INSERT INTO test_table VALUES (1,2);\"",
            "UPDATE test_table SET column1 = 1;",
            "DELETE FROM test_table WHERE column1 = 1;",
            "SELECT * FROM test_table;",
        ]);
        let values: String = StringFormatter::escape_record(record);
        assert_eq!(values, "'INSERT INTO test_table VALUES (1,2);', 'UPDATE test_table SET column1 = 1;', 'DELETE FROM test_table WHERE column1 = 1;', 'SELECT * FROM test_table;'");
    }

    #[tokio::test]
    async fn test_sanitize_value() {
        assert_eq!(StringFormatter::sanitize_value("value1"), "value1");
        assert_eq!(StringFormatter::sanitize_value("'value1'"), "''value1''");
        assert_eq!(StringFormatter::sanitize_value("value1'"), "value1''");
        assert_eq!(
            StringFormatter::sanitize_value("value1\\value2"),
            "value1\\\\value2"
        );
        assert_eq!(
            StringFormatter::sanitize_value("value1\"value2"),
            "value1value2"
        );
    }

    #[tokio::test]
    async fn test_sanitize_column() {
        // Standard cases
        assert_eq!(StringFormatter::sanitize_column("column1"), "column1");
        assert_eq!(StringFormatter::sanitize_column("'column1'"), "column1");
        assert_eq!(StringFormatter::sanitize_column("column1'"), "column1");
        assert_eq!(
            StringFormatter::sanitize_column("column1\\column2"),
            "column1column2"
        );
        assert_eq!(
            StringFormatter::sanitize_column("column1\"column2"),
            "column1column2"
        );
        assert_eq!(StringFormatter::sanitize_column("column 1"), "column_1");
        assert_eq!(StringFormatter::sanitize_column("column 1'"), "column_1");
        assert_eq!(StringFormatter::sanitize_column("COlUMn 1'"), "column_1");
        assert_eq!(StringFormatter::sanitize_column("  CoDE_DEP  "), "code_dep");
        assert_eq!(StringFormatter::sanitize_column("  column\\1  "), "column1");
        assert_eq!(StringFormatter::sanitize_column(" COlu.Mn/1"), "column1");

        // Non-printable characters
        assert_eq!(
            StringFormatter::sanitize_column("\u{feff}code_departement"),
            "code_departement"
        ); // BOM
        assert_eq!(
            StringFormatter::sanitize_column("code\u{0000}_dep"),
            "code_dep"
        ); // NULL
        assert_eq!(
            StringFormatter::sanitize_column("code\u{001F}_dep"),
            "code_dep"
        ); // Unit Separator
        assert_eq!(StringFormatter::sanitize_column("code\n_dep"), "code\n_dep"); // newline is preserved (can change if undesired)
        assert_eq!(StringFormatter::sanitize_column("code\tdep"), "code\tdep"); // tab is preserved (can change if undesired)
        assert_eq!(StringFormatter::sanitize_column("code\rdep"), "codedep"); // carriage return removed
    }

    #[tokio::test]
    async fn test_get_formated_column_names() {
        assert_eq!(
            StringFormatter::get_formated_column_names(&vec!["header 1".into(), " header2".into()]),
            vec!["header_1", "header2"]
        );
        assert_eq!(
            StringFormatter::get_formated_column_names(&vec![
                "header    1".into(),
                String::new(),
                "header2".into(),
                "".into()
            ]),
            vec!["header____1", "column_2", "header2", "column_4"]
        );
        assert_eq!(
            StringFormatter::get_formated_column_names(&vec![
                "header'\" 1".into(),
                "header 2''\\".into()
            ]),
            vec!["header_1", "header_2"]
        );
    }
}
