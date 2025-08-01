import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {HardDrive} from "lucide-react";
import SelectDBMS from "@/components/hooks/database/SelectDatabaseEngine.tsx";
import * as dialog from "@tauri-apps/plugin-dialog";
import {toast} from "sonner";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

interface SqliteFormProps {
    dbConfig: {
        sqliteFilePath: string;
        dbDriver: string;
    };
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}

const SqliteForm: React.FC<SqliteFormProps> = ({dbConfig, updateDbConfigField}) => {
    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: "SQLite", extensions: ["sqlite", "db", "sqlite3"]}],
                multiple: false,
            });

            if (selectedFilePath === null) {
                return;
            }

            updateDbConfigField("sqliteFilePath", selectedFilePath);
        } catch (error) {
            toast.error(error as string);
        }
    };

    return (
        <div className="p-8  space-y-8">

            <div className="flex flex-col sm:flex-row items-center gap-4">
                <Button
                    onClick={openFileDialog}
                    className="flex items-center gap-2 bg-gradient-to-r from-blue-600 to-purple-600 hover:opacity-90 text-white font-medium py-3 px-5 rounded-lg shadow-md transition duration-300"
                >
                    <HardDrive className="w-5 h-5"/>
                    Select SQLite File
                </Button>
                <Input
                    type="text"
                    value={dbConfig.sqliteFilePath}
                    placeholder="Path to SQLite file"
                    disabled
                    className="w-full border rounded-lg px-4 py-3 text-sm text-gray-700 bg-gray-100"
                />
            </div>

            <div className="flex items-center gap-4 justify-center">
                <span className="text-sm font-medium text-gray-800">Database Engine:</span>
                <SelectDBMS
                    dbDriver={dbConfig.dbDriver}
                    updateDbConfigField={updateDbConfigField}
                />
            </div>

        </div>

    );
};

export default SqliteForm;
