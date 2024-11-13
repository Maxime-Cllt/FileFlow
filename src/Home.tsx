import React, {useCallback, useEffect, useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import './Loader.css';
import {Card, CardContent, CardHeader} from "@/components/ui/card";
import Menu from "@/components/fileflowui/Menu.tsx";
import Loader from "@/Loader.tsx";
import FormComponent from "@/components/fileflowui/FormComponent.tsx";
import ModeSelectionComponent from "@/components/fileflowui/ModeSelectionComponent.tsx";
import ButtonGroupComponent from "@/components/fileflowui/ButtonGroupComponent.tsx";
import LogComponent from "@/components/fileflowui/LogComponent.tsx";
import SqliteFormComponent from "@/components/fileflowui/SqliteFormComponent.tsx";
import {initialDbConfig, initialUiState} from "@/components/object/initialState.tsx";

const Home: React.FC = () => {
    const [dbConfig, setDbConfig] = useState(initialDbConfig);
    const [uiState, setUiState] = useState(initialUiState);

    useEffect(() => {
        console.log("on modifie")
        console.log(uiState)
    }, [uiState]);

    const updateDbConfigField = useCallback((field: keyof typeof dbConfig, value: string) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    }, []);

    const updateUiStateField = (field: keyof typeof uiState, value: any) => {
        setUiState(prev => ({...prev, [field]: value}));
    };

    const addLog = (message: string) => {
        updateUiStateField('histoLog', `${uiState.histoLog}\n${message}`);
    };

    const handleConnection = async (e: React.FormEvent) => {
        e.preventDefault();
        const newConfig = {...dbConfig};

        if (!dbConfig.dbUrl || !dbConfig.port || !dbConfig.username) {
            addLog('Please fill all fields');
            return;
        }

        addLog('Trying to connect...');

        try {
            const response = await invoke('connect_to_database', {
                config: {
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
            addLog(response as string);
            newConfig.is_connected = true;
            addLog('Connected successfully!');
        } catch (error) {
            addLog(`Connection error: ${error}`);
            newConfig.is_connected = false;
            addLog('Connection failed!');
        }
        setDbConfig(newConfig); // Trigger re-render by updating state
    };

    const handleInsert = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!uiState.filePath) {
            addLog('Please select a file');
            return;
        }

        if (!dbConfig.is_connected) {
            addLog('Please connect to the database');
            return;
        }

        addLog('Inserting data...');
        updateUiStateField('showLoader', true);

        try {
            const response = await invoke('insert_csv_data', {
                csv: {
                    table_name: dbConfig.tableName,
                    file_path: uiState.filePath,
                    db_driver: dbConfig.dbDriver.toLowerCase(),
                    mode: uiState.mode,
                },
            });

            updateUiStateField('showLoader', false);
            addLog(response as string);
        } catch (error) {
            addLog(`Insert error: ${error}`);
            updateUiStateField('showLoader', false);
        }
    };

    const handledbDriverChange = (value: string) => {
        const portMap: Record<string, string> = {mysql: '3306', mariadb: '3306', postgres: '5432'};
        setDbConfig(prev => ({
            ...prev,
            dbDriver: value,
            port: portMap[value] || '',
        }));
        updateUiStateField('sqlite', value === 'sqlite');
    };

    const handleReset = () => {
        setDbConfig(prev => ({
            ...prev,
            dbDriver: '',
            dbUrl: '',
            port: '',
            username: '',
            password: '',
            dbName: '',
            tableName: '',
            sqliteFilePath: '',
            is_connected: false,
        }));

        setUiState(prev => ({
            ...prev,
            fileName: '',
            fileSize: '',
            filePath: null,
            histoLog: '',
            showLoader: false,
        }));
    };

    const saveConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response = await invoke('save_database_config', {
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
            addLog(response as string);
        } catch (error) {
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
            } else {
                addLog('Error loading config');
            }
        } catch (error) {
            addLog(`Error loading config: ${error}`);
        }
    };

    const handleDeconnection = async (e: React.MouseEvent) => {
        e.preventDefault();
        try {
            const response: unknown = await invoke('disconnect_from_database');
            addLog(response as string);
            dbConfig.is_connected = false;
        } catch (error) {
            addLog(error as string);
        }
    };


    const renderForm = () => {
        if (uiState.sqlite) {
            return (
                <SqliteFormComponent
                    {...{
                        addLog,
                        sqliteFilePath: dbConfig.sqliteFilePath,
                        setSqliteFilePath: (value: string) => updateDbConfigField('sqliteFilePath', value),
                        dbDriver: dbConfig.dbDriver,
                        handledbDriverChange,
                        fileName: uiState.fileName,
                        fileSize: uiState.fileSize,
                        setFilePath: (value: string | null) => updateUiStateField('filePath', value),
                        setFileName: (value: string) => updateUiStateField('fileName', value),
                        setFileSize: (value: string) => updateUiStateField('fileSize', value),
                        setTableName: (value: string) => updateDbConfigField('tableName', value),
                    }}
                />
            );
        }

        return (
            <FormComponent
                {...{
                    dbConfig,
                    uiState,
                    setters: {
                        setDbUrl: (value: string) => updateDbConfigField('dbUrl', value),
                        setPort: (value: string) => updateDbConfigField('port', value),
                        setUsername: (value: string) => updateDbConfigField('username', value),
                        setPassword: (value: string) => updateDbConfigField('password', value),
                        setDbName: (value: string) => updateDbConfigField('dbName', value),
                        setTableName: (value: string) => updateDbConfigField('tableName', value),
                        setFilePath: (filePath: string | null) => updateUiStateField('filePath', filePath),
                        setFileName: (name: string) => updateUiStateField('fileName', name),
                        setFileSize: (size: string) => updateUiStateField('fileSize', size),
                        setMode: (mode: string) => updateUiStateField('mode', mode),
                    },
                    actions: {
                        addLog,
                        handledbDriverChange,
                    },
                }}
            />
        );
    };

    return (
        <div className="min-h-screen bg-gray-100">

            {/* Fixed Navigation Bar */}
            <div className="fixed top-0 w-full z-50 bg-white shadow-md">
                <Menu handleDeconnection={handleDeconnection} saveConfig={saveConfig} loadConfig={loadConfig}/>
            </div>

            {/* Main Content */}
            <div className="pt-16 px-4 md:px-8">
                <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                    {/* Card Header with Form */}
                    <CardHeader className="border-b-2 border-gray-200 pb-4">
                        <CardContent>
                            {/* Render Form */}
                            {renderForm()}
                        </CardContent>
                    </CardHeader>

                    {/* Mode Selection Component */}
                    <div className="mt-6">
                        <ModeSelectionComponent setMode={(value: string) => updateUiStateField('mode', value)}/>
                    </div>

                    {/* Loader */}
                    {uiState.showLoader && (
                        <div className="flex justify-center mt-6">
                            <Loader/>
                        </div>
                    )}

                    {/* Button Group */}
                    <div className="flex justify-center mt-6">
                        <ButtonGroupComponent
                            handleInsert={handleInsert}
                            handleSubmit={handleConnection}
                            handleReset={handleReset}
                            is_connected={dbConfig.is_connected}
                        />
                    </div>
                </Card>

                {/* Logs Section */}
                <div className="mt-8 bg-gray-50 p-4 rounded-lg shadow-inner">
                    <LogComponent histoLog={uiState.histoLog}/>
                </div>
            </div>
        </div>
    );
};

export default Home;
