import {DatabaseEngineType} from "@/state/DatabaseEngineType.tsx";

/**
 * Database configuration interface
 *
 * Defines the structure for database connection configuration objects
 * used throughout the application for database connectivity.
 */
export interface DatabaseConfig {
    /** Human-readable name for this database configuration */
    config_name: string;

    /** Database driver type (e.g., 'mysql', 'postgresql', 'sqlite') */
    db_driver: DatabaseEngineType;

    /** Database server hostname or IP address */
    db_host: string;

    /** Name of the database to connect to */
    db_name: string;

    /** Database user password */
    password: string;

    /** Database server port number */
    port: string;

    /** File path for SQLite database (only used when db_driver is 'sqlite') */
    sqlite_file_path?: string;

    /** Database username for authentication */
    username: string;

    /** Current connection status */
    isConnected: boolean;
}