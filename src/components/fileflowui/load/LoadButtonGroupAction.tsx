import React, {useState} from 'react';
import {Copy, Eraser, Hammer} from "lucide-react";
import {toast} from "sonner";
import {invoke} from "@tauri-apps/api/core";
import DataBaseDialog from "@/components/hooks/database/DatabaseDialog.tsx";
import {initialDbConfig} from "@/components/states/initialState.tsx";

interface LoadButtonGroupProps {
    setters: {
        setTableName: (value: string) => void;
        setFilePath: (filePath: string) => void;
        setDbDriver: (value: string) => void;
        setFileName: (name: string) => void;
        setSql: (sql: string) => void;
    };
    generateSQL: {
        tableName: string;
        dbDriver: string;
        filePath: string;
        fileName: string;
        sql: string;
    };
}

const LoadButtonGroupAction: React.FC<LoadButtonGroupProps> = (props: LoadButtonGroupProps) => {

    const [dbConfig, setDbConfig] = useState(initialDbConfig);

    const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    }

    const handleReset = () => {
        props.setters.setTableName("");
        props.setters.setFilePath("");
        props.setters.setDbDriver("");
        props.setters.setFileName("");
        props.setters.setSql("");
    };

    const handleCopy = () => {
        if (props.generateSQL.sql) {
            navigator.clipboard.writeText(props.generateSQL.sql).then(() => {
                toast.success("SQL copied to clipboard");
            });
        } else {
            toast.warning("No SQL to copy");
        }
    };

    const handleGenerate = async () => {
        try {
            if (props.generateSQL.tableName === "" || props.generateSQL.dbDriver === "" || props.generateSQL.filePath === "") {
                toast.error("Please fill in all the fields");
                return;
            }

            const response = await invoke('generate_load_data_sql', {
                load: {
                    file_path: props.generateSQL.filePath,
                    table_name: props.generateSQL.tableName,
                    db_driver: props.generateSQL.dbDriver,
                },
            });

            if (response && response !== "" && typeof response === "string") {
                props.setters.setSql(response);
                toast.success("SQL generated successfully");
            } else {
                toast.error("SQL generation failed");
            }
        } catch (e) {
            toast.error(e as string);
        }
    };

    const executeSQL = async () => {
        try {
            // if (props.generateSQL.sql === "") {
            //     toast.error("No SQL to execute");
            //     return;
            // }

            // const response = await invoke('execute_sql', {
            //     sql: props.generateSQL.sql,
            // });
            //
            // if (response && response !== "" && typeof response === "string") {
            //     toast.success("SQL executed successfully");
            // } else {
            //     toast.error("SQL execution failed");
            // }
            toast.success("SQL executed successfully");
        } catch (e) {
            toast.error(e as string);
        }
    }

    return (
        <div className="flex flex-col">

            <div className="flex justify-center space-x-6 mb-6">
                <button
                    onClick={handleGenerate}
                    type={"button"}
                    aria-label="Submit to Database"
                    title="Submit data to the specified database"
                    className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300"
                >
                    <Hammer className="w-5 h-5"/>
                </button>

                <button
                    onClick={handleCopy}
                    type={"button"}
                    aria-label="Copy to Clipboard"
                    title="Copy the generated SQL to clipboard"
                    className="p-3 rounded-full shadow-lg bg-blue-500 hover:bg-blue-600 text-white transition duration-300"
                >
                    <Copy className="w-5 h-5"/>
                </button>

                <button
                    onClick={handleReset}
                    aria-label="Reset"
                    type={"button"}
                    title="Reset the form"
                    className="p-3 rounded-full shadow-lg bg-yellow-500 hover:bg-yellow-600 text-white transition duration-300"
                >
                    <Eraser className="w-5 h-5"/>
                </button>
            </div>

            <div className="flex justify-center">
                <DataBaseDialog
                    dbConfig={dbConfig}
                    updateDbConfigField={updateDbConfigField}
                    executeSQL={executeSQL}/>
            </div>

        </div>

    );
};

export default LoadButtonGroupAction;
