import React from 'react';
import {Eraser, Upload} from "lucide-react";
import {toast} from "sonner";
import {invoke} from "@tauri-apps/api/core";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip.tsx";
import {log_error} from "@/components/hooks/utils.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";
import {InsertionType} from "@/components/fileflowui/load/insert/Insert.tsx";

interface ButtonGroupProps {
    dbConfig: DatabaseConfig;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
    filesPath: string[];
    setFilesPath: (path: string[]) => void;
    tableName: string;
    setTableName: (name: string) => void
    mode: string;
    setMode: (mode: InsertionType) => void;
    showLoader: boolean;
    setShowLoader: (showLoader: boolean) => void;
}

const ButtonGroupAction: React.FC<ButtonGroupProps> = (props: ButtonGroupProps) => {

    const handleInsert = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            if (!props.filesPath) {
                toast.warning('Please select a file');
                return;
            }

            if (!props.dbConfig.is_connected) {
                toast.warning('Please connect to the database');
                return;
            }


            props.setShowLoader(true);

            const tableName: string[] = props.tableName.split(',').map((name) => name.trim());

            for (const [index, file] of props.filesPath.entries()) {
                const insert_csv_data_response: string = await invoke<string>('insert_csv_data', {
                    csv: {
                        table_name: tableName[index],
                        file_path: file,
                        db_driver: props.dbConfig.db_driver.toLowerCase(),
                        mode: props.mode,
                    },
                });

                if (insert_csv_data_response.startsWith("Error:")) {
                    log_error(insert_csv_data_response);
                }
                toast.success(insert_csv_data_response);
            }

        } catch (error) {
            log_error(error);
        }
        props.setShowLoader(false);
    };

    const handleReset = (): void => {
        props.updateDbConfigField('db_driver', '');
        props.updateDbConfigField('db_host', '');
        props.updateDbConfigField('port', '');
        props.updateDbConfigField('username', '');
        props.updateDbConfigField('password', '');
        props.updateDbConfigField('db_name', '');
        props.updateDbConfigField('sqlite_file_path', '');

        props.setMode(InsertionType.Fast);
        props.setFilesPath([]);
        props.setTableName('');
        props.setShowLoader(false);
    };

    const insertOk: boolean = props.filesPath.length > 0 && props.dbConfig.is_connected;

    return (
        <div className="flex items-center justify-center gap-x-6 mt-4 p-4">

            {/* Insert Data Button */}
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger>
                        <button
                            onClick={handleInsert}
                            disabled={!insertOk}
                            className={`flex items-center justify-center p-3 rounded-full shadow-md transition duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                                ${
                                insertOk
                                    ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white hover:opacity-90"
                                    : "bg-gray-400 text-gray-700 cursor-not-allowed"
                            }`}
                        >
                            <Upload className="w-6 h-6"/>
                        </button>
                    </TooltipTrigger>
                    <TooltipContent>
                        {props.dbConfig.is_connected ? "Start insertion" : "Connect first"}
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>


            {/* Reset Button */}
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger>
                        <button
                            onClick={handleReset}
                            className="flex items-center justify-center bg-yellow-500 hover:bg-yellow-600 text-white p-3 rounded-full shadow-md transition duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500"
                            title="Reset form"
                        >
                            <Eraser className="w-6 h-6"/>
                        </button>
                    </TooltipTrigger>
                    <TooltipContent>
                        Reset form
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>

        </div>
    );
};

export default ButtonGroupAction;
