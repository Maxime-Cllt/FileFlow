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


    const handledbDriverChange = (value: string) => {
        const portMap: Record<string, string> = {mysql: '3306', mariadb: '3306', postgres: '5432'};
        updateDbConfigField('dbDriver', value);
        updateDbConfigField('port', portMap[value] || '');
        updateUiStateField('sqlite', value === 'sqlite');
    };


    const renderForm = () => {
        if (uiState.sqlite) {
            return (
                <SqliteForm
                    {...{
                        addLog,
                        sqliteFilePath: dbConfig.sqliteFilePath,
                        setSqliteFilePath: (value: string) => updateDbConfigField('sqliteFilePath', value),
                        dbDriver: dbConfig.dbDriver,
                        handledbDriverChange,
                        fileName: uiState.fileName,
                        setFilePath: (value: string | null) => updateUiStateField('filePath', value),
                        setFileName: (value: string) => updateUiStateField('fileName', value),
                        setTableName: (value: string) => updateDbConfigField('tableName', value),
                    }}
                />
            );
        }

        return (
            <HomeForm
                {...{
                    dbConfig,
                    uiState,
                    setters: {
                        setDbUrl: (value: string) => updateDbConfigField('dbUrl', value),
                        setPort: (value: string) => updateDbConfigField('port', value),
                        setUsername: (value: string) => updateDbConfigField('username', value),
                        setPassword: (value: string) => updateDbConfigField('password', value),
                        setDbName: (value: string) => updateDbConfigField('dbName', value),
                        setTableName: (value: string) => updateDbConfigField('tableName', value),
                        setFilePath: (filePath: string | null) => updateUiStateField('filePath', filePath),
                        setFileName: (name: string) => updateUiStateField('fileName', name),
                        setMode: (mode: string) => updateUiStateField('mode', mode),
                    },
                    actions: {
                        addLog,
                        handledbDriverChange,
                    },
                }}
            />
        );
    };

    return (
        <div className="min-h-screen bg-gray-100">

            {/* Main Content */}
            <div className="pt-8 px-4 md:px-8">
                <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                    {/* Save and Load Buttons */}
                    <div className="flex justify-end space-x-4 mb-4">

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
