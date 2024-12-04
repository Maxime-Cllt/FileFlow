/**
 * Initial state of the application for the store and the UI state
 */

export const initialDbConfig = {
    db_driver: '',
    db_host: '',
    port: '',
    username: '',
    password: '',
    db_name: '',
    tableName: '',
    sqliteFilePath: '',
    is_connected: false
};

export const initialUiState = {
    histoLog: 'Logs will appear here',
    filePath: null as string | null,
    fileName: '',
    mode: 'fast',
    showLoader: false,
    sqlite: false
};
