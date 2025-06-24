use criterion::{criterion_group, criterion_main, Criterion};
use csv::{Reader, ReaderBuilder};
use fileflow::fileflow::action::database_command::fast_insert;
use fileflow::fileflow::database::connection::Connection;
use fileflow::fileflow::enumeration::database_engine::DatabaseEngine;
use fileflow::fileflow::stuct::db_config::DbConfig;
use fileflow::fileflow::stuct::string_formater::StringFormatter;
use fileflow::fileflow::utils::csv_utils::{find_separator, read_first_line};
use sqlx::Error;
use std::fs::File;

async fn test_insert_file() {
    const TEST_FILE: &str = r"C:\Users\HHBL8703\Downloads\reference.csv";

    let config: DbConfig = DbConfig {
        db_driver: DatabaseEngine::SQLite,
        username: String::new(),
        password: String::new(),
        db_host: String::new(),
        port: String::new(),
        db_name: String::new(),
        sqlite_file_path: r"C:\Users\HHBL8703\RustroverProjects\FileFlow\test.db".into(),
    };

    let conn: Result<Connection, Error> = Connection::connect(&config).await;

    if conn.is_err() {
        eprintln!("Error connecting to the database");
        return;
    }

    let connection: &Connection = conn.as_ref().unwrap();

    let file: File = match File::open(TEST_FILE) {
        Ok(file) => file,
        Err(_) => return,
    };

    let first_line: String = read_first_line(TEST_FILE).unwrap();

    let separator: char = find_separator(&first_line).unwrap();

    let final_columns_name: Vec<String> = StringFormatter::get_formated_column_names(
        &first_line
            .split(separator)
            .map(|s| StringFormatter::sanitize_column(s))
            .collect::<Vec<String>>(),
    );

    let mut reader: Reader<File> = ReaderBuilder::new()
        .delimiter(u8::try_from(separator).unwrap())
        .has_headers(true)
        .from_reader(file);

    fast_insert(
        connection,
        &mut reader,
        &final_columns_name,
        "test_table",
        &DatabaseEngine::SQLite,
    )
    .await
    .unwrap();
}

fn benchmark_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark_application");

    group.bench_function("test_insert_file", |b| {
        b.iter(async || {
            test_insert_file().await;
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_application);
criterion_main!(benches);
