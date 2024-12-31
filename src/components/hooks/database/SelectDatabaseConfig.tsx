import React from "react";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {Label} from "@/components/ui/label.tsx";


interface SelectDatabaseDialogProps {
    updateConfigName: (name: string) => void;
    configName: string;
    configNameList: Array<Item>;
}

const SelectDatabaseDialog: React.FC<SelectDatabaseDialogProps> = (props: SelectDatabaseDialogProps) => {


    return (
        <div>
            <Label>Database Configuration</Label>
            <Select onValueChange={(value:string) => props.updateConfigName(value)} value={props.configName}>
                <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select a database configuration"/>
                </SelectTrigger>
                <SelectContent>
                    {props.configNameList.map((item: Item) => (
                        <SelectItem key={item.id} value={item.id}>
                            {item.id}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </div>
    );
};

export default SelectDatabaseDialog;
