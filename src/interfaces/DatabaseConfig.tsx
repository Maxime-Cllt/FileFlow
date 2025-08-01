import {DatabaseEngineType} from "@/state/DatabaseEngineType.tsx";

/**
 * Database configuration interface
 *
 * Defines the structure for database connection configuration objects
 * used throughout the application for database connectivity.
 */
export interface DatabaseConfig {
    /** Human-readable name for this database configuration */
    configName: string;

    /** Database driver type (e.g., 'mysql', 'postgresql', 'sqlite') */
    dbDriver: DatabaseEngineType;

    /** Database server hostname or IP address */
    dbHost: string;

    /** Name of the database to connect to */
    dbName: string;

    /** Database user password */
    password: string;

    /** Database server port number */
    port: string;

    /** File path for SQLite database (only used when dbDriver is 'sqlite') */
    sqliteFilePath?: string;

    /** Database username for authentication */
    username: string;

    /** Current connection status */
    isConnected: boolean;
}