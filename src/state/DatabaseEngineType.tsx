/**
 * DatabaseEngineType enum
 * This enum represents the different types of database engines that can be used in the application.
 */
export enum DatabaseEngineType {
    MYSQL = 'MySQL',
    MARIADB = 'MariaDB',
    POSTGRES = 'Postgres',
    SQLITE = 'SQLite',
    UNDEFINED = 'Undefined',
}

/**
 * Converts a DatabaseEngineType enum value to its string representation.
 * @param engine - The DatabaseEngineType enum value to convert.
 * @returns The string representation of the database engine type.
 */
export function databaseEngineTypeToString(engine: DatabaseEngineType): string {
    switch (engine) {
        case DatabaseEngineType.MYSQL:
            return 'MySQL';
        case DatabaseEngineType.MARIADB:
            return 'MariaDB';
        case DatabaseEngineType.POSTGRES:
            return 'Postgres';
        case DatabaseEngineType.SQLITE:
            return 'SQLite';
        case DatabaseEngineType.UNDEFINED:
            return 'Undefined';
        default:
            return DatabaseEngineType.UNDEFINED
    }
}

/**
 * Converts a string to its corresponding DatabaseEngineType enum value.
 * @param engine - The string representing the database engine type.
 * @returns The corresponding DatabaseEngineType enum value.
 * @throws Error if the string is not a valid database engine type.
 */
export function stringToDatabaseEngineType(engine: string | null | undefined): DatabaseEngineType {

    if (engine === null || engine === undefined) {
        return DatabaseEngineType.UNDEFINED;
    }

    switch (engine.trim().toLowerCase()) {
        case 'mysql':
            return DatabaseEngineType.MYSQL;
        case 'mariadb':
            return DatabaseEngineType.MARIADB;
        case 'postgres':
            return DatabaseEngineType.POSTGRES;
        case 'sqlite':
            return DatabaseEngineType.SQLITE;
        case 'undefined':
            return DatabaseEngineType.UNDEFINED;
        default:
            throw new Error(`Unsupported database engine: ${engine}`);
    }
}