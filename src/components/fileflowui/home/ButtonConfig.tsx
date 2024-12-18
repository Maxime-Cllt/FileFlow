import React from 'react';
import {ArrowDownFromLine, SaveAll} from "lucide-react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

interface ButtonConfigComponent {
    dbConfig: {
        db_driver: string;
        db_host: string;
        port: string;
        username: string;
        password: string;
        db_name: string;
        tableName: string;
        sqlite_file_path: string;
    };
    updateDbConfigField: (field: any, value: any) => void;
    addLog: (message: string) => void;
}

const ButtonConfigComponent: React.FC<ButtonConfigComponent> = ({
                                                                    dbConfig,
                                                                    updateDbConfigField,
                                                                    addLog
                                                                }) => {


    const saveConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            await invoke('save_database_config', {
                save: {
                    db_driver: dbConfig.db_driver.toLowerCase(),
                    db_host: dbConfig.db_host,
                    port: dbConfig.port,
                    username: dbConfig.username,
                    password: dbConfig.password,
                    db_name: dbConfig.db_name,
                    table_name: dbConfig.tableName,
                    sqlite_file_path: dbConfig.sqlite_file_path,
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

                Object.keys(loadDbConfig).forEach((key) => {
                    updateDbConfigField(key, loadDbConfig[key]);
                });
                updateDbConfigField('is_connected', false);

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

            {/* Load config button */}
            <button
                onClick={loadConfig}
                aria-label="Load Config"
                title="Load Config"
                className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-blue-500 hover:bg-blue-600 text-white"
            >
                <ArrowDownFromLine className="w-5 h-5"/>
            </button>

            {/* Save config button */}
            <button
                onClick={saveConfig}
                className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-green-500 hover:bg-green-600 text-white"
                aria-label="Save Config"
                title="Save Config"
            >
                <SaveAll className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default ButtonConfigComponent;

