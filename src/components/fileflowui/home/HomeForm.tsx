import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import SelectDatabase from "@/components/hooks/database/SelectDatabase.tsx";

interface FormProps {
    dbConfig: {
        dbUrl: string;
        port: string;
        username: string;
        password: string;
        dbName: string;
        tableName: string;
        dbDriver: string;
        is_connected: boolean;
    };
    uiState: {
        fileName: string;
    };
    setters: {
        setDbUrl: (value: string) => void;
        setPort: (value: string) => void;
        setUsername: (value: string) => void;
        setPassword: (value: string) => void;
        setDbName: (value: string) => void;
        setTableName: (value: string) => void;
        setFilePath: (filePath: string | null) => void;
        setFileName: (name: string) => void;
        setMode: (mode: string) => void;
    };
    actions: {
        addLog: (message: string) => void;
        handledbDriverChange: (value: string) => void;
    };
}


const HomeForm: React.FC<FormProps> = ({dbConfig, uiState, setters, actions}: FormProps) => {
    return (
        <form className="grid grid-cols-2 gap-4">

            {/* Left Column */}
            <div className="space-y-4">
                <FormInput label="URL of the database" value={dbConfig.dbUrl} onChange={setters.setDbUrl}
                           placeholder="localhost" required/>

                <FormInput label="Username" value={dbConfig.username} onChange={setters.setUsername}
                           placeholder="Username" required/>

                <FormInput label="Name of the database" value={dbConfig.dbName} onChange={setters.setDbName}
                           placeholder="Database Name" required/>
            </div>

            {/* Right Column */}
            <div className="space-y-4">
                <FormInput label="Port" type="number" value={dbConfig.port} onChange={setters.setPort}
                           placeholder="Port" required/>
                <FormInput label="Password" type="password" value={dbConfig.password} onChange={setters.setPassword}
                           placeholder="Password" required/>

                <FormInput label="Name of the table" value={dbConfig.tableName} onChange={setters.setTableName}
                           placeholder="Table Name" required/>
            </div>

            {/* Database Type Selection and File Upload */}
            <div className="col-span-2 grid grid-cols-2 gap-4 items-center justify-center">
                <SelectDatabase handledbDriverChange={actions.handledbDriverChange}
                                dbDriver={dbConfig.dbDriver}/>
                <FileUpload
                    setFileName={setters.setFileName}
                    setTableName={setters.setTableName}
                    addLog={actions.addLog}
                    setFilePath={setters.setFilePath}
                    fileName={uiState.fileName}
                />
            </div>

        </form>
    );
};

interface FormInputProps {
    label: string;
    type?: string;
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    required?: boolean;
}

const FormInput: React.FC<FormInputProps> = ({
                                                 label,
                                                 type = "text",
                                                 value,
                                                 onChange,
                                                 placeholder,
                                                 required
                                             }: FormInputProps) => (
    <div>
        <label className="block text-sm font-medium mb-1">{label}</label>
        <Input
            type={type}
            autoComplete="off"
            autoCorrect="off"
            value={value}
            onChange={(e) => onChange(e.target.value)}
            placeholder={placeholder}
            required={required}
            className="w-full"
        />
    </div>
);

export default HomeForm;
