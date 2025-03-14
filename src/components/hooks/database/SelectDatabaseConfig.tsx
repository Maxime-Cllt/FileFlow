import React from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {Label} from "@/components/ui/label.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";
import {loadConfig, log_error} from "@/components/hooks/utils.tsx";


interface SelectDatabaseDialogProps {
    updateConfigName: (name: string) => void;
    configName: string;
    configNameList: Array<Item>;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}


const SelectDatabaseDialog: React.FC<SelectDatabaseDialogProps> = (props) => {

    const loadSelectedConfig = async (configName: string): Promise<void> => {
        try {

            if (!configName) {
                throw new Error('Please select a configuration');
            }


            const config: string | boolean = await loadConfig(configName);

            if (typeof config !== "string") {
                throw new Error('Failed to load config');
            }

            const loadDbConfig: DatabaseConfig = JSON.parse(config) as DatabaseConfig;
            Object.keys(loadDbConfig).forEach((key) => {
                props.updateDbConfigField(key as keyof DatabaseConfig, loadDbConfig[key as keyof DatabaseConfig]);
            });
            props.updateConfigName(configName);
        } catch (error) {
            log_error(error);
        }
    }

    return (
        <div>
            <div className="grid grid-cols-[auto,1fr] gap-4 items-center">
                <Label htmlFor="export-format" className="text-sm font-medium text-gray-700">
                    Choose a database:
                </Label>

                <Select onValueChange={(value: string) => loadSelectedConfig(value)} value={props.configName}>
                    <SelectTrigger
                        className="w-full border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500">
                        <SelectValue placeholder="Select a configuration"/>
                    </SelectTrigger>
                    <SelectContent>
                        {props.configNameList.map((item) => (
                            <SelectItem key={item.id} value={item.id}>
                                {item.id}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            </div>
        </div>
    );
};

export default SelectDatabaseDialog;
