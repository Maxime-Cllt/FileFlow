import React from 'react';
import {Database, Eraser, Upload} from "lucide-react";
import {toast} from "sonner";
import {invoke} from "@tauri-apps/api/core";

interface ButtonGroupProps {
    dbConfig: {
        db_driver: string;
        db_host: string;
        port: string;
        username: string;
        password: string;
        db_name: string;
        tableName: string;
        sqlite_file_path: string;
        is_connected: boolean;
    };
    uiState: {
        fileName: string;
        filePath: string | null;
        histoLog: string;
        showLoader: boolean;
        sqlite: boolean;
        mode: string;
    };
    updateUiStateField: (field: any, value: any) => void;
    updateDbConfigField: (field: any, value: any) => void;
    addLog: (message: string) => void;
}

const ButtonGroupAction: React.FC<ButtonGroupProps> = (props: ButtonGroupProps) => {

    const handleInsert = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!props.uiState.filePath) {
            toast.warning('Please select a file');
            return;
        }

        if (!props.dbConfig.is_connected) {
            toast.warning('Please connect to the database');
            return;
        }

        props.addLog('Inserting data...');
        props.updateUiStateField('showLoader', true);

        try {
            const response = await invoke('insert_csv_data', {
                csv: {
                    table_name: props.dbConfig.tableName,
                    file_path: props.uiState.filePath,
                    db_driver: props.dbConfig.db_driver.toLowerCase(),
                    mode: props.uiState.mode,
                },
            });

            props.updateUiStateField('showLoader', false);
            props.addLog(response as string);
            toast.success('Data inserted successfully');
        } catch (error) {
            props.addLog(`Insert error: ${error}`);
            props.updateUiStateField('showLoader', false);
            toast.error('Error inserting data');
        }
    };

    const handleReset = () => {

        props.updateDbConfigField('db_driver', '');
        props.updateDbConfigField('db_host', '');
        props.updateDbConfigField('port', '');
        props.updateDbConfigField('username', '');
        props.updateDbConfigField('password', '');
        props.updateDbConfigField('db_name', '');
        props.updateDbConfigField('tableName', '');
        props.updateDbConfigField('sqlite_file_path', '');


        props.updateUiStateField('fileName', '');
        props.updateUiStateField('filePath', null);
        props.updateUiStateField('histoLog', '');
        props.updateUiStateField('showLoader', false);
    };

    const handleDeconnection = async (e: React.MouseEvent) => {
        e.preventDefault();
        try {
            if (!props.dbConfig.is_connected) {
                toast.warning('You are not connected to any database');
                return;
            }
            await invoke('disconnect_from_database');
            props.updateDbConfigField('is_connected', false);
            toast.success('Disconnected successfully');
        } catch (error) {
            props.addLog(error as string);
            toast.error('Error disconnecting');
        }
    };

    const handleConnection = async (e: React.FormEvent) => {
        e.preventDefault();

        if (props.dbConfig.is_connected) {
            handleDeconnection(e as React.MouseEvent).then(() => {
                props.updateDbConfigField('is_connected', false);
            });
            return;
        }

        if (!props.dbConfig.db_driver && props.dbConfig.db_driver !== 'sqlite') {
            if (!props.dbConfig.db_host || !props.dbConfig.port || !props.dbConfig.username) {
                toast.warning('Please fill in all the required fields');
                return;
            }
        } else if (props.dbConfig.db_driver === 'sqlite') {
            if (!props.dbConfig.sqlite_file_path) {
                toast.warning('Please select a SQLite file');
                return;
            }
        }

        try {
            const response = await invoke('connect_to_database', {
                config: {
                    db_driver: props.dbConfig.db_driver.toLowerCase(),
                    db_host: props.dbConfig.db_host,
                    port: props.dbConfig.port,
                    username: props.dbConfig.username,
                    password: props.dbConfig.password,
                    db_name: props.dbConfig.db_name,
                    table_name: props.dbConfig.tableName,
                    sqlite_file_path: props.dbConfig.sqlite_file_path,
                },
            });

            if (response !== 'true') {
                props.addLog(response as string);
                toast.error('Connection failed');
                return;
            }

            toast.success('Connected successfully');
            props.addLog("Connected to database " + props.dbConfig.db_name + " with user " + props.dbConfig.username);
            props.updateDbConfigField('is_connected', true);
        } catch (error) {
            props.addLog(`Connection error: ${error}`);
            toast.error('Connection failed');
        }
    }


    return (
        <div className="flex items-center justify-center gap-6 mb-6 p-4">
            <button
                onClick={handleConnection}
                className={`flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 ${
                    props.dbConfig.is_connected ? 'bg-green-500 hover:bg-green-600 text-white' : 'bg-red-500 text-white'
                }`}
                aria-label="Submit to Database"

                title={props.dbConfig.is_connected ? "Submit data to the specified database" : "Connect to a database first"}
            >
                <Database className="w-5 h-5"/>
            </button>

            <button
                onClick={handleInsert}
                className={`flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 ${
                    props.dbConfig.is_connected ? 'bg-blue-500 hover:bg-blue-600 text-white' : 'bg-gray-500 text-gray-700'
                }`}
                aria-label="Insert Data"

                title={props.dbConfig.is_connected ? "Insert data to the specified database" : "Connect to a database first"}
            >
                <Upload className="w-5 h-5"/>
            </button>

            <button
                onClick={handleReset}
                className="flex items-center justify-center bg-yellow-500 hover:bg-yellow-600 text-white p-3 rounded-full shadow-lg transition duration-300"
                aria-label="Reset" title={"Reset the form"}
            >
                <Eraser className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default ButtonGroupAction;
