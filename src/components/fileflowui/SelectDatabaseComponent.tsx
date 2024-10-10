import React from 'react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";


interface SelectDatabaseProps {
    handledbDriverChange: (value: string) => void;
    dbDriver: string;
}

const SelectDatabaseComponent: React.FC<SelectDatabaseProps> = ({handledbDriverChange, dbDriver}) => {
    return (
        <div className="space-y-4">

            {/* Sélection du type de base de données */}
            <Select onValueChange={handledbDriverChange} value={dbDriver}>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="Type de base de données"/>
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="mysql">MySQL</SelectItem>
                    <SelectItem value="mariadb">MariaDB</SelectItem>
                    <SelectItem value="postgres">PostgreSQL</SelectItem>
                    <SelectItem value="sqlite">SQLite</SelectItem>
                </SelectContent>
            </Select>
        </div>
    );
};

export default SelectDatabaseComponent;
