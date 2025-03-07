import React, {useEffect, useState} from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Play} from "lucide-react";
import {Label} from "@/components/ui/label";
import {Input} from "@/components/ui/input";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import SelectDatabaseConfig from "@/components/hooks/database/SelectDatabaseConfig.tsx";

interface DataBaseDialogProps {
    dbConfig: {
        db_driver: string;
        db_host: string;
        port: string;
        username: string;
        password: string;
        db_name: string;
        tableName: string;
        sqlite_file_path: string;
        is_connected: boolean;
    };
    sql: string;
    updateDbConfigField: (field: any, value: any) => void;
    executeSQL: () => void;
}

const DataBaseDialog: React.FC<DataBaseDialogProps> = (props: DataBaseDialogProps) => {

    const handleInputChange = (id: string, value: string) => {
        props.updateDbConfigField(id, value);
    };

    const [configName, setConfigName] = useState('');
    const [configNameList, setConfigNameList] = useState<Array<Item>>([]);

    const loadConfig = async (config_name: string) => {
        try {
            const response = await invoke('load_database_config_by_name', {
                name: config_name,
            });
            if (typeof response === "string") {
                const loadDbConfig = JSON.parse(response);
                for (const key in loadDbConfig) {
                    props.updateDbConfigField(key, loadDbConfig[key]);
                }
                props.updateDbConfigField('is_connected', false);
            }
        } catch (error) {
            toast.error(error as string);
        }
    };

    const updateConfigName = (name: string) => {
        setConfigName(name);
        loadConfig(name);
    };

    const getAllConfigs = async () => {
        try {
            const response: string | boolean = await invoke<string | boolean>('get_all_database_configs_name');

            if (!response) {
                throw new Error('Error getting all configs');
            }

            const configs = JSON.parse(response as string);
            let configList: Array<Item> = [];
            for (let i = 0; i < configs.length; i++) {
                configList.push({
                    id: configs[i],
                });
            }
            setConfigNameList(configList);
        } catch (error) {
            toast.error('Error getting all configs');
        }
    };

    useEffect(() => {
        getAllConfigs();
    }, []);

    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    {props.sql !== "" && (
                        <button
                            aria-label="Open Database Configuration"
                            type="button"
                            title="Configure Database"
                            className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300 focus:ring-4 focus:ring-green-300"
                        >
                            <Play className="w-5 h-5"/>
                        </button>
                    )}
                </DialogTrigger>

                {/* Dialog Content */}
                <DialogContent className="sm:max-w-[700px]">
                    <DialogHeader>
                        <DialogTitle>Database Configuration</DialogTitle>
                        <DialogDescription>
                            Configure the database connection and execute your SQL query.
                        </DialogDescription>
                    </DialogHeader>

                    {/* Form Layout */}
                    <div className="flex flex-col gap-6 py-4">

                        <div>
                            <SelectDatabaseConfig
                                updateConfigName={updateConfigName}
                                configNameList={configNameList}
                                configName={configName}
                            />
                        </div>

                        {/* First Row: Username and Password */}
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <Label htmlFor="username" className="block text-sm font-medium text-gray-700">
                                    Username
                                </Label>
                                <Input
                                    id="username"
                                    type="text"
                                    value={props.dbConfig.username}
                                    onChange={(e) => handleInputChange("username", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                            <div>
                                <Label htmlFor="password" className="block text-sm font-medium text-gray-700">
                                    Password
                                </Label>
                                <Input
                                    id="password"
                                    type="password"
                                    value={props.dbConfig.password}
                                    onChange={(e) => handleInputChange("password", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Second Row: URL and Port */}
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <Label htmlFor="db_host" className="block text-sm font-medium text-gray-700">
                                    URL
                                </Label>
                                <Input
                                    id="db_host"
                                    type="text"
                                    value={props.dbConfig.db_host}
                                    onChange={(e) => handleInputChange("db_host", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                            <div>
                                <Label htmlFor="port" className="block text-sm font-medium text-gray-700">
                                    Port
                                </Label>
                                <Input
                                    id="port"
                                    type="number"
                                    value={props.dbConfig.port}
                                    onChange={(e) => handleInputChange("port", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Third Row: Database Name */}
                        <div>
                            <Label htmlFor="db_name" className="block text-sm font-medium text-gray-700">
                                Database Name
                            </Label>
                            <Input
                                id="db_name"
                                type="text"
                                value={props.dbConfig.db_name}
                                onChange={(e) => handleInputChange("db_name", e.target.value)}
                                className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                            />
                        </div>
                    </div>

                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4">
                        <div
                            onClick={props.executeSQL}
                        >
                            <DialogClose>
                                <Button
                                    className="bg-green-500 hover:bg-green-600 text-white focus:ring-4 focus:ring-green-300"
                                    type="button"
                                >
                                    Execute SQL
                                </Button>
                            </DialogClose>
                        </div>

                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default DataBaseDialog;
