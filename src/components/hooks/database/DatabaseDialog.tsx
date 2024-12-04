import React from "react";
import {
    Dialog,
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

interface DataBaseDialogProps {
    dbConfig: {
        dbDriver: string;
        dbUrl: string;
        port: string;
        username: string;
        password: string;
        dbName: string;
        tableName: string;
        sqliteFilePath: string;
        is_connected: boolean;
    };
    updateDbConfigField: (field: any, value: string) => void;
    executeSQL: () => void;
}

const DataBaseDialog: React.FC<DataBaseDialogProps> = ({
                                                           dbConfig,
                                                           updateDbConfigField,
                                                           executeSQL,
                                                       }) => {

    const handleInputChange = (id: string, value: string) => {
        updateDbConfigField(id, value);
    };

    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    <button
                        aria-label="Open Database Configuration"
                        type="button"
                        title="Configure Database"
                        className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300 focus:ring-4 focus:ring-green-300"
                    >
                        <Play className="w-5 h-5"/>
                    </button>
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
                                    value={dbConfig.username}
                                    onChange={(e) => handleInputChange("username", e.target.value)}
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
                                    value={dbConfig.password}
                                    onChange={(e) => handleInputChange("password", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Second Row: URL and Port */}
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <Label htmlFor="dbUrl" className="block text-sm font-medium text-gray-700">
                                    URL
                                </Label>
                                <Input
                                    id="dbUrl"
                                    type="text"
                                    value={dbConfig.dbUrl}
                                    onChange={(e) => handleInputChange("dbUrl", e.target.value)}
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
                                    value={dbConfig.port}
                                    onChange={(e) => handleInputChange("port", e.target.value)}
                                    className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                                />
                            </div>
                        </div>

                        {/* Third Row: Database Name */}
                        <div>
                            <Label htmlFor="dbName" className="block text-sm font-medium text-gray-700">
                                Database Name
                            </Label>
                            <Input
                                id="dbName"
                                type="text"
                                value={dbConfig.dbName}
                                onChange={(e) => handleInputChange("dbName", e.target.value)}
                                className="w-full border rounded-md p-2 shadow-sm focus:ring-purple-300 focus:border-purple-500"
                            />
                        </div>
                    </div>

                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4">
                        <Button
                            className="bg-green-500 hover:bg-green-600 text-white focus:ring-4 focus:ring-green-300"
                            onClick={executeSQL}
                            type="button"
                        >
                            Execute SQL
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default DataBaseDialog;
