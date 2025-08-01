import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";

/**
 * DatabaseFormProps interface
 * This interface defines the props for the DatabaseForm component.
 */
export interface DatabaseFormProps {
    dbConfig: DatabaseConfig;
    updateDbConfigField: (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => void;
}