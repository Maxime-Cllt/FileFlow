import React from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Play} from "lucide-react";
import {Label} from "@/components/ui/label";
import {Input} from "@/components/ui/input";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

interface DataBaseDialogProps {
    dbConfig: DatabaseConfig;
    sql: string;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
    executeSQL: () => void;
}

const DataBaseDialog: React.FC<DataBaseDialogProps> = (props: DataBaseDialogProps) => {

    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    {props.sql !== "" && (
                        <button
                            aria-label="Open Database Configuration"
                            type="button"
                            title="Configure Database"
                            className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300 focus:ring-4 focus:ring-green-300"
                        >
                            <Play className="w-5 h-5"/>
                        </button>
                    )}
                </DialogTrigger>

                {/* Dialog Content */}
                <DialogContent className="sm:max-w-[700px]">
                    <DialogHeader>
                        <DialogTitle>Database Configuration</DialogTitle>
                        <DialogDescription>
                            Configure the database connection and execute your SQL query.
                        </DialogDescription>
                    </DialogHeader>

                    {/* Form Layout */}
                    <div className="flex flex-col gap-6 py-4">


                        {/* First Row: Username and Password */}
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <Label htmlFor="username" className="block text-sm font-medium text-gray-700">
                                    Username
                                </Label>
                                <Input
                                    id="username"
                                    type="text"
                                    value={props.dbConfig.username}
                                    onChange={(e) => props.updateDbConfigField("username", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                            <div>
                                <Label htmlFor="password" className="block text-sm font-medium text-gray-700">
                                    Password
                                </Label>
                                <Input
                                    id="password"
                                    type="password"
                                    value={props.dbConfig.password}
                                    onChange={(e) => props.updateDbConfigField("password", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Second Row: URL and Port */}
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <Label htmlFor="db_host" className="block text-sm font-medium text-gray-700">
                                    URL
                                </Label>
                                <Input
                                    id="db_host"
                                    type="text"
                                    value={props.dbConfig.db_host}
                                    onChange={(e) => props.updateDbConfigField("db_host", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                            <div>
                                <Label htmlFor="port" className="block text-sm font-medium text-gray-700">
                                    Port
                                </Label>
                                <Input
                                    id="port"
                                    type="number"
                                    value={props.dbConfig.port}
                                    onChange={(e) => props.updateDbConfigField("port", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Third Row: Database Name */}
                        <div>
                            <Label htmlFor="db_name" className="block text-sm font-medium text-gray-700">
                                Database Name
                            </Label>
                            <Input
                                id="db_name"
                                type="text"
                                value={props.dbConfig.db_name}
                                onChange={(e) => props.updateDbConfigField("db_name", e.target.value)}
                                className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                            />
                        </div>
                    </div>

                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4">
                        <div
                            onClick={props.executeSQL}
                        >
                            <DialogClose>
                                <Button
                                    className="bg-green-500 hover:bg-green-600 text-white focus:ring-4 focus:ring-green-300"
                                    type="button"
                                >
                                    Execute SQL
                                </Button>
                            </DialogClose>
                        </div>

                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default DataBaseDialog;
