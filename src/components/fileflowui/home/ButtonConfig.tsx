import React from 'react';
import {ArrowDownFromLine, SaveAll} from "lucide-react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

interface ButtonConfigComponent {
    dbConfig: {
        dbDriver: string;
        dbUrl: string;
        port: string;
        username: string;
        password: string;
        dbName: string;
        tableName: string;
        sqliteFilePath: string;
    };
    setDbConfig: (dbConfig: any) => void;
    addLog: (message: string) => void;
    updateUiStateField: (field: any, value: any) => void;
}

const ButtonConfigComponent: React.FC<ButtonConfigComponent> = ({
                                                                    dbConfig,
                                                                    setDbConfig,
                                                                    addLog,
                                                                    updateUiStateField
                                                                }) => {


    const saveConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            await invoke('save_database_config', {
                save: {
                    db_driver: dbConfig.dbDriver.toLowerCase(),
                    db_host: dbConfig.dbUrl,
                    port: dbConfig.port,
                    username: dbConfig.username,
                    password: dbConfig.password,
                    db_name: dbConfig.dbName,
                    table_name: dbConfig.tableName,
                    sqlite_file_path: dbConfig.sqliteFilePath,
                },
            });
            toast.success('Config saved successfully');
        } catch (error) {
            toast.error('Error saving config');
            addLog(`Error saving config: ${error}`);
        }
    };

    const loadConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response = await invoke('load_database_config');
            if (typeof response === "string") {
                const loadDbConfig = JSON.parse(response);

                setDbConfig({
                    dbDriver: loadDbConfig.db_driver || "",
                    dbUrl: loadDbConfig.db_host || "",
                    port: loadDbConfig.port || "",
                    username: loadDbConfig.username || "",
                    password: loadDbConfig.password || "",
                    dbName: loadDbConfig.db_name || "",
                    tableName: loadDbConfig.table_name || "",
                    sqliteFilePath: loadDbConfig.sqlite_file_path || "",
                    is_connected: false,
                });
                updateUiStateField('sqlite', !!loadDbConfig.sqlite_file_path);
                toast.success('Config loaded successfully');
            } else {
                addLog('Error loading config');
                toast.error('Error loading config');
            }
        } catch (error) {
            addLog(`Error loading config: ${error}`);
            toast.error('Error loading config');
        }
    };

    return (
        <div className="flex space-x-4">

            {/* Save config button */}
            <button
                onClick={saveConfig}
                className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-green-500 hover:bg-green-600 text-white"
                aria-label="Save Config"
                title="Save Config"
            >
                <SaveAll className="w-5 h-5"/>
            </button>

            {/* Load config button */}
            <button
                onClick={loadConfig}
                aria-label="Load Config"
                title="Load Config"
                className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-blue-500 hover:bg-blue-600 text-white"
            >
                <ArrowDownFromLine className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default ButtonConfigComponent;

