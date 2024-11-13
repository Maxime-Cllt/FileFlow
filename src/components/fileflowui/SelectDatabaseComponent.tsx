import React from 'react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

interface SelectDatabaseProps {
    handledbDriverChange: (value: string) => void;
    dbDriver: string;
}

const databaseOptions = {
    mysql: "MySQL",
    mariadb: "MariaDB",
    postgres: "Postgres",
    sqlite: "SQLite",
};

const SelectDatabaseComponent: React.FC<SelectDatabaseProps> = (props: SelectDatabaseProps) => {
    return (
        <div className="space-y-4">
            <Select onValueChange={props.handledbDriverChange} value={props.dbDriver}>
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

export default SelectDatabaseComponent;
