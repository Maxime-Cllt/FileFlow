import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {HardDrive} from "lucide-react";
import SelectDatabase from "@/components/hooks/database/SelectDatabase.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import * as dialog from "@tauri-apps/plugin-dialog"

interface SqliteFormProps {
    addLog: (message: string) => void;
    sqliteFilePath: string;
    setSqliteFilePath: (value: string) => void;
    dbDriver: string;
    handledbDriverChange: (value: string) => void;
    fileName: string;
    setFilePath: (filePath: string | null) => void;
    setFileName: (name: string) => void;
    setTableName: (tableName: string) => void;
}


const SqliteForm: React.FC<SqliteFormProps> = (props: SqliteFormProps) => {

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'SQLite', extensions: ['sqlite', 'db']}],
                multiple: false
            });

            if (selectedFilePath) {
                props.setSqliteFilePath(selectedFilePath.toString());
            }
        } catch (error) {
            props.addLog(`Error opening file dialog: ${error}`);
        }
    };


    return (
        <div className="flex items-center gap-6">

            <SelectDatabase handledbDriverChange={props.handledbDriverChange} dbDriver={props.dbDriver}/>

            <div className={"w-full"}>
                <div className={"flex items-center gap-4 mb-8"}>
                    <Button onClick={openFileDialog}
                            className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded">
                        <HardDrive/>
                    </Button>
                    <Input
                        type="text"
                        value={props.sqliteFilePath}
                        placeholder="Select SQLite file"
                        disabled
                        className="w-full"
                    />
                </div>

                {/* Upload file */}
                <FileUpload {...{
                    fileName: props.fileName,
                    setFilePath: props.setFilePath,
                    setFileName: props.setFileName,
                    setTableName: props.setTableName,
                    addLog: props.addLog
                }} />
            </div>

        </div>
    );
};

export default SqliteForm;
