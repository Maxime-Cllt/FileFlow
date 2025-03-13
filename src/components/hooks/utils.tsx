// Return the file name from a path
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

export const getFileNameFromPath = (path: string) => path.split("\\").toString().split('/').pop() || '';


// Return the normalized table name from a path
export const getNormalizedTableName = (path: string) => {
    const fileName = getFileNameFromPath(path).split('.').shift() || '';
    return fileName
        .replace(/[^a-zA-Z0-9_]/g, '')
        .replace(/^_/, '').toLowerCase();
};

// Get all the configs for the database connection
export const getAllConfigs = async (): Promise<Array<Item>> => {
    try {
        const response: string | boolean = await invoke<string | boolean>('get_all_database_configs_name');

        if (typeof response !== "string") {
            throw new Error('Error getting all configs');
        }

        const configs = JSON.parse(response) as string[];

        return configs.map((config: string) => ({id: config}));
    } catch (error) {
        log_error(error);
        return [];
    }
};

// Load a config for the database connection from its name
export const loadConfig = async (item: string): Promise<string | boolean> => {
    try {
        const response: string | boolean = await invoke<string | boolean>('load_database_config_by_name', {
            name: item,
        });

        if (typeof response !== "string") {
            throw new Error('Error loading config');
        }

        return response;
    } catch (error) {
        log_error(error);
        return false;
    }
};

export const is_connected = async (): Promise<string | boolean> => {
    try {
        const response: string | boolean = await invoke<string | boolean>('is_connected');

        if (typeof response !== "string") {
            throw new Error("Failed to check connection");
        }

        if (response === '') {
            return false;
        }

        return response;
    } catch (error) {
        log_error(error);
        return false;
    }
}

// Disconnect the user from the current session
export const disconnect_from_database = async () => {
    try {
        const response = await invoke<boolean | string>('disconnect_from_database');

        if (typeof response !== 'boolean') {
            throw new Error("Failed to disconnect");
        }

        toast.success('Disconnected successfully');
        return true;
    } catch (error) {
        log_error(error);
        return false;
    }
}

// Connect to the database and disconnect if already connected for the user
export const connect_to_database = async (db_driver: string, db_host: string, port: string, username: string, password: string, db_name: string, sqlite_file_path: string): Promise<void | boolean> => {
    try {
        const is_connected_response: string | boolean = await is_connected();

        if (typeof is_connected_response === "boolean" && is_connected_response) {
            await disconnect_from_database();
        }

        if (db_driver !== 'sqlite' && (!db_host || !port || !username)) {
            toast.warning('Please fill in all the required fields');
            return;
        } else if (db_driver === 'sqlite' && sqlite_file_path === '') {
            toast.warning('Please select a SQLite file');
            return;
        }

        const response: string | boolean = await invoke<string | boolean>('connect_to_database', {
            config: {
                db_driver: db_driver.toLowerCase(),
                db_host: db_host,
                port: port,
                username: username,
                password: password,
                db_name: db_name,
                sqlite_file_path: sqlite_file_path,
            },
        });

        if (typeof response !== "boolean") {
            throw new Error(response as string);
        }

        toast.success('Connected successfully to ' + db_name);
        return true;
    } catch (error) {
        log_error(error);
        return false;
    }
}

// Log the error message
export const log_error = (error: any) => {
    if (error instanceof Error) {
        toast.error(error.message);
    } else if (typeof error === 'string') {
        toast.error(error);
    } else {
        toast.error('An unknown error occurred');
    }
}

