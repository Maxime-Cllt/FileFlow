import React from 'react';
import {Input} from "@/components/ui/input";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import {Button} from "@/components/ui/button.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Play} from "lucide-react";
import {toast} from "sonner";

interface DataBaseDialog {
    dbConfig: {
        dbDriver: string,
        dbUrl: string,
        port: string,
        username: string,
        password: string,
        dbName: string,
        tableName: string,
        sqliteFilePath: string,
        is_connected: boolean
    };
    updateDbConfigField: (field: any, value: any) => void;
    executeSQL: () => void;
}

const DataBaseDialog: React.FC<DataBaseDialog> = (props: DataBaseDialog) => {


    const execute = () => {
        toast.success("SQL executed successfully");
        props.executeSQL();
    }

    return (
        <div>
            <Dialog>
                <DialogTrigger asChild>
                    <button
                        aria-label="Reset"
                        type={"button"}
                        title="Execute the SQL"
                        className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300">
                        <Play className="w-5 h-5"/>
                    </button>
                </DialogTrigger>
                <DialogContent className="sm:max-w-[425px]">
                    <DialogHeader>
                        <DialogTitle>Database configuration</DialogTitle>
                        <DialogDescription>
                            Configure the database connection and execute the SQL
                        </DialogDescription>
                    </DialogHeader>
                    <div className="grid gap-4 py-4">
                        <div className="grid grid-cols-4 items-center gap-4">
                            <Label htmlFor="name" className="text-right">
                                Username
                            </Label>
                            <Input
                                value={props.dbConfig.username}
                                onChange={(e) => props.updateDbConfigField('username', e.target.value)}
                                className="col-span-3"
                            />
                        </div>
                        <div className="grid grid-cols-4 items-center gap-4">
                            <Label htmlFor="username" className="text-right">
                                Password
                            </Label>
                            <Input
                                value={props.dbConfig.password}
                                onChange={(e) => props.updateDbConfigField('password', e.target.value)}
                                className="col-span-3"
                            />
                        </div>
                    </div>
                    <DialogFooter>
                        <Button
                            className="bg-green-500 hover:bg-green-600 text-white"
                            onClick={execute}
                            type="button">Execute</Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default DataBaseDialog;
