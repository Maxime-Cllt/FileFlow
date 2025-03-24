export interface DatabaseConfig {
    config_name: string;
    db_driver: string;
    db_host: string;
    db_name: string;
    password: string;
    port: string;
    sqlite_file_path: string;
    username: string;
    is_connected: boolean;
}
