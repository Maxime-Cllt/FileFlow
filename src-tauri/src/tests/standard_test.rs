use crate::fileflow::utils::fileflowlib::{detect_separator_in_file, escaped_values};
use crate::tests::utils::{generate_csv_file, remove_csv_file};
use csv::StringRecord;

#[tokio::test]
async fn test_detect_separator_in_file() {
    let csv_file_path: String =
        generate_csv_file(String::from("test_detect_separator_in_file")).unwrap();
    let separator: char = detect_separator_in_file(&csv_file_path).unwrap();
    assert_eq!(separator, ',');
    let _ = remove_csv_file(String::from("test_detect_separator_in_file"));
}

#[tokio::test]
async fn test_escape_values() {
    let record: StringRecord = StringRecord::from(vec!["value1", "value2"]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'value1', 'value2'");

    let record: StringRecord = StringRecord::from(vec![
        "INSERT INTO test_table VALUES (1,2);",
        "UPDATE test_table SET column1 = 1;",
        "DELETE FROM test_table WHERE column1 = 1;",
        "SELECT * FROM test_table;",
    ]);
    let values: String = escaped_values(record);
    assert_eq!(values, "'INSERT INTO test_table VALUES (1,2);', 'UPDATE test_table SET column1 = 1;', 'DELETE FROM test_table WHERE column1 = 1;', 'SELECT * FROM test_table;'");
}
