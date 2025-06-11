use crate::fileflow::stuct::string_formater::{StringFormatter};
use csv::StringRecord;

#[tokio::test]
async fn test_escape_values() {
    let record: StringRecord = StringRecord::from(vec!["value1", "value2"]);
    let values: String = StringFormatter::escaped_record(record);
    assert_eq!(values, "'value1', 'value2'");

    let record: StringRecord = StringRecord::from(vec![
        "\"INSERT INTO test_table VALUES (1,2);\"",
        "UPDATE test_table SET column1 = 1;",
        "DELETE FROM test_table WHERE column1 = 1;",
        "SELECT * FROM test_table;",
    ]);
    let values: String = StringFormatter::escaped_record(record);
    assert_eq!(values, "'INSERT INTO test_table VALUES (1,2);', 'UPDATE test_table SET column1 = 1;', 'DELETE FROM test_table WHERE column1 = 1;', 'SELECT * FROM test_table;'");
}

#[tokio::test]
async fn test_sanitize_value() {
    assert_eq!(StringFormatter::sanitize_value("value1"), "value1");
    assert_eq!(StringFormatter::sanitize_value("'value1'"), "''value1''");
    assert_eq!(StringFormatter::sanitize_value("value1'"), "value1''");
    assert_eq!(StringFormatter::sanitize_value("value1\\value2"), "value1\\\\value2");
    assert_eq!(StringFormatter::sanitize_value("value1\"value2"), "value1value2");
}

#[tokio::test]
async fn test_sanitize_column() {
    // Standard cases
    assert_eq!(StringFormatter::sanitize_column("column1"), "column1");
    assert_eq!(StringFormatter::sanitize_column("'column1'"), "column1");
    assert_eq!(StringFormatter::sanitize_column("column1'"), "column1");
    assert_eq!(StringFormatter::sanitize_column("column1\\column2"), "column1column2");
    assert_eq!(StringFormatter::sanitize_column("column1\"column2"), "column1column2");
    assert_eq!(StringFormatter::sanitize_column("column 1"), "column_1");
    assert_eq!(StringFormatter::sanitize_column("column 1'"), "column_1");
    assert_eq!(StringFormatter::sanitize_column("COlUMn 1'"), "column_1");
    assert_eq!(StringFormatter::sanitize_column("CoDE_DEP"), "code_dep");
    assert_eq!(StringFormatter::sanitize_column("  CoDE_DEP  "), "code_dep");

    // Non-printable characters
    assert_eq!(StringFormatter::sanitize_column("\u{feff}code_departement"), "code_departement"); // BOM
    assert_eq!(StringFormatter::sanitize_column("code\u{0000}_dep"), "code_dep");                // NULL
    assert_eq!(StringFormatter::sanitize_column("code\u{001F}_dep"), "code_dep");                // Unit Separator
    assert_eq!(StringFormatter::sanitize_column("code\n_dep"), "code\n_dep");                    // newline is preserved (can change if undesired)
    assert_eq!(StringFormatter::sanitize_column("code\tdep"), "code\tdep");                      // tab is preserved (can change if undesired)
    assert_eq!(StringFormatter::sanitize_column("code\rdep"), "codedep");                        // carriage return removed
}

#[tokio::test]
async fn test_get_formated_column_names() {
    assert_eq!(
        StringFormatter::get_formated_column_names(&vec!["header 1".into(), " header2".into()]),
        vec!["header_1", "header2"]
    );
    assert_eq!(
        StringFormatter::get_formated_column_names(&vec!["header    1".into(), String::new(), "header2".into(), "".into()]),
        vec!["header____1", "column_2", "header2", "column_4"]
    );
    assert_eq!(
        StringFormatter::get_formated_column_names(&vec!["header'\" 1".into(), "header 2''\\".into()]),
        vec!["header_1", "header_2"]
    );
}
