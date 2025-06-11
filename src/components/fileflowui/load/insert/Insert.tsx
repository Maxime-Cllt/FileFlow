import React, {useCallback, useEffect, useState} from 'react';
import '../../../../Loader.css';
import Loader from "@/components/hooks/Loader.tsx";
import ModeSelection from "@/components/fileflowui/load/insert/ModeSelection.tsx";
import ButtonGroupAction from "@/components/fileflowui/load/insert/ButtonGroupAction.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import {Input} from "@/components/ui/input.tsx";
import {getNormalizedTableName} from "@/components/hooks/utils.tsx";
import ConnectionForm from "@/components/hooks/database/ConnectionForm.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";


export enum InsertionType {
    Fast = "fast",
    Optimized = "optimized",
}

const Insert: React.FC = () => {

    const [dbConfig, setDbConfig] = useState<DatabaseConfig>({
        config_name: '',
        db_driver: '',
        db_host: '',
        db_name: '',
        password: '',
        port: '',
        sqlite_file_path: '',
        username: '',
        is_connected: false
    });

    const [filesPath, setFilesPath] = useState<string[]>([]);
    const [mode, setMode] = useState<InsertionType>(InsertionType.Fast);
    const [showLoader, setShowLoader] = useState<boolean>(false);
    const [tableName, setTableName] = useState<string>('');

    const updateDbConfigField = useCallback(
        <K extends keyof DatabaseConfig>(key: K, value: DatabaseConfig[K]) => {
            setDbConfig(prev => ({...prev, [key]: value}));
        },
        []
    );

    useEffect(() => {
        updateTablesName();
    }, [filesPath]);


    const updateTablesName = (): void => {
        let tableMessage: string = '';
        for (const file of filesPath) {
            const tableName: string = getNormalizedTableName(file);
            if (file !== filesPath[filesPath.length - 1]) {
                tableMessage += `${tableName}, `;
            } else {
                tableMessage += `${tableName}`;
            }
        }
        setTableName(tableMessage);
    }

    return (
        <div className="h-full w-full">

            {/* Connection Mode Section */}
            <ConnectionForm dbConfig={dbConfig} updateDbConfigField={updateDbConfigField}/>

            {/* Loader */}
            {showLoader && (
                <div className="flex justify-center mt-6">
                    <Loader/>
                </div>
            )}

            {/* Insertion Configuration Section */}
            <div className="container mx-auto pt-8 px-4 md:px-8 space-y-6">

                {/* Insertion Configuration Section */}
                <Card>
                    <CardHeader>
                        {/* Section Header */}
                        <div className="flex items-center justify-between border-b pb-4">
                            <CardTitle
                                className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                                Insertion Configuration
                            </CardTitle>
                        </div>
                    </CardHeader>

                    <CardContent>

                        {/* File Upload */}
                        <FileUpload filesPath={filesPath} setFilePath={setFilesPath} multiple={true}/>

                        {/* Table Name Input */}

                        <div className="flex justify-center">
                            <div className="flex items-center space-x-4 w-1/2">
                                <label className="text-sm font-medium text-gray-700">Table Name</label>
                                <Input
                                    type="text"
                                    value={tableName}
                                    placeholder="Enter table name"
                                    onChange={(e) => setTableName(e.target.value)}
                                    className="w-2/3 border border-gray-300 rounded-lg px-4 py-3 text-gray-800"
                                />
                            </div>
                        </div>


                        <ModeSelection setMode={setMode}/>

                        <ButtonGroupAction
                            dbConfig={dbConfig}
                            updateDbConfigField={updateDbConfigField}
                            filesPath={filesPath}
                            setFilesPath={setFilesPath}
                            tableName={tableName}
                            setTableName={setTableName}
                            mode={mode}
                            setMode={setMode}
                            showLoader={showLoader}
                            setShowLoader={setShowLoader}
                        />

                    </CardContent>
                </Card>
            </div>

        </div>
    );
};

export default Insert;
