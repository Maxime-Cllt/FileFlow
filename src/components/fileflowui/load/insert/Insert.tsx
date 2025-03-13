import React, {useEffect, useState} from 'react';
import '../../../../Loader.css';
import Loader from "@/components/hooks/Loader.tsx";
import ModeSelection from "@/components/fileflowui/load/insert/ModeSelection.tsx";
import ButtonGroupAction from "@/components/fileflowui/load/insert/ButtonGroupAction.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import {Input} from "@/components/ui/input.tsx";
import {getNormalizedTableName} from "@/components/hooks/utils.tsx";
import ConnectionForm from "@/components/hooks/database/ConnectionForm.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";

const Insert: React.FC = () => {

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
    const [filePath, setFilePath] = useState<string>('');
    const [mode, setMode] = useState<string>('fast');
    const [showLoader, setShowLoader] = useState<boolean>(false);

    const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    };

    useEffect(() => {
        if (filePath && filePath !== "") {
            updateDbConfigField("tableName", getNormalizedTableName(filePath));
        }

    }, [filePath]);

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
                        <FileUpload filePath={filePath} setFilePath={setFilePath}/>

                        {/* Table Name Input */}

                        <div className="flex justify-center">
                            <div className="flex items-center space-x-4 w-1/2">
                                <label className="text-sm font-medium text-gray-700">Table Name</label>
                                <Input
                                    type="text"
                                    value={dbConfig.tableName}
                                    placeholder="Enter table name"
                                    onChange={(e) => updateDbConfigField('tableName', e.target.value)}
                                    className="w-2/3 border border-gray-300 rounded-lg px-4 py-3 text-gray-800"
                                />
                            </div>
                        </div>


                        <ModeSelection setMode={setMode}/>

                        <ButtonGroupAction
                            dbConfig={dbConfig}
                            updateDbConfigField={updateDbConfigField}
                            filePath={filePath}
                            setFilePath={setFilePath}
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
