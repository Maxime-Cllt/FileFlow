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

const Home: React.FC = () => {

    const [dbDriver, setdbDriver] = useState('postgres');
    const [dbUrl, setDbUrl] = useState('localhost');
    const [port, setPort] = useState('5432');
    const [username, setUsername] = useState('root');
    const [password, setPassword] = useState('root');
    const [dbName, setDbName] = useState('postgres');
    const [tableName, setTableName] = useState('');
    const [histoLog, setHistoLog] = useState('Historique des logs');
    const [filePath, setFilePath] = useState<string | null>(null);
    const [fileName, setFileName] = useState('');
    const [fileSize, setFileSize] = useState('');
    const [mode, setMode] = useState('fast');
    const [showLoader, setShowLoader] = useState(false);
    const [sqlite, setSqlite] = useState(false);
    const [sqliteFilePath, setSqliteFilePath] = useState('');


    const addLog = (message: string) => {
        setHistoLog((prev) => `${prev}\n${message}`);
    }

    const handleConnection = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();

        if (!dbUrl || !port || !username || !password || !dbName) {
            addLog('Veuillez remplir tous les champs');
            return;
        }

        addLog('Connexion en cours...');
        try {
            const response = await invoke('connect_to_database', {
                config: {
                    db_driver: dbDriver.toString().toLowerCase(),
                    db_host: dbUrl,
                    port: port,
                    username: username,
                    password: password,
                    db_name: dbName,
                    table_name: tableName,
                    sqlite_file_path: sqliteFilePath
                }
            });
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const checkFields = () => {
        return !(!dbUrl || !port || !username || !password || !dbName || !tableName);
    }

    const handleInsert = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();

        if (!filePath) {
            addLog('Veuillez sélectionner un fichier à importer');
            return;
        }

        if (!checkFields()) {
            addLog('Veuillez remplir tous les champs');
            return;
        }
        addLog('Insertion en cours...');
        setShowLoader(true);

        try {
            const response = await invoke('insert_csv_data', {
                csv: {
                    table_name: tableName,
                    file_path: filePath,
                    db_driver: dbDriver.toString().toLowerCase(),
                    mode: mode
                }
            });

            setShowLoader(false);
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur d'insertion: ${error}`);
        }
    }

    const handledbDriverChange = (value: React.SetStateAction<string>) => {
        setdbDriver(value);
        switch (value) {
            case 'mysql':
            case 'mariadb':
                setPort('3306');
                setSqlite(false);
                break;
            case 'postgres':
                setPort('5432');
                setSqlite(false);
                break;
            case 'sqlite':
                setSqlite(true);
                break;
            default:
                setPort('');
        }
    };

    const handleReset = () => {
        setDbUrl('');
        setPort('');
        setUsername('');
        setPassword('');
        setDbName('');
        setTableName('');
        setdbDriver('');
        setPort('');
        setHistoLog('');
        setFilePath(null);
        setFileName('');
        setFileSize('');
    };

    const saveConfig = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response = await invoke('save_database_config', {
                save: {
                    db_driver: dbDriver.toString().toLowerCase(),
                    db_host: dbUrl.toString(),
                    port: port.toString(),
                    username: username.toString(),
                    password: password.toString(),
                    db_name: dbName.toString(),
                    table_name: tableName.toString(),
                    sqlite_file_path: sqliteFilePath.toString()
                }
            });
            addLog(response as string);
            return;
        } catch (error) {
            addLog(`Erreur de sauvegarde: ${error}`);
        }
    };

    const loadConfig = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response = await invoke('load_database_config');
            if (typeof response === "string") {
                const dbConfig = JSON.parse(response);
                setdbDriver(dbConfig.db_driver.toString());
                setDbUrl(dbConfig.db_host.toString());
                setPort(dbConfig.port.toString());
                setUsername(dbConfig.username.toString());
                setPassword(dbConfig.password.toString());
                setDbName(dbConfig.db_name.toString());
                setTableName(dbConfig.table_name.toString());

                if (dbConfig.sqlite_file_path.toString().length > 0) {
                    setSqliteFilePath(dbConfig.sqlite_file_path.toString());
                    setSqlite(true);
                }

            } else {
                addLog('Aucune configuration trouvée');
            }
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const renderForm = () => {
        if (sqlite) {
            return <SqliteFormComponent {...{
                addLog,
                sqliteFilePath,
                setSqliteFilePath,
                dbDriver,
                handledbDriverChange,
                fileName,
                fileSize,
                setFilePath,
                setFileName,
                setFileSize,
                setTableName
            }}/>;
        }
        return <FormComponent {...{
            dbUrl,
            setDbUrl,
            port,
            setPort,
            username,
            setUsername,
            password,
            setPassword,
            dbName,
            setDbName,
            tableName,
            setTableName,
            dbDriver,
            setdbDriver,
            histoLog,
            setHistoLog,
            filePath,
            setFilePath,
            fileName,
            setFileName,
            fileSize,
            setFileSize,
            mode,
            setMode,
            addLog,
            checkFields,
            handleInsert,
            handleSubmit: handleConnection,
            handleReset,
            handledbDriverChange,
            sqlite,
            setSqlite
        }}/>;
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
                        mode,
                        setMode
                    }}/>

                    {/* Loader */}
                    {
                        showLoader && <div className="flex justify-center mt-4">
                            <Loader/>
                        </div>
                    }

                </CardHeader>

                {/* Boutons en bas */}
                <ButtonGroupComponent {...{
                    handleInsert,
                    handleSubmit: handleConnection,
                    handleReset
                }}/>

            </Card>

            {/* TextArea tout en bas */}
            <LogComponent {...{
                histoLog
            }}/>

        </div>
    );
}

export default Home;
