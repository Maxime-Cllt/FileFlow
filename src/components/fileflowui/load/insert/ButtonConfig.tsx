import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import InputTextDialog from "@/components/hooks/file/InputTextDialog.tsx";
import ConfigItemList from "@/components/fileflowui/load/insert/ConfigItemList.tsx";

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
    updateUiStateField: (field: any, value: any) => void;
}

const ButtonConfigComponent: React.FC<ButtonConfigComponentProps> = (props: ButtonConfigComponentProps) => {
    const [configName, setConfigName] = useState(''); // Config name
    const [configNameList, setConfigNameList] = useState<Array<Item>>([]); // List of config names
    const [hasChanged, setHasChanged] = useState(false); // Used to refresh the config list

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

            const response: boolean = await invoke<boolean>('save_database_config', {
                save: {
                    config_name: configName,
                    db_driver: props.dbConfig.db_driver.toLowerCase(),
                    db_host: props.dbConfig.db_host,
                    port: props.dbConfig.port,
                    username: props.dbConfig.username,
                    password: props.dbConfig.password,
                    db_name: props.dbConfig.db_name,
                    table_name: props.dbConfig.tableName,
                    sqlite_file_path: props.dbConfig.sqlite_file_path,
                },
            });

            if (!response) {
                throw new Error('Error saving config');
            }

            toast.success(`Config "${configName}" saved successfully`);
            setHasChanged(prevState => !prevState);
        } catch (error) {
            toast.error(`Error saving config: ${error}`);
        }
    };

    const loadConfig = async (item: Item) => {
        try {
            const response: string | boolean = await invoke<string | boolean>('load_database_config_by_name', {
                name: item.id,
            });

            if (!response) {
                throw new Error('Error loading config');
            }
            const loadDbConfig = JSON.parse(response as string);

            Object.keys(loadDbConfig).forEach((key: string) => {
                props.updateDbConfigField(key, loadDbConfig[key]);
            });

            props.updateDbConfigField('is_connected', false);
            props.updateUiStateField('sqlite', loadDbConfig.sqlite_file_path.length > 0)
        } catch (error) {
            toast.error(error as string);
        }
    };

    const deleteConfig = async (item: Item) => {
        try {
            const response = await invoke<boolean>('delete_database_config', {name: item.id});

            if (!response) throw new Error('Error deleting config');

            toast.success('Config deleted successfully');
        } catch (error) {
            toast.error(error as string);
        }
    };


    const getAllConfigs = async () => {
        try {
            const response: string | boolean = await invoke<string | boolean>('get_all_database_configs_name');

            if (!response) {
                throw new Error('Internal error');
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
            toast.error(`Error getting all configs: ${error}`);
        }
    };

    useEffect(() => {
        getAllConfigs();
    }, [hasChanged]);

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
