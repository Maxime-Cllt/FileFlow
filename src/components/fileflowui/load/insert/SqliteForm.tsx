import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {HardDrive} from "lucide-react";
import SelectDBMS from "@/components/hooks/database/SelectDatabase.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import * as dialog from "@tauri-apps/plugin-dialog";
import {toast} from "sonner";

interface SqliteFormProps {
    dbConfig: {
        sqlite_file_path: string;
        db_driver: string;
        tableName: string;
    };
    updateDbConfigField: (field: string, value: any) => void;
    updateUiStateField: (field: string, value: any) => void;
    fileName: string;
    setFileName: (value: string) => void;
}

const SqliteForm: React.FC<SqliteFormProps> = (props: SqliteFormProps) => {
    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: "SQLite", extensions: ["sqlite", "db", "sqlite3"]}],
                multiple: false,
            });

            if (selectedFilePath) {
                props.updateDbConfigField("sqlite_file_path", selectedFilePath);
            }
        } catch (error) {
            toast.error("Failed to open file");
        }
    };

    return (
        <div>

            {/* Table Name Input */}
            <div className="mb-8 flex items-center gap-4 w-full">

                <div className="w-3/4">
                    <label htmlFor="tableName" className="block text-sm font-medium text-gray-700">
                        Table Name
                    </label>
                    <Input
                        id="tableName"
                        type="text"
                        value={props.dbConfig.tableName}
                        placeholder="Table Name"
                        onChange={(e) => props.updateDbConfigField("tableName", e.target.value)}
                        className="w-full"
                    />
                </div>

                <div className="mt-4 w-1/4">
                    <SelectDBMS
                        db_driver={props.dbConfig.db_driver}
                        updateDbConfigField={props.updateDbConfigField}
                        updateUiStateField={props.updateUiStateField}
                    />
                </div>
            </div>

            {/* SQLite File Selector */}
            <div className="flex items-center gap-4 mb-8">
                <Button
                    onClick={openFileDialog}
                    className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded"
                >
                    <HardDrive/>
                </Button>
                <Input
                    type="text"
                    value={props.dbConfig.sqlite_file_path}
                    placeholder="Path to SQLite file"
                    disabled
                    className="w-full"
                />
            </div>

            {/* File Upload Component */}
            <FileUpload
                fileName={props.fileName}
                tableName={props.dbConfig.tableName}
                updateDbConfigField={props.updateDbConfigField}
                updateUiStateField={props.setFileName}
            />
        </div>
    );
};

export default SqliteForm;
