import React from 'react';
import {generateSqlConfig} from "@/components/states/generateSqlConfig.tsx";
import FormLoadData from "@/components/fileflowui/load/upload/FormLoadData.tsx";

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

            {/* Load Data Form */}
            <div className="p-4 md:p-8 mt-6">
                <FormLoadData
                    generateSQL={generateSQL}
                    updateGenerateSQL={updateGenerateSQL}
                />
            </div>

        </div>
    );
};

export default LoadDataSql;
