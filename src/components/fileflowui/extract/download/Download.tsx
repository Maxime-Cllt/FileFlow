import React, {useEffect, useState} from 'react';
import SelectDatabaseConfig from "@/components/hooks/database/SelectDatabaseConfig.tsx";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group.tsx";
import {Label} from "@/components/ui/label.tsx";
import {ComboboxComponent} from "@/components/hooks/ComboboxComponent.tsx";


const Download: React.FC = () => {

        const [dbConfig, setDbConfig] = useState({
            db_driver: '',
            db_host: '',
            port: '',
            username: '',
            password: '',
            db_name: '',
            tableName: '',
            sqlite_file_path: '',
            is_connected: false
        });

        const [configName, setConfigName] = useState<string>('');

        const [configList, setConfigList] = useState<Array<Item>>([]);

        const [tables, setTables] = useState<Array<ComboItem>>([]);

        const [loadingTables, setLoadingTables] = useState<boolean>(false);
        const [loadingDownload, setLoadingDownload] = useState<boolean>(false);
        const [connectionMode, setConnectionMode] = useState<'predefined' | 'manual'>('predefined');
        const [selectedTable, setSelectedTable] = useState<string | null>(null);

        const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
            setDbConfig(prev => ({...prev, [field]: value}));
        }


        const updateConfigName = (name: string) => {
            setConfigName(name);
        }

        const handleConnect = async () => {
            try {

                // Check if user is already connected
                const is_connected_response = await invoke<string | boolean>('is_connected');

                if (typeof is_connected_response === "boolean") {
                    throw Error('Failed to check connection');
                }

                // If user is already connected, disconnect from current connection
                if (is_connected_response !== '') {
                    const response = await invoke<boolean | string>('disconnect_from_database');
                    if (typeof response === 'string') {
                        throw Error(response as string)
                    }
                }

                // Load config if connection mode is predefined
                if (connectionMode === 'predefined') {

                    const config_response = await invoke<string | boolean>('load_database_config_by_name', {
                        name: configName,
                    });

                    if (typeof config_response === "boolean") {
                        throw Error('Failed to load config');
                    }

                    const loadDbConfig = JSON.parse(config_response);
                    const updatedDbConfig = {...dbConfig, ...loadDbConfig};
                    setDbConfig(updatedDbConfig);
                }

                const connect_to_database_response = await invoke<string | boolean>('connect_to_database', {
                    config: {
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

                if (typeof connect_to_database_response === "string") {
                    throw Error("Failed to connect to database");
                }

                updateDbConfigField('is_connected', true);


                const get_table_list_response = await invoke<Array<ComboItem> | boolean>('get_table_list');

                if (typeof get_table_list_response === "boolean") {
                    throw Error('Failed to get table list');
                }

                const parsedData: ComboItem[] = get_table_list_response.map(item => ({
                    value: item.value,
                    label: item.label
                }));

                setTables(parsedData);
            } catch (error) {
                toast.error(error as string);
            }
        }


        function handleDownload() {

        }

        const getAllConfigs = async () => {
            try {
                const response: string | boolean = await invoke<string | boolean>('get_all_database_configs_name');

                if (!response) {
                    throw new Error('Error getting all configs');
                }

                const configs = JSON.parse(response as string);
                let configList: Array<Item> = [];
                for (let i = 0; i < configs.length; i++) {
                    configList.push({
                        id: configs[i],
                    });
                }
                setConfigList(configList);
            } catch
                (error) {
                toast.error('Error getting all configs');
            }
        };

        useEffect(() => {
            getAllConfigs();
        }, []);

        return (
            <div className="h-full w-full flex items-center justify-center p-4">
                <div className="bg-white shadow-2xl rounded-lg p-8 w-full max-w-3xl">

                    {/* Section choix du mode de connexion */}
                    <div className="mb-6">
                        <h2 className="text-xl font-semibold text-gray-700 mb-2">Connection mode</h2>
                        <div className="flex space-x-4 justify-center">
                            <RadioGroup defaultValue="predefined" className="flex justify-center gap-10"
                                        onValueChange={(e: string): void => {
                                            if (e === "manual" || e === "predefined") {
                                                setConnectionMode(e);
                                            }
                                        }
                                        }>
                                <div className="space-x-2">
                                    <RadioGroupItem value="predefined" id="r1"/>
                                    <Label htmlFor="r1">Saved configurations</Label>
                                </div>
                                <div className="space-x-2">
                                    <RadioGroupItem value="manual" id="r2"/>
                                    <Label htmlFor="r2">Manual</Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>

                    {/* Section de configuration */}
                    {connectionMode === 'predefined' ? (
                        <div className="mb-6">
                            <label htmlFor="predefined" className="block text-gray-700 font-medium mb-2">
                                Choose a predefined configuration
                            </label>
                            <SelectDatabaseConfig
                                updateConfigName={updateConfigName}
                                configName={configName}
                                configNameList={configList}
                            />
                        </div>

                    ) : (
                        <div className="mb-6 grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label htmlFor="host" className="block text-gray-700 font-medium mb-2">
                                    Host
                                </label>
                                <input
                                    id="host"
                                    type="text"
                                    value={dbConfig.db_host}
                                    onChange={(e) => updateDbConfigField('db_host', e.target.value)}
                                    placeholder="ex: localhost"
                                    className="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                                />
                            </div>
                            <div>
                                <label htmlFor="port" className="block text-gray-700 font-medium mb-2">
                                    Port
                                </label>
                                <input
                                    id="port"
                                    type="number"
                                    value={dbConfig.port || ''}
                                    onChange={(e) => updateDbConfigField('port', e.target.value)}
                                    placeholder="ex: 5432"
                                    className="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                                />
                            </div>
                            <div>
                                <label htmlFor="database" className="block text-gray-700 font-medium mb-2">
                                    Database
                                </label>
                                <input
                                    id="database"
                                    type="text"
                                    value={dbConfig.db_name}
                                    onChange={(e) => updateDbConfigField('db_name', e.target.value)}
                                    placeholder="Nom de la base"
                                    className="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                                />
                            </div>
                            <div>
                                <label htmlFor="user" className="block text-gray-700 font-medium mb-2">
                                    User
                                </label>
                                <input
                                    id="user"
                                    type="text"
                                    value={dbConfig.username}
                                    onChange={(e) => updateDbConfigField('username', e.target.value)}
                                    placeholder="Nom d'utilisateur"
                                    className="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                                />
                            </div>
                            <div className="md:col-span-2">
                                <label htmlFor="password" className="block text-gray-700 font-medium mb-2">
                                    Password
                                </label>
                                <input
                                    id="password"
                                    type="password"
                                    value={dbConfig.password}
                                    onChange={(e) => updateDbConfigField('password', e.target.value)}
                                    placeholder="Mot de passe"
                                    className="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                                />
                            </div>
                        </div>
                    )}

                    {/* Bouton pour se connecter et charger les tables */}
                    <div className="flex justify-center mb-6">
                        <button
                            onClick={handleConnect}
                            disabled={loadingTables}
                            className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-6 rounded focus:outline-none focus:ring"
                        >
                            {loadingTables ? "Connexion en cours..." : "Se connecter"}
                        </button>
                    </div>

                    {/* Affichage des tables récupérées */}
                    {tables.length > 0 && (
                        <div className="mb-6">
                            <h2 className="text-xl font-semibold text-gray-700 mb-2 text-center">
                                Tables available ({tables.length})
                            </h2>
                            <div className="flex justify-center">
                                <ComboboxComponent lists={tables}/>
                            </div>
                        </div>
                    )}

                    {/* Bouton de téléchargement */}
                    <div className="flex justify-center">
                        <button
                            onClick={handleDownload}
                            disabled={loadingDownload || !selectedTable}
                            className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-6 rounded focus:outline-none focus:ring"
                        >
                            {loadingDownload ? "Téléchargement en cours..." : "Télécharger la table"}
                        </button>
                    </div>

                </div>
            </div>
        );
    }
;

export default Download;
