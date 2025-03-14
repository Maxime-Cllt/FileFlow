import React from 'react';
import SelectDBMS from "@/components/hooks/database/SelectDatabase.tsx";
import {Input} from "@/components/ui/input.tsx";
import SqliteForm from "@/components/hooks/database/SqliteForm.tsx";
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

export interface DatabaseFormProps {
    dbConfig: DatabaseConfig;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}

const DatabaseForm: React.FC<DatabaseFormProps> = ({dbConfig, updateDbConfigField}) => {
    const renderForm = () => {
        if (dbConfig.db_driver === 'sqlite') {
            return (
                <SqliteForm dbConfig={dbConfig} updateDbConfigField={updateDbConfigField}/>
            );
        } else {
            return (
                <form className="grid grid-cols-1 md:grid-cols-2 gap-6 p-6">

                    {/* Left Column */}
                    <div className="space-y-4">
                        <FormInput
                            label="URL of the database"
                            value={dbConfig.db_host}
                            onChange={(value: string) => updateDbConfigField('db_host', value)}
                            placeholder="localhost"
                            required
                        />
                        <FormInput
                            label="Username"
                            value={dbConfig.username}
                            onChange={(value: string) => updateDbConfigField('username', value)}
                            placeholder="Username"
                            required
                        />
                        <FormInput
                            label="Name of the database"
                            value={dbConfig.db_name}
                            onChange={(value: string) => updateDbConfigField('db_name', value)}
                            placeholder="Database Name"
                            required
                        />
                    </div>

                    {/* Right Column */}
                    <div className="space-y-4">
                        <FormInput
                            label="Port"
                            type="number"
                            value={dbConfig.port}
                            onChange={(value: string) => updateDbConfigField('port', value)}
                            placeholder="Port"
                            required
                        />
                        <FormInput
                            label="Password"
                            type="password"
                            value={dbConfig.password}
                            onChange={(value: string) => updateDbConfigField('password', value)}
                            placeholder="Password"
                            required
                        />
                    </div>

                    {/* Database Engine Selection */}
                    <div className="col-span-2">
                        <div className="flex items-center gap-4 justify-center">
                            <label className="text-sm font-medium text-gray-700">
                                Database Engine
                            </label>
                            <SelectDBMS
                                db_driver={dbConfig.db_driver}
                                updateDbConfigField={updateDbConfigField}
                            />
                        </div>
                    </div>

                </form>
            );
        }
    };

    return (
        <div>
            {renderForm()}
        </div>
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
                                                 required,
                                             }: FormInputProps) => (
    <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">{label}</label>
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

export default DatabaseForm;
