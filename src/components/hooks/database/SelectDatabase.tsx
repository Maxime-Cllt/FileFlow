import React from 'react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

interface SelectDatabaseProps {
    db_driver: string;
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
        props.updateDbConfigField('db_driver', value);
        props.updateUiStateField('sqlite', value === 'sqlite');
    };

    return (
        <div>
            <Select onValueChange={handledbDriverChange} value={props.db_driver}>
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
