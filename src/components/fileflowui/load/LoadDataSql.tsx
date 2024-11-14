import React from 'react';
import FormLoadData from "@/components/fileflowui/load/FormLoadData.tsx";
import Menu from "@/components/fileflowui/home/Menu.tsx";
import {generateSqlConfig} from "@/components/object/generateSqlConfig.tsx";

const LoadDataSql: React.FC = () => {

    const [generateSQL, setGenerateSQL] = React.useState(generateSqlConfig);

    const updateGenerateSQL = (key: string, value: string) => {
        setGenerateSQL((prevGenerateSQL) => ({
            ...prevGenerateSQL,
            [key]: value,
        }));
    }

    return (
        <div className="min-h-screen bg-gray-100">

            {/* Fixed Navigation Bar */}
            <div className="fixed top-0 w-full z-50 bg-white shadow-md">
                <Menu/>
            </div>

            {/* Load Data Form */}
            <div className="pt-16 px-4 md:px-8 lg:px-16">
                <FormLoadData
                    generateSQL={generateSQL}
                    setters={{
                        setTableName: (name: string) => updateGenerateSQL('tableName', name),
                        setDbDriver: (value: string) => updateGenerateSQL('dbDriver', value),
                        setFileName: (name: string) => updateGenerateSQL('fileName', name),
                        setFilePath: (filePath: string) => updateGenerateSQL('filePath', filePath),
                        setSql: (sql: string) => updateGenerateSQL('sql', sql),
                    }}
                />
            </div>

        </div>
    );
};

export default LoadDataSql;
