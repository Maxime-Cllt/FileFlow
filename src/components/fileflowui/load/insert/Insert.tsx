import React, {useState} from 'react';
import '../../../../Loader.css';
import {Card, CardContent, CardHeader} from "@/components/ui/card.tsx";
import Loader from "@/components/hooks/Loader.tsx";
import ModeSelection from "@/components/fileflowui/load/insert/ModeSelection.tsx";
import ButtonGroupAction from "@/components/fileflowui/load/insert/ButtonGroupAction.tsx";
import SqliteForm from "@/components/fileflowui/load/insert/SqliteForm.tsx";
import {initialDbConfig, initialUiState} from "@/components/states/initialState.tsx";
import ButtonConfigComponent from "@/components/fileflowui/load/insert/ButtonConfig.tsx";
import {invoke} from "@tauri-apps/api/core";
import InsertForm from "@/components/fileflowui/load/insert/InsertForm.tsx";
import {toast} from "sonner";

const Insert: React.FC = () => {
    const [dbConfig, setDbConfig] = useState(initialDbConfig);
    const [uiState, setUiState] = useState(initialUiState);

    const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    }

    const updateUiStateField = (field: keyof typeof uiState, value: any) => {
        setUiState(prev => ({...prev, [field]: value}));
    };

    const checkConnection = async () => {
        try {
            const response = await invoke<string | boolean>('is_connected');

            if (typeof response === "boolean") {
                throw Error('Failed to check connection');
            }

            if (response === '') {
                updateDbConfigField('is_connected', false);
                return;
            }
            const loadDbConfig = JSON.parse(response);
            Object.keys(loadDbConfig).forEach((key) => {
                updateDbConfigField(key as keyof typeof dbConfig, loadDbConfig[key]);
            });
            updateDbConfigField('is_connected', true);
        } catch (error) {
            toast.error(error as string);
        }
    }

    React.useEffect(() => {
            checkConnection().then();
        },
        []);

    const renderForm = () => {
        if (uiState.sqlite) {
            return (
                <SqliteForm
                    dbConfig={dbConfig}
                    fileName={uiState.fileName}
                    setFileName={(value: string) => updateUiStateField('fileName', value)}
                    updateDbConfigField={
                        (field: string, value: string) => {
                            updateDbConfigField(field as keyof typeof dbConfig, value);
                        }
                    }
                    updateUiStateField={
                        (field: string, value: string) => {
                            updateUiStateField(field as keyof typeof uiState, value);
                        }
                    }
                />
            );
        }

        return (
            <InsertForm
                dbConfig={dbConfig}
                uiState={{
                    fileName: uiState.fileName,
                }}
                updateDbConfigField={updateDbConfigField}
                updateUiStateField={updateUiStateField}
            />
        );
    };

    return (
        <div className="h-full w-full">

            {/* Main Content */}
            <div className="pt-8 px-4 md:px-8 mt-6">
                <Card className="bg-white shadow-lg rounded-lg mb-8 p-6">

                    {/* Save and Load Buttons */}
                    <div className="flex justify-end space-x-4">
                        <ButtonConfigComponent
                            dbConfig={dbConfig}
                            updateDbConfigField={updateDbConfigField}
                            updateUiStateField={
                                (field: string, value: string) => {
                                    updateUiStateField(field as keyof typeof uiState, value);
                                }
                            }
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
                            updateUiStateField={updateUiStateField}
                            updateDbConfigField={updateDbConfigField}
                        />
                    </div>
                </Card>
            </div>
        </div>
    );

};

export default Insert;
