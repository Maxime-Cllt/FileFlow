import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import FileUploadComponent from "@/components/fileflowui/FileUploadComponent.tsx";
import SelectDatabaseComponent from "@/components/fileflowui/SelectDatabaseComponent.tsx";

interface FormProps {
    dbUrl: string;
    setDbUrl: (value: string) => void;
    port: string;
    setPort: (value: string) => void;
    username: string;
    setUsername: (value: string) => void;
    password: string;
    setPassword: (value: string) => void;
    dbName: string;
    setDbName: (value: string) => void;
    tableName: string;
    fileName: string;
    fileSize: string;
    setFilePath: (filePath: string | null) => void;
    setFileName: (name: string) => void;
    setFileSize: (size: string) => void;
    setTableName: (tableName: string) => void;
    addLog: (message: string) => void;

    setMode: (mode: string) => void;

    dbDriver: string;
    handledbDriverChange: (value: string) => void;
}

const FormComponent: React.FC<FormProps> = ({
                                                dbUrl, setDbUrl, port, setPort, username, setUsername,
                                                password, setPassword, dbName, setDbName, tableName, setTableName,
                                                fileName, fileSize, setFilePath, setFileName, setFileSize, addLog,
                                                dbDriver, handledbDriverChange

                                            }) => {
    return (
        <form className="grid grid-cols-2 gap-4">

            {/*Colonne de gauche */}
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

            {/*Colonne de droite */}
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
                <SelectDatabaseComponent handledbDriverChange={handledbDriverChange} dbDriver={dbDriver}/>

                {/* Upload de fichier et affichage du nom du fichier dans un input à côté */}
                <FileUploadComponent {...{
                    fileName,
                    fileSize,
                    setFilePath,
                    setFileName,
                    setFileSize,
                    setTableName,
                    addLog
                }} />
            </div>

        </form>
    );
};

export default FormComponent;
