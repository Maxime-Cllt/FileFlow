import React, {useState} from 'react';
import {Input} from "@/components/ui/input";
import {Button} from "@/components/ui/button";
import {invoke} from '@tauri-apps/api/tauri';
import {Label} from "@/components/ui/label";
import {Textarea} from "@/components/ui/textarea";
import './Loader.css';

import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group"

import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

import {
    Card,
    CardContent,
    CardHeader,
} from "@/components/ui/card"
import {Database, Eraser, FileArchive, Upload} from "lucide-react";
import Menu from "@/Menu.tsx";
import {dialog} from "@tauri-apps/api";
import Loader from "@/Loader.tsx";


function Home() {
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
    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: false
            });

            if (selectedFilePath) {

                setTableName(normalizeTableName(selectedFilePath.toString()));
                setFileName(selectedFilePath.toString().split('/').pop() || '');
                setFilePath(selectedFilePath.toString());
                const response = await invoke('get_size_of_file', {
                    filePath: selectedFilePath
                });

                if (typeof response === "string") {
                    setFileSize(response);
                    return;
                }
            }
        } catch (error) {
            addLog(`Erreur lors de la sélection du fichier: ${error}`);
        }
    };

    const addLog = (message: string) => {
        setHistoLog((prev) => `${prev}\n${message}`);
    }

    const handleSubmit = async (e: { preventDefault: () => void; }) => {
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
                }
            });
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const handleDeconnection = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response = await invoke('disconnect_from_database');
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const normalizeTableName = (tableName: string) => {
        if (!tableName || tableName.length === 0 || tableName.indexOf('.') === -1) {
            return '';
        }
        tableName = tableName.split('/').pop() as string;
        tableName = tableName.split('.').shift() as string;
        tableName = tableName.replace(/([A-Z])/g, '_$1');
        tableName = tableName.replace(/[^a-zA-Z0-9_]/g, '');
        tableName = tableName.replace(/^_/, '');
        tableName = tableName.toLowerCase();
        return tableName;

    }

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
            case 'MySQL':
            case 'MariaDB':
                setPort('3306');
                break;
            case 'PostgreSQL':
                setPort('5432');
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
                    db_host: dbUrl,
                    port: port,
                    username: username,
                    password: password,
                    db_name: dbName,
                    table_name: tableName,
                }
            });
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    const loadConfig = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response = await invoke('load_database_config');
            if (typeof response === "string") {
                const dbConfig = JSON.parse(response);
                setdbDriver(dbConfig.db_driver);
                setDbUrl(dbConfig.db_host);
                setPort(dbConfig.port);
                setUsername(dbConfig.username);
                setPassword(dbConfig.password);
                setDbName(dbConfig.db_name);
                setTableName(dbConfig.table_name);
            }
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };


    return (
        <div>
            <Menu handleDeconnection={handleDeconnection} saveConfig={saveConfig} loadConfig={loadConfig}/>

            {/* Card */}
            <Card>
                <CardHeader>
                    <CardContent>
                        {/* Formulaire */}
                        <form className="grid grid-cols-2 gap-4">

                            {/* Colonne gauche */}
                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm font-medium mb-1">URL de la base de données
                                        :</label>
                                    <Input
                                        type="text"
                                        value={dbUrl}
                                        onChange={(e) => setDbUrl(e.target.value)}
                                        placeholder="localhost"
                                        required
                                        className="w-full"
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium mb-1">Port :</label>
                                    <Input
                                        type="number"
                                        value={port}
                                        onChange={(e) => setPort(e.target.value)}
                                        placeholder="Port"
                                        required
                                        className="w-full"
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium mb-1">Nom d'utilisateur :</label>
                                    <Input
                                        type="text"
                                        value={username}
                                        onChange={(e) => setUsername(e.target.value)}
                                        placeholder="Username"
                                        required
                                        className="w-full"
                                    />
                                </div>
                            </div>

                            {/* Colonne droite */}
                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm font-medium mb-1">Mot de passe :</label>
                                    <Input
                                        type="password"
                                        value={password}
                                        onChange={(e) => setPassword(e.target.value)}
                                        placeholder="Password"
                                        required
                                        className="w-full"
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium mb-1">Nom de la base de données
                                        :</label>
                                    <Input
                                        type="text"
                                        value={dbName}
                                        onChange={(e) => setDbName(e.target.value)}
                                        placeholder="Database Name"
                                        required
                                        className="w-full"
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium mb-1">Nom de la table :</label>
                                    <Input
                                        type="text"
                                        value={tableName}
                                        onChange={(e) => setTableName(e.target.value)}
                                        placeholder="Table Name"
                                        required
                                        className="w-full"
                                    />
                                </div>
                            </div>

                            {/* Sélection du type de base de données et upload de fichier */}
                            <div className="col-span-2 grid grid-cols-2 gap-4 items-center justify-center">
                                {/* Sélection du type de base de données */}
                                <Select onValueChange={handledbDriverChange} value={dbDriver}>
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Type de base de données"/>
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="mysql">MySQL</SelectItem>
                                        <SelectItem value="mariadb">MariaDB</SelectItem>
                                        <SelectItem value="postgres">PostgreSQL</SelectItem>
                                    </SelectContent>
                                </Select>

                                {/* Upload de fichier et affichage du nom du fichier dans un input à côté */}
                                <div className="flex items-center gap-4">
                                    <Button onClick={openFileDialog} className="bg-blue-500 hover:bg-blue-600">
                                        <FileArchive/>
                                    </Button>
                                    <Input
                                        type="text"
                                        value={fileName ? `${fileName} (${fileSize})` : ''}
                                        placeholder="Fichier CSV"
                                        disabled
                                        className="w-full"
                                    />

                                </div>
                            </div>

                        </form>

                        {/* center the radio group  and inline the radio group items */}
                        <div className="flex justify-center mt-10">
                            <RadioGroup defaultValue="fast" className={"flex justify-center gap-10"}
                                        onChange={(e) => {
                                            // @ts-ignore
                                            setMode(e.target.value as string);
                                        }}
                            >
                                <div className="space-x-2">
                                    <RadioGroupItem value="fast" id="r1"/>
                                    <Label htmlFor="r1">Insertion rapide</Label>
                                </div>
                                <div className="space-x-2">
                                    <RadioGroupItem value="optimized" id="r2"/>
                                    <Label htmlFor="r2">Insertion optimisée</Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </CardContent>

                    {/* Boutons */}
                    <div className="flex items-center justify-center col-span-2 gap-4">
                        <Button onClick={handleSubmit}
                                type="submit"
                                className=" mx-auto bg-green-600 hover:bg-green-700"
                        >
                            <Database/>
                        </Button>

                        <Button
                            type="button"
                            onClick={handleInsert}
                            className=" mx-auto bg-blue-500 hover:bg-blue-600"
                        >
                            <Upload/>
                        </Button>

                        <Button
                            type="button"
                            onClick={handleReset}
                            className=" mx-auto bg-red-500 hover:bg-red-600"
                        >
                            <Eraser/>
                        </Button>
                    </div>

                </CardHeader>

            </Card>

            {showLoader && <div className="flex justify-center mt-4">
                <Loader/>
            </div>}

            {/* TextArea tout en bas */}
            <div className="flex flex-col mt-4 align-center">
                <Textarea disabled value={histoLog} className="w-full h-72"/>
            </div>
        </div>
    );
}

export default Home;
