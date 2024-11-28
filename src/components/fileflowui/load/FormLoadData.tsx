import React from 'react';
import {Input} from "@/components/ui/input";
import FileUploadComponent from "@/components/fileflowui/style/FileUploadComponent.tsx";
import SelectDatabaseComponent from "@/components/fileflowui/home/SelectDatabaseComponent";
import LoadButtonGroupComponent from "@/components/fileflowui/load/LoadButtonGroupComponent.tsx";

interface FormLoadDataProps {
    generateSQL: {
        tableName: string;
        dbDriver: string;
        filePath: string;
        fileName: string;
        sql: string;
    };
    setters: {
        setTableName: (value: string) => void;
        setFilePath: (filePath: string) => void;
        setDbDriver: (value: string) => void;
        setFileName: (name: string) => void;
        setSql: (sql: string) => void;
    };
}

const FormLoadData: React.FC<FormLoadDataProps> = ({generateSQL, setters}) => {
    return (
        <form className="space-y-6 p-6 rounded-lg shadow-lg bg-white  mx-auto">

            {/* Table Name Input */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Table Name</label>
                <Input
                    type="text"
                    autoComplete="off"
                    autoCorrect="off"
                    value={generateSQL.tableName}
                    placeholder="Enter table name"
                    required
                    onChange={(e) => setters.setTableName(e.target.value)}
                    className="w-full rounded-lg border border-gray-300 p-2 focus:ring-2 focus:ring-green-500"
                />
            </div>

            {/* Database Selection */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Database Type</label>
                <SelectDatabaseComponent
                    handledbDriverChange={setters.setDbDriver}
                    dbDriver={generateSQL.dbDriver}
                />
            </div>

            {/* File Upload Section */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Upload File</label>
                <FileUploadComponent
                    fileName={generateSQL.fileName}
                    setFilePath={setters.setFilePath}
                    setFileName={setters.setFileName}
                    setTableName={setters.setTableName}
                    addLog={() => {
                    }}
                />
            </div>

            {/* Action Buttons */}
            <LoadButtonGroupComponent
                setters={setters}
                generateSQL={generateSQL}
            />

            {/* Textarea for Generated SQL */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Generated SQL</label>
                <textarea
                    className="w-full p-2 rounded-lg border border-gray-300 shadow-md focus:ring-2 focus:ring-blue-500"
                    rows={10}
                    value={generateSQL.sql}
                    readOnly
                    placeholder="Your generated SQL will appear here..."
                />
            </div>

        </form>
    );
};

export default FormLoadData;
