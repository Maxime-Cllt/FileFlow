import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import InputTextDialog from "@/components/hooks/file/InputTextDialog.tsx";
import ConfigItemList from "@/components/fileflowui/home/ConfigItemList.tsx";

interface ButtonConfigComponentProps {
    dbConfig: {
        db_driver: string;
        db_host: string;
        port: string;
        username: string;
        password: string;
        db_name: string;
        tableName: string;
        sqlite_file_path: string;
    };
    updateDbConfigField: (field: any, value: any) => void;
    addLog: (message: string) => void;
}

const ButtonConfigComponent: React.FC<ButtonConfigComponentProps> = ({
                                                                         dbConfig,
                                                                         updateDbConfigField,
                                                                         addLog,
                                                                     }: ButtonConfigComponentProps) => {
    const [configName, setConfigName] = useState('');
    const [configNameList, setConfigNameList] = useState<Array<Item>>([]);

    const updateConfigName = (name: string) => {
        setConfigName(name);
    };

    const saveConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {

            if (configName === '' || configName === null || configName === undefined) {
                toast.error('Please enter a name for the configuration');
                return;
            }

            await invoke('save_database_config', {
                save: {
                    config_name: configName,
                    db_driver: dbConfig.db_driver.toLowerCase(),
                    db_host: dbConfig.db_host,
                    port: dbConfig.port,
                    username: dbConfig.username,
                    password: dbConfig.password,
                    db_name: dbConfig.db_name,
                    table_name: dbConfig.tableName,
                    sqlite_file_path: dbConfig.sqlite_file_path,
                },
            });
            toast.success(`Config "${configName}" saved successfully`);
        } catch (error) {
            toast.error('Error saving config');
            addLog(`Error saving config: ${error}`);
        }
    };

    const loadConfig = async (item: Item) => {
        try {
            const response = await invoke('load_database_config_by_name', {
                name: item.id,
            });

            if (typeof response === 'string') {
                const loadDbConfig = JSON.parse(response);

                Object.keys(loadDbConfig).forEach((key) => {
                    updateDbConfigField(key, loadDbConfig[key]);
                });
                updateDbConfigField('is_connected', false);

                toast.success('Config loaded successfully');
            }
        } catch (error) {
            addLog(`Error loading config: ${error}`);
            toast.error('Error loading config');
        }
    };

    const deleteConfig = async (item: Item) => {
        try {
            const response = await invoke('delete_database_config', {
                name: item.id,
            });

            if (typeof response === 'string') {
                toast.success(response);
            }
        } catch (error) {
            addLog(`Error loading config: ${error}`);
            toast.error('Error loading config');
        }
    };

    const getAllConfigs = async () => {
        try {
            const response = await invoke('get_all_database_configs_name');
            if (typeof response === 'string') {
                const configs = JSON.parse(response);
                let configList: Array<Item> = [];
                for (let i = 0; i < configs.length; i++) {
                    configList.push({
                        id: configs[i],
                    });
                }
                setConfigNameList(configList);
            }
        } catch (error) {
            addLog(`Error getting all configs: ${error}`);
            toast.error('Error getting all configs');
        }
    };

    useEffect(() => {
        getAllConfigs();
    }, []);

    return (
        <div className="flex space-x-4">
            {/* Config Item List */}
            <ConfigItemList
                onItemSelect={loadConfig}
                list={configNameList}
                onItemDelete={deleteConfig}
            />

            {/* Save Config Input Dialog */}
            <InputTextDialog
                message_text={configName}
                updateMessage={updateConfigName}
                fonction={saveConfig}
            />
        </div>
    );
};

export default ButtonConfigComponent;
