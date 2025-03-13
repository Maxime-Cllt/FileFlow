import React from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {Label} from "@/components/ui/label.tsx";


interface SelectDatabaseDialogProps {
    updateConfigName: (name: string) => void;
    configName: string;
    configNameList: Array<Item>;
}


const SelectDatabaseDialog: React.FC<SelectDatabaseDialogProps> = (props) => {
    return (
        <div>
            <div className="grid grid-cols-[auto,1fr] gap-4 items-center">
                <Label htmlFor="export-format" className="text-sm font-medium text-gray-700">
                    Choose a database:
                </Label>

                <Select onValueChange={(value: string) => props.updateConfigName(value)} value={props.configName}>
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
