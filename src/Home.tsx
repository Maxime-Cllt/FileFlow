import React, {useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import './Loader.css';

import {Card, CardContent, CardHeader,} from "@/components/ui/card"
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

    const updateDbConfigField = (field: keyof typeof dbConfig, value: string) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    };

    const updateUiStateField = (field: keyof typeof uiState, value: any) => {
        setUiState(prev => ({...prev, [field]: value}));
    };

    const addLog = (message: string) => {
        updateUiStateField('histoLog', `${uiState.histoLog}\n${message}`);
    };

    const handleConnection = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!dbConfig.dbUrl || !dbConfig.port || !dbConfig.username) {
            addLog('Veuillez remplir tous les champs');
            return;
        }

        addLog('Tentative de connexion...');

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
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const checkFields = () => {
        return dbConfig.dbDriver && dbConfig.dbUrl && dbConfig.port && dbConfig.username && dbConfig.dbName && dbConfig.tableName;
    };

    const handleInsert = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!uiState.filePath) {
            addLog('Veuillez sélectionner un fichier à importer');
            return;
        }

        if (!checkFields()) {
            addLog('Veuillez remplir tous les champs');
            return;
        }

        addLog('Insertion en cours...');
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
            addLog(`Erreur d'insertion: ${error}`);
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
        setDbConfig({
            dbDriver: '',
            dbUrl: '',
            port: '',
            username: '',
            password: '',
            dbName: '',
            tableName: '',
            sqliteFilePath: '',
        });

        setUiState({
            histoLog: 'Historique des logs',
            filePath: null,
            fileName: '',
            fileSize: '',
            mode: uiState.mode,
            showLoader: false,
            sqlite: uiState.sqlite,
        });
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
            addLog(`Erreur de sauvegarde: ${error}`);
        }
    };

    const loadConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response = await invoke('load_database_config');
            if (typeof response === "string") {
                const loadDbConfig = JSON.parse(response);
                setDbConfig({
                    dbDriver: loadDbConfig.db_driver,
                    dbUrl: loadDbConfig.db_host,
                    port: loadDbConfig.port,
                    username: loadDbConfig.username,
                    password: loadDbConfig.password,
                    dbName: loadDbConfig.db_name,
                    tableName: loadDbConfig.table_name,
                    sqliteFilePath: loadDbConfig.sqlite_file_path || '',
                });
                updateUiStateField('sqlite', !!loadDbConfig.sqlite_file_path);
            } else {
                addLog('Aucune configuration trouvée');
            }
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
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
        <div>
            <div className={"fixed top-0 w-full"}>
                <Menu addLog={addLog} saveConfig={saveConfig} loadConfig={loadConfig}/>
            </div>

            {/* Formulaire */}
            <Card className={"mt-16"}>
                <CardHeader>
                    <CardContent>

                        {/* Form */}
                        {renderForm()}

                    </CardContent>

                    {/* Mode selection pour l'insertion */}
                    <ModeSelectionComponent {...{
                        mode: uiState.mode,
                        setMode: (value: string) => updateUiStateField('mode', value),
                    }}/>

                    {/* Loader */}
                    {
                        uiState.showLoader && <div className="flex justify-center mt-4">
                            <Loader/>
                        </div>
                    }

                    {/* Boutons en bas */}
                    <ButtonGroupComponent {...{
                        handleInsert: handleInsert,
                        handleSubmit: handleConnection,
                        handleReset: handleReset,
                    }}/>

                </CardHeader>
            </Card>

            {/* TextArea tout en bas */}
            <LogComponent {...{
                histoLog: uiState.histoLog,
            }}/>

        </div>
    );
}

export default Home;
