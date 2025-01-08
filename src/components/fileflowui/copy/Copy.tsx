import React, {useState} from 'react';
import '../../../Loader.css';
import Log from "@/components/fileflowui/insert/Log.tsx";
import {Card, CardContent, CardHeader} from "@/components/ui/card.tsx";

const Copy: React.FC = () => {
    const [sourceDB, setSourceDB] = useState('');
    const [sourceTable, setSourceTable] = useState('');
    const [targetDB, setTargetDB] = useState('');
    const [targetTable, setTargetTable] = useState('');
    const [histoLog, setHistoLog] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    const handleCopyData = async () => {
        if (!sourceDB || !sourceTable || !targetDB || !targetTable) {
            setHistoLog('Veuillez remplir tous les champs avant de copier.');
            return;
        }

        setIsLoading(true);
        setHistoLog('Copie des données en cours...');

        try {
            // Simuler une requête asynchrone pour copier les données
            await new Promise((resolve) => setTimeout(resolve, 2000)); // Remplacer par une requête réelle

            setHistoLog(`Les données ont été copiées avec succès de ${sourceDB}.${sourceTable} à ${targetDB}.${targetTable}.`);
        } catch (error) {
            setHistoLog('Erreur lors de la copie des données : ' + error.message);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-gray-100 flex flex-col items-center">

            {/* 2 divs horizontaly */}
            <div className={"flex flex-row space-x-4"}>

                <div>

                    <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                        <h2>
                            Base de données source
                        </h2>

                        {/* Card Header with Form */}
                        <CardHeader className="border-b-2 border-gray-200 pb-4">
                            <CardContent>

                                <div className="flex flex-col space-y-4">
                                    <label>
                                        Base de données source
                                        <input
                                            type="text"
                                            value={sourceDB}
                                            onChange={(e) => setSourceDB(e.target.value)}
                                            placeholder="Nom de la base de données source"
                                            className="w-full border border-gray-300 rounded-lg p-2"
                                        />
                                    </label>

                                    <label>
                                        Table source
                                        <input
                                            type="text"
                                            value={sourceTable}
                                            onChange={(e) => setSourceTable(e.target.value)}
                                            placeholder="Nom de la table source"
                                            className="w-full border border-gray-300 rounded-lg p-2"
                                        />
                                    </label>
                                </div>

                            </CardContent>
                        </CardHeader>

                    </Card>
                </div>

                <div>

                    <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                        <h2>
                            Base de données source
                        </h2>

                        {/* Card Header with Form */}
                        <CardHeader className="border-b-2 border-gray-200 pb-4">
                            <CardContent>

                                <div className="flex flex-col space-y-4">
                                    <label>
                                        Base de données source
                                        <input
                                            type="text"
                                            value={sourceDB}
                                            onChange={(e) => setSourceDB(e.target.value)}
                                            placeholder="Nom de la base de données source"
                                            className="w-full border border-gray-300 rounded-lg p-2"
                                        />
                                    </label>

                                    <label>
                                        Table source
                                        <input
                                            type="text"
                                            value={sourceTable}
                                            onChange={(e) => setSourceTable(e.target.value)}
                                            placeholder="Nom de la table source"
                                            className="w-full border border-gray-300 rounded-lg p-2"
                                        />
                                    </label>
                                </div>

                            </CardContent>
                        </CardHeader>

                    </Card>
                </div>

            </div>

            {/* Logs Section */}
            <div className="bg-gray-50 p-4 mt-6 rounded-lg shadow-inner w-full max-w-2xl">
                <Log histoLog={histoLog}/>
            </div>
        </div>
    );
};

export default Copy;
