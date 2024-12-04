import React, {useEffect, useState} from 'react';
import {Copy, Eraser, Hammer} from "lucide-react";
import {toast} from "sonner";
import {invoke} from "@tauri-apps/api/core";
import DataBaseDialog from "@/components/hooks/database/DatabaseDialog.tsx";
import {initialDbConfig} from "@/components/states/initialState.tsx";

interface LoadButtonGroupProps {
    generateSQL: {
        tableName: string;
        db_driver: string;
        filePath: string;
        fileName: string;
        sql: string;
    };
    updateGenerateSQL: (key: string, value: string) => void;
}

const LoadButtonGroupAction: React.FC<LoadButtonGroupProps> = (props: LoadButtonGroupProps) => {

    const [dbConfig, setDbConfig] = useState(initialDbConfig);

    useEffect(() => {
        loadConfig().then();
    }, []);

    const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
        setDbConfig(prev => ({...prev, [field]: value}));
    }

    const loadConfig = async () => {
        try {
            const response = await invoke('load_database_config');
            if (typeof response === "string") {
                const loadDbConfig = JSON.parse(response);
                for (const key in loadDbConfig) {
                    updateDbConfigField(key as keyof typeof dbConfig, loadDbConfig[key]);
                }
                updateDbConfigField('is_connected', false);
                props.updateGenerateSQL("db_driver", loadDbConfig.db_driver);
            }
        } catch (error) {
            toast.error(error as string);
        }
    };

    const handleReset = () => {
        props.updateGenerateSQL("tableName", "");
        props.updateGenerateSQL("filePath", "");
        props.updateGenerateSQL("db_driver", "");
        props.updateGenerateSQL("fileName", "");
        props.updateGenerateSQL("sql", "");
    }

    const handleCopy = () => {
        if (props.generateSQL.sql !== "") {
            navigator.clipboard.writeText(props.generateSQL.sql).then(() => {
                toast.success("SQL copied to clipboard");
            });
        } else {
            toast.warning("No SQL to copy");
        }
    };

    const handleGenerate = async () => {
        try {
            if (props.generateSQL.tableName === "" || props.generateSQL.db_driver === "" || props.generateSQL.filePath === "") {
                toast.error("Please fill in all the fields");
                return;
            }

            const response = await invoke('generate_load_data_sql', {
                load: {
                    file_path: props.generateSQL.filePath,
                    table_name: props.generateSQL.tableName,
                    db_driver: props.generateSQL.db_driver,
                },
            });

            if (response && response !== "" && typeof response === "string") {
                props.updateGenerateSQL("sql", response);
                toast.success("SQL generated successfully");
            } else {
                toast.error("SQL generation failed");
            }
        } catch (e) {
            toast.error(e as string);
        }
    };

    const checkConnection = async () => {
        try {
            if (props.generateSQL.sql === "") {
                toast.error("No SQL to execute");
                return;
            }

            if (!dbConfig.is_connected) {
                const response = await invoke('connect_to_database', {
                        config: {
                            db_driver: props.generateSQL.db_driver.toLowerCase(),
                            db_host: dbConfig.db_host,
                            port: dbConfig.port,
                            username: dbConfig.username,
                            password: dbConfig.password,
                            db_name: dbConfig.db_name,
                            table_name: dbConfig.tableName,
                            sqlite_file_path: dbConfig.sqliteFilePath,
                        },
                    }
                );

                if (response && response !== "" && typeof response === "string") {
                    updateDbConfigField('is_connected', true);
                    executeSQL().then(() => {
                        toast.success("SQL executed successfully");
                    }).catch((e) => {
                            toast.error(e as string);
                        }
                    );
                } else {
                    toast.error("SQL execution failed");
                }
            } else {
                executeSQL().then(() => {
                    toast.success("SQL executed successfully");
                }).catch((e) => {
                        toast.error(e as string);
                    }
                );
            }
        } catch (e) {
            toast.error(e as string);
        }
    }

    const executeSQL = async () => {
        const response = await invoke('execute_sql', {
            sql: props.generateSQL.sql.trim(),
        });

        if (response && response !== "" && typeof response === "string") {
            toast.success("SQL executed successfully");
        } else {
            toast.error("SQL execution failed");
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
                    sql={props.generateSQL.sql}
                    executeSQL={checkConnection}/>
            </div>

        </div>

    );
};

export default LoadButtonGroupAction;
