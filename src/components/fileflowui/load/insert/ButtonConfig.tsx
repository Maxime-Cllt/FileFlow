import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import InputTextDialog from "@/components/hooks/file/InputTextDialog.tsx";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuLabel,
    DropdownMenuSeparator
} from '@/components/ui/dropdown-menu';
import {DropdownMenuTrigger} from "@/components/ui/dropdown-menu.tsx";
import {Settings} from "lucide-react";
import {getAllConfigs, log_error} from "@/components/hooks/utils.tsx";
import ConfigItemList from "@/components/hooks/component/ConfigItemList.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

interface ButtonConfigComponentProps {
    dbConfig: DatabaseConfig;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}

const ButtonConfigComponent: React.FC<ButtonConfigComponentProps> = (props: ButtonConfigComponentProps) => {
    const [configName, setConfigName] = useState<string>('');
    const [configNameList, setConfigNameList] = useState<Array<Item>>([]);
    const [hasChanged, setHasChanged] = useState<boolean>(false);

    const updateConfigName = (name: string): void => {
        setConfigName(name);
    };

    const saveConfig = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            if (!configName) {
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
                    sqlite_file_path: props.dbConfig.sqlite_file_path,
                },
            });
            if (!response) {
                throw new Error('Error saving config');
            }
            toast.success(`Config "${configName}" saved successfully`);
            setHasChanged(prev => !prev);
        } catch (error) {
            log_error(error);
        }
    };

    const deleteConfig = async (item: Item) => {
        toast.info(`Deleting config "${item.id}"`);
    }


    useEffect(() => {
        getAllConfigs().then(
            (configs: Array<Item>) => {
                setConfigNameList(configs);
            }
        )
    }, [hasChanged]);

    return (
        <DropdownMenu>
            <DropdownMenuTrigger><Settings/></DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuLabel>Manage Configurations</DropdownMenuLabel>
                <DropdownMenuSeparator/>

                <div className="bg-white rounded-lg shadow p-6 flex flex-col items-center space-y-4">
                    <div className="flex space-x-8">

                        {/* Load Configuration Button */}
                        <div className="flex flex-col items-center">
                            <ConfigItemList
                                title="Your Configuration"
                                description=""
                                onItemSelect={function () {
                                }}
                                list={configNameList}
                                onItemDelete={deleteConfig}
                            />
                            <span className="mt-2 text-sm text-gray-600">Edit a saved config</span>
                        </div>

                        {/* Save Configuration Button */}
                        <div className="flex flex-col items-center">
                            <InputTextDialog
                                message_text={configName}
                                updateMessage={updateConfigName}
                                fonction={saveConfig}
                            />
                            <span className="mt-2 text-sm text-gray-600">Save the current config</span>
                        </div>
                    </div>
                </div>
            </DropdownMenuContent>
        </DropdownMenu>
    );
};

export default ButtonConfigComponent;
