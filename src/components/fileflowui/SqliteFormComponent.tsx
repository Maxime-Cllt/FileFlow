import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {dialog} from "@tauri-apps/api";
import {HardDrive} from "lucide-react";
import SelectDatabaseComponent from "@/components/fileflowui/SelectDatabaseComponent.tsx";
import FileUploadComponent from "@/components/fileflowui/FileUploadComponent.tsx";

interface SqliteFormProps {
    addLog: (message: string) => void;
    sqliteFilePath: string;
    setSqliteFilePath: (value: string) => void;
    dbDriver: string;
    handledbDriverChange: (value: string) => void;
    fileName: string;
    fileSize: string;
    setFilePath: (filePath: string | null) => void;
    setFileName: (name: string) => void;
    setFileSize: (size: string) => void;
    setTableName: (tableName: string) => void;
}

const SqliteFormComponent: React.FC<SqliteFormProps> = ({
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
                                                        }) => {

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'SQLite', extensions: ['sqlite', 'db']}],
                multiple: false
            });

            if (selectedFilePath) {
                setSqliteFilePath(selectedFilePath.toString());
            }
        } catch (error) {
            addLog(`Erreur lors de la sélection de la base de données SQLite : ${error}`);
        }
    };


    return (
        <div className="flex items-center gap-6">


            <SelectDatabaseComponent handledbDriverChange={handledbDriverChange} dbDriver={dbDriver}/>


            <div className={"w-full"}>
                <div className={"flex items-center gap-4 mb-8"}>
                    <Button onClick={openFileDialog}
                            className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded">
                        <HardDrive/>
                    </Button>
                    <Input
                        type="text"
                        value={sqliteFilePath}
                        placeholder="Fichier SQLite"
                        disabled
                        className="w-full"
                    />
                </div>

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

        </div>
    );
};

export default SqliteFormComponent;
