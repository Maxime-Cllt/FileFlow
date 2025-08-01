import React from 'react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";
import {DatabaseEngineType, databaseEngineTypeToString} from "@/state/DatabaseEngineType.tsx";

interface SelectDatabaseProps {
    dbDriver: string;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}

const SelectDatabaseEngine: React.FC<SelectDatabaseProps> = (props: SelectDatabaseProps) => {

    const handledbDriverChange = (value: string) => {
        const portMap: Record<string, string> = {mysql: '3306', mariadb: '3306', postgres: '5432'};
        props.updateDbConfigField('port', portMap[value] || '');
        props.updateDbConfigField('dbDriver', value);
    };

    return (
        <div>
            <Select onValueChange={handledbDriverChange} value={props.dbDriver}>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select a database driver"/>
                </SelectTrigger>
                <SelectContent>
                    {Object.entries(DatabaseEngineType).map(([key, value]) => (
                        <SelectItem key={key} value={databaseEngineTypeToString(value)}>
                            {value}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </div>
    );
};

export default SelectDatabaseEngine;
