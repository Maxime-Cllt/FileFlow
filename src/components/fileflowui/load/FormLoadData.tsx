import React from 'react';
import {Input} from "@/components/ui/input";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import SelectDatabase from "@/components/hooks/database/SelectDatabase.tsx";
import LoadButtonGroupAction from "@/components/fileflowui/load/LoadButtonGroupAction.tsx";

interface FormLoadDataProps {
    generateSQL: {
        tableName: string;
        db_driver: string;
        filePath: string;
        fileName: string;
        sql: string;
    };
    updateGenerateSQL: (key: string, value: string) => void;
}

const FormLoadData: React.FC<FormLoadDataProps> = ({generateSQL, updateGenerateSQL}) => {
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
                    onChange={(e) => updateGenerateSQL('tableName', e.target.value)}
                    className="w-full rounded-lg border border-gray-300 p-2 focus:ring-2 focus:ring-green-500"
                />
            </div>

            {/* Database Selection */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Database Type</label>
                <SelectDatabase
                    db_driver={generateSQL.db_driver}
                    updateDbConfigField={
                        (field, value) => {
                            updateGenerateSQL(field, value);
                        }
                    }
                    updateUiStateField={() => {
                    }}
                />
            </div>

            {/* File Upload Section */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Upload File</label>
                <FileUpload
                    fileName={generateSQL.fileName}
                    setFilePath={(value) => updateGenerateSQL('filePath', value)}
                    setFileName={(value) => updateGenerateSQL('fileName', value)}
                    setTableName={(value) => updateGenerateSQL('tableName', value)}
                    addLog={() => {
                    }}
                />
            </div>

            {/* Action Buttons */}
            <LoadButtonGroupAction
                generateSQL={generateSQL}
                updateGenerateSQL={updateGenerateSQL}
            />

            {/* Textarea for Generated SQL */}
            <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">Generated SQL</label>
                <textarea
                    className="w-full p-2 rounded-lg border border-gray-300 shadow-md focus:ring-2 focus:ring-blue-500"
                    rows={10}
                    value={generateSQL.sql}
                    onChange={(e) => updateGenerateSQL('sql', e.target.value)}
                    placeholder="Your generated SQL will appear here..."
                />
            </div>

        </form>
    );
};

export default FormLoadData;
