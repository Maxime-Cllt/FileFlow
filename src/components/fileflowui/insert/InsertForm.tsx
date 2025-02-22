import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import FileUpload from "@/components/hooks/file/FileUpload.tsx";
import SelectDatabase from "@/components/hooks/database/SelectDatabase.tsx";

interface FormProps {
    dbConfig: {
        db_host: string;
        port: string;
        username: string;
        password: string;
        db_name: string;
        tableName: string;
        db_driver: string;
        is_connected: boolean;
    };
    uiState: {
        fileName: string;
    };
    updateUiStateField: (field: any, value: any) => void;
    updateDbConfigField: (field: any, value: any) => void;
    actions: {
        addLog: (message: string) => void;
    };
}


const InsertForm: React.FC<FormProps> = (props: FormProps) => {
    return (
        <form className="grid grid-cols-2 gap-4">

            {/* Left Column */}
            <div className="space-y-4">
                <FormInput label="URL of the database" value={props.dbConfig.db_host}
                           onChange={(value: string) => props.updateDbConfigField('db_host', value)}
                           placeholder="localhost" required/>

                <FormInput label="Username" value={props.dbConfig.username}
                           onChange={(value: string) => props.updateDbConfigField('username', value)}
                           placeholder="Username" required/>

                <FormInput label="Name of the database" value={props.dbConfig.db_name}
                           onChange={(value: string) => props.updateDbConfigField('db_name', value)}
                           placeholder="Database Name" required/>
            </div>

            {/* Right Column */}
            <div className="space-y-4">
                <FormInput label="Port" type="number" value={props.dbConfig.port}
                           onChange={(value: string) => props.updateDbConfigField('port', value)}
                           placeholder="Port" required/>
                <FormInput label="Password" type="password" value={props.dbConfig.password}
                           onChange={(value: string) => props.updateDbConfigField('password', value)}
                           placeholder="Password" required/>

                <FormInput label="Name of the table" value={props.dbConfig.tableName}
                           onChange={(value: string) => props.updateDbConfigField('tableName', value)}
                           placeholder="Table Name" required/>
            </div>

            {/* Database Type Selection and File Upload */}
            <div className="col-span-2 grid grid-cols-2 gap-4 items-center justify-center">
                <SelectDatabase
                    updateUiStateField={props.updateUiStateField}
                    updateDbConfigField={props.updateDbConfigField}
                    db_driver={props.dbConfig.db_driver}
                />
                <FileUpload
                    tableName={props.dbConfig.tableName}
                    fileName={props.uiState.fileName}
                    updateDbConfigField={props.updateDbConfigField}
                    updateUiStateField={props.updateUiStateField}
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

export default InsertForm;
