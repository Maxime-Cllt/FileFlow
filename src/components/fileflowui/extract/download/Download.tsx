import React, {useEffect, useState} from 'react';
import SelectDatabaseConfig from "@/components/hooks/database/SelectDatabaseConfig.tsx";
import {initialDbConfig} from "@/components/states/initialState.tsx";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";


const Download: React.FC = () => {

    const [dbConfig, setDbConfig] = useState(initialDbConfig);

    const [configName, setConfigName] = useState<string>('');

    const [configList, setConfigList] = useState<Array<Item>>([]);

    const [error, setError] = useState<string | null>(null);
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

    function handleConnect() {


    }

    function handleDownload() {

    }

    const getAllConfigs = async () => {
        try {
            const response = await invoke('get_all_database_configs_name');
            if (typeof response === 'string') {
                const configs = JSON.parse(response);
                let configList: Array<Item> = [];
                for (let i = 0; i < configs.length; i++) {
                    configList.push({
                        id: configs[i],
                    });
                }
               setConfigList(configList);
            }
        } catch (error) {
            toast.error('Error getting all configs');
        }
    };

    useEffect(() => {
        getAllConfigs();
    }, []);

    return (
        <div className="min-h-screen flex items-center justify-center bg-gradient-to-r bg-gray-100 p-4">

            <div className="bg-white shadow-2xl rounded-lg p-8 w-full max-w-3xl">
                <h1 className="text-3xl font-bold text-center text-gray-800 mb-6">
                    Téléchargement du contenu d'une table
                </h1>
                {error && (
                    <div className="bg-red-200 text-red-700 px-4 py-2 rounded mb-4">
                        {error}
                    </div>
                )}

                {/* Section choix du mode de connexion */}
                <div className="mb-6">
                    <h2 className="text-xl font-semibold text-gray-700 mb-2">Mode de connexion</h2>
                    <div className="flex space-x-4">
                        <label className="inline-flex items-center">
                            <input
                                type="radio"
                                name="connectionMode"
                                value="predefined"
                                checked={connectionMode === 'predefined'}
                                onChange={() => setConnectionMode('predefined')}
                                className="form-radio text-blue-500"
                            />
                            <span className="ml-2">Prédéfinie</span>
                        </label>
                        <label className="inline-flex items-center">
                            <input
                                type="radio"
                                name="connectionMode"
                                value="manual"
                                checked={connectionMode === 'manual'}
                                onChange={() => setConnectionMode('manual')}
                                className="form-radio text-blue-500"
                            />
                            <span className="ml-2">Manuelle</span>
                        </label>
                    </div>
                </div>

                {/* Section de configuration */}
                {connectionMode === 'predefined' ? (
                    <div className="mb-6">
                        <label htmlFor="predefined" className="block text-gray-700 font-medium mb-2">
                            Choisir une connexion prédéfinie
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
                {/*{tables.length > 0 && (*/}
                {/*    <div className="mb-6">*/}
                {/*        <h2 className="text-xl font-semibold text-gray-700 mb-2">*/}
                {/*            Tables disponibles ({tables.length})*/}
                {/*        </h2>*/}
                {/*        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">*/}
                {/*            {tables.map((table, idx) => (*/}
                {/*                <div*/}
                {/*                    key={idx}*/}
                {/*                    onClick={() => setSelectedTable(table)}*/}
                {/*                    className={`cursor-pointer p-4 rounded shadow hover:shadow-lg transition ${*/}
                {/*                        selectedTable === table ? "bg-green-100 border-2 border-green-500" : "bg-gray-50"*/}
                {/*                    }`}*/}
                {/*                >*/}
                {/*                    {table}*/}
                {/*                </div>*/}
                {/*            ))}*/}
                {/*        </div>*/}
                {/*    </div>*/}
                {/*)}*/}

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
};

export default Download;
