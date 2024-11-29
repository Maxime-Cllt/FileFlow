import React from 'react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

interface SelectDatabaseProps {
    dbDriver: string;
    updateDbConfigField: (field: any, value: any) => void;
    updateUiStateField: (field: any, value: any) => void;
}

const databaseOptions = {
    mysql: "MySQL",
    mariadb: "MariaDB",
    postgres: "Postgres",
    sqlite: "SQLite",
};

const SelectDatabase: React.FC<SelectDatabaseProps> = (props: SelectDatabaseProps) => {

    const handledbDriverChange = (value: string) => {
        const portMap: Record<string, string> = {mysql: '3306', mariadb: '3306', postgres: '5432'};
        props.updateDbConfigField('port', portMap[value] || '');
        props.updateUiStateField('sqlite', value === 'sqlite');
        props.updateDbConfigField('dbDriver', value);
    };

    return (
        <div className="space-y-4">
            <Select onValueChange={handledbDriverChange} value={props.dbDriver}>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select a database driver"/>
                </SelectTrigger>
                <SelectContent>
                    {Object.entries(databaseOptions).map(([value, label]) => (
                        <SelectItem key={value} value={value}>
                            {label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </div>
    );
};

export default SelectDatabase;
