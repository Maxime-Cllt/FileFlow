import React, {useState} from 'react';
import '../../../Loader.css';
import {Card, CardContent, CardHeader} from "@/components/ui/card.tsx";
import Loader from "@/components/hooks/Loader.tsx";
import HomeForm from "@/components/fileflowui/home/HomeForm.tsx";
import ModeSelection from "@/components/fileflowui/home/ModeSelection.tsx";
import ButtonGroupAction from "@/components/fileflowui/home/ButtonGroupAction.tsx";
import Log from "@/components/fileflowui/home/Log.tsx";
import SqliteForm from "@/components/fileflowui/home/SqliteForm.tsx";
import {initialDbConfig, initialUiState} from "@/components/states/initialState.tsx";
import ButtonConfigComponent from "@/components/fileflowui/home/ButtonConfig.tsx";

const Home: React.FC = () => {
    const [dbConfig, setDbConfig] = useState(initialDbConfig);
    const [uiState, setUiState] = useState(initialUiState);

    const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    }

    const updateUiStateField = (field: keyof typeof uiState, value: any) => {
        setUiState(prev => ({...prev, [field]: value}));
    };

    const addLog = (message: string) => {
        updateUiStateField('histoLog', `${uiState.histoLog}\n${message}`);
    };

    const renderForm = () => {
        if (uiState.sqlite) {
            return (
                <SqliteForm
                    dbConfig={{
                        sqliteFilePath: dbConfig.sqliteFilePath,
                        db_driver: dbConfig.db_driver,
                    }}
                    addLog={addLog}
                    updateDbConfigField={updateDbConfigField}
                    updateUiStateField={updateUiStateField}
                    uiState={{
                        fileName: uiState.fileName,
                    }}
                />
            );
        }

        return (
            <HomeForm
                dbConfig={dbConfig}
                uiState={{
                    fileName: uiState.fileName,
                }}
                actions={{
                    addLog
                }}
                updateDbConfigField={updateDbConfigField}
                updateUiStateField={updateUiStateField}
            />
        );
    };

    return (
        <div className="min-h-screen bg-gray-100">

            {/* Main Content */}
            <div className="pt-8 px-4 md:px-8 mt-6">
                <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                    {/* Save and Load Buttons */}
                    <div className="flex justify-end space-x-4">
                        <ButtonConfigComponent
                            dbConfig={dbConfig}
                            updateDbConfigField={updateDbConfigField}
                            addLog={addLog}
                        />
                    </div>

                    {/* Card Header with Form */}
                    <CardHeader className="border-b-2 border-gray-200 pb-4">
                        <CardContent>
                            {/* Render Form */}
                            {renderForm()}
                        </CardContent>
                    </CardHeader>

                    {/* Mode Selection Component */}
                    <div className="mt-6">
                        <ModeSelection setMode={(value: string) => updateUiStateField('mode', value)}/>
                    </div>

                    {/* Loader */}
                    {uiState.showLoader && (
                        <div className="flex justify-center mt-6">
                            <Loader/>
                        </div>
                    )}

                    {/* Button Group */}
                    <div className="flex justify-center mt-6">
                        <ButtonGroupAction
                            dbConfig={dbConfig}
                            uiState={uiState}
                            addLog={addLog}
                            updateUiStateField={updateUiStateField}
                            updateDbConfigField={updateDbConfigField}
                        />
                    </div>
                </Card>

                {/* Logs Section */}
                <div className="bg-gray-50 p-4 rounded-lg shadow-inner">
                    <Log histoLog={uiState.histoLog}/>
                </div>
            </div>
        </div>
    );

};

export default Home;
