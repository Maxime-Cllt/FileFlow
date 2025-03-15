import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

export interface DatabaseFormProps {
    dbConfig: DatabaseConfig;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}