import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {HardDrive} from "lucide-react";
import SelectDatabase from "@/components/hooks/database/SelectDatabase.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import * as dialog from "@tauri-apps/plugin-dialog"

interface SqliteFormProps {
    dbConfig: {
        sqlite_file_path: string;
        db_driver: string;
    };
    uiState: {
        fileName: string;
    }
    addLog: (message: string) => void;
    updateUiStateField: (field: any, value: any) => void;
    updateDbConfigField: (field: any, value: any) => void;
}


const SqliteForm: React.FC<SqliteFormProps> = (props: SqliteFormProps) => {

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'SQLite', extensions: ['sqlite', 'db']}],
                multiple: false
            });

            if (selectedFilePath) {
                props.updateDbConfigField('sqlite_file_path', selectedFilePath);
            }
        } catch (error) {
            props.addLog(`Error opening file dialog: ${error}`);
        }
    };


    return (
        <div className="flex items-center gap-6">

            <SelectDatabase
                db_driver={props.dbConfig.db_driver}
                updateDbConfigField={props.updateDbConfigField}
                updateUiStateField={props.updateUiStateField}
            />

            <div className={"w-full"}>
                <div className={"flex items-center gap-4 mb-8"}>
                    <Button onClick={openFileDialog}
                            className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded">
                        <HardDrive/>
                    </Button>
                    <Input
                        type="text"
                        value={props.dbConfig.sqlite_file_path}
                        placeholder="Select SQLite file"
                        disabled
                        className="w-full"
                    />
                </div>

                {/* Upload file */}
                <FileUpload {...{
                    fileName: props.uiState.fileName,
                    setFilePath: (value: string | null) => props.updateUiStateField('filePath', value),
                    setFileName: (value: string) => props.updateUiStateField('fileName', value),
                    setTableName: (value: string) => props.updateDbConfigField('tableName', value),
                    addLog: props.addLog
                }} />
            </div>

        </div>
    );
};

export default SqliteForm;
