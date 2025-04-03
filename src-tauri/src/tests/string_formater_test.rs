use crate::fileflow::utils::string_formater::{
    escaped_values, get_formated_column_names, sanitize_column, sanitize_value,
};
use csv::StringRecord;

#[tokio::test]
async fn test_escape_values() {
    let record: StringRecord = StringRecord::from(vec!["value1", "value2"]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'value1', 'value2'");

    let record: StringRecord = StringRecord::from(vec![
        "\"INSERT INTO test_table VALUES (1,2);\"",
        "UPDATE test_table SET column1 = 1;",
        "DELETE FROM test_table WHERE column1 = 1;",
        "SELECT * FROM test_table;",
    ]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'INSERT INTO test_table VALUES (1,2);', 'UPDATE test_table SET column1 = 1;', 'DELETE FROM test_table WHERE column1 = 1;', 'SELECT * FROM test_table;'");
}

#[tokio::test]
async fn test_sanitize_value() {
    assert_eq!(sanitize_value("value1"), "value1");
    assert_eq!(sanitize_value("'value1'"), "''value1''");
    assert_eq!(sanitize_value("value1'"), "value1''");
    assert_eq!(sanitize_value("value1\\value2"), "value1\\\\value2");
    assert_eq!(sanitize_value("value1\"value2"), "value1value2");
}

#[tokio::test]
async fn test_sanitize_column() {
    assert_eq!(sanitize_column("column1"), "column1");
    assert_eq!(sanitize_column("'column1'"), "column1");
    assert_eq!(sanitize_column("column1'"), "column1");
    assert_eq!(sanitize_column("column1\\column2"), "column1_column2");
    assert_eq!(sanitize_column("column1\"column2"), "column1column2");
    assert_eq!(sanitize_column("column 1"), "column_1");
    assert_eq!(sanitize_column("column 1'"), "column_1");
}

#[tokio::test]
async fn test_get_formated_column_names() {
    assert_eq!(
        get_formated_column_names(&vec!["header 1".into(), " header2".into()]),
        vec!["header_1", "header2"]
    );
    assert_eq!(
        get_formated_column_names(&vec!["header    1".into(), String::new(), "header2".into()]),
        vec!["header____1", "column_2", "header2"]
    );
    assert_eq!(
        get_formated_column_names(&vec!["header'\" 1".into(), "header 2''\\".into()]),
        vec!["header_1", "header_2"]
    );
}
