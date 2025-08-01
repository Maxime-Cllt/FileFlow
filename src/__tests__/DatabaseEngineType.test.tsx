import { DatabaseEngineType, databaseEngineTypeToString, stringToDatabaseEngineType } from "../state/DatabaseEngineType.tsx";

describe('DatabaseEngineType Enum', () => {
    describe('Enum Values', () => {
        it('should have correct string values for each database engine type', () => {
            expect(DatabaseEngineType.MYSQL).toBe('MySQL');
            expect(DatabaseEngineType.MARIADB).toBe('MariaDB');
            expect(DatabaseEngineType.POSTGRES).toBe('Postgres');
            expect(DatabaseEngineType.SQLITE).toBe('SQLite');
            expect(DatabaseEngineType.UNDEFINED).toBe('Undefined');
        });

        it('should have exactly 5 database engine types', () => {
            const engineCount = Object.keys(DatabaseEngineType).length;
            expect(engineCount).toBe(5);
        });

        it('should contain all expected database engine types', () => {
            const expectedTypes = ['MYSQL', 'MARIADB', 'POSTGRES', 'SQLITE', 'UNDEFINED'];
            const actualTypes = Object.keys(DatabaseEngineType);
            expect(actualTypes).toEqual(expect.arrayContaining(expectedTypes));
        });

        it('should have unique values for each enum member', () => {
            const values = Object.values(DatabaseEngineType);
            const uniqueValues = new Set(values);
            expect(uniqueValues.size).toBe(values.length);
        });
    });
});

describe('databaseEngineTypeToString', () => {
    describe('Valid Database Engine Types', () => {
        it('should convert MYSQL to "MySQL"', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.MYSQL);
            expect(result).toBe('MySQL');
        });

        it('should convert MARIADB to "MariaDB"', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.MARIADB);
            expect(result).toBe('MariaDB');
        });

        it('should convert POSTGRES to "Postgres"', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.POSTGRES);
            expect(result).toBe('Postgres');
        });

        it('should convert SQLITE to "SQLite"', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.SQLITE);
            expect(result).toBe('SQLite');
        });

        it('should convert UNDEFINED to "Undefined"', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.UNDEFINED);
            expect(result).toBe('Undefined');
        });
    });

    describe('Invalid Database Engine Types', () => {
        it('should return "Undefined" for invalid engine type', () => {
            const invalidEngine = 'invalid' as DatabaseEngineType;
            const result = databaseEngineTypeToString(invalidEngine);
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });

        it('should return "Undefined" for null value', () => {
            const result = databaseEngineTypeToString(null as any);
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });

        it('should return "Undefined" for undefined value', () => {
            const result = databaseEngineTypeToString(undefined as any);
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });

        it('should return "Undefined" for empty string', () => {
            const result = databaseEngineTypeToString('' as any);
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });
    });

    describe('Type Safety', () => {
        it('should return string type', () => {
            const result = databaseEngineTypeToString(DatabaseEngineType.MYSQL);
            expect(typeof result).toBe('string');
        });

        it('should handle all enum values without compilation errors', () => {
            Object.values(DatabaseEngineType).forEach(engine => {
                expect(() => databaseEngineTypeToString(engine)).not.toThrow();
            });
        });

        it('should return the same value as the enum value for all valid engines', () => {
            Object.values(DatabaseEngineType).forEach(engine => {
                const result = databaseEngineTypeToString(engine);
                expect(result).toBe(engine);
            });
        });
    });
});

describe('stringToDatabaseEngineType', () => {
    describe('Valid Strings - Lowercase', () => {
        it('should convert "mysql" to DatabaseEngineType.MYSQL', () => {
            const result = stringToDatabaseEngineType('mysql');
            expect(result).toBe(DatabaseEngineType.MYSQL);
        });

        it('should convert "mariadb" to DatabaseEngineType.MARIADB', () => {
            const result = stringToDatabaseEngineType('mariadb');
            expect(result).toBe(DatabaseEngineType.MARIADB);
        });

        it('should convert "postgres" to DatabaseEngineType.POSTGRES', () => {
            const result = stringToDatabaseEngineType('postgres');
            expect(result).toBe(DatabaseEngineType.POSTGRES);
        });

        it('should convert "sqlite" to DatabaseEngineType.SQLITE', () => {
            const result = stringToDatabaseEngineType('sqlite');
            expect(result).toBe(DatabaseEngineType.SQLITE);
        });

        it('should convert "undefined" to DatabaseEngineType.UNDEFINED', () => {
            const result = stringToDatabaseEngineType('undefined');
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });
    });

    describe('Valid Strings - Mixed Case', () => {
        it('should convert "MySQL" to DatabaseEngineType.MYSQL', () => {
            const result = stringToDatabaseEngineType('MySQL');
            expect(result).toBe(DatabaseEngineType.MYSQL);
        });

        it('should convert "MYSQL" to DatabaseEngineType.MYSQL', () => {
            const result = stringToDatabaseEngineType('MYSQL');
            expect(result).toBe(DatabaseEngineType.MYSQL);
        });

        it('should convert "MariaDB" to DatabaseEngineType.MARIADB', () => {
            const result = stringToDatabaseEngineType('MariaDB');
            expect(result).toBe(DatabaseEngineType.MARIADB);
        });

        it('should convert "MARIADB" to DatabaseEngineType.MARIADB', () => {
            const result = stringToDatabaseEngineType('MARIADB');
            expect(result).toBe(DatabaseEngineType.MARIADB);
        });

        it('should convert "PostgreS" to DatabaseEngineType.POSTGRES', () => {
            const result = stringToDatabaseEngineType('PostgreS');
            expect(result).toBe(DatabaseEngineType.POSTGRES);
        });

        it('should convert "POSTGRES" to DatabaseEngineType.POSTGRES', () => {
            const result = stringToDatabaseEngineType('POSTGRES');
            expect(result).toBe(DatabaseEngineType.POSTGRES);
        });

        it('should convert "SQLite" to DatabaseEngineType.SQLITE', () => {
            const result = stringToDatabaseEngineType('SQLite');
            expect(result).toBe(DatabaseEngineType.SQLITE);
        });

        it('should convert "SQLITE" to DatabaseEngineType.SQLITE', () => {
            const result = stringToDatabaseEngineType('SQLITE');
            expect(result).toBe(DatabaseEngineType.SQLITE);
        });

        it('should convert "UNDEFINED" to DatabaseEngineType.UNDEFINED', () => {
            const result = stringToDatabaseEngineType('UNDEFINED');
            expect(result).toBe(DatabaseEngineType.UNDEFINED);
        });
    });

    describe('Invalid Strings', () => {
        it('should throw error for unsupported engine "oracle"', () => {
            expect(() => stringToDatabaseEngineType('oracle')).toThrow('Unsupported database engine: oracle');
        });

        it('should throw error for unsupported engine "mongodb"', () => {
            expect(() => stringToDatabaseEngineType('mongodb')).toThrow('Unsupported database engine: mongodb');
        });

        it('should throw error for unsupported engine "redis"', () => {
            expect(() => stringToDatabaseEngineType('redis')).toThrow('Unsupported database engine: redis');
        });

        it('should throw error for empty string', () => {
            expect(() => stringToDatabaseEngineType('')).toThrow('Unsupported database engine: ');
        });

        it('should throw error for numeric input', () => {
            expect(() => stringToDatabaseEngineType('123')).toThrow('Unsupported database engine: 123');
        });

        it('should throw error for special characters', () => {
            expect(() => stringToDatabaseEngineType('my-sql')).toThrow('Unsupported database engine: my-sql');
        });

        it('should throw error for partial matches', () => {
            expect(() => stringToDatabaseEngineType('my')).toThrow('Unsupported database engine: my');
            expect(() => stringToDatabaseEngineType('sql')).toThrow('Unsupported database engine: sql');
        });
    });

    describe('Edge Cases', () => {
        it('should be case insensitive but exact match required', () => {
            const validInputs = ['mysql', 'MySQL', 'MYSQL', 'mYsQl'];
            validInputs.forEach(input => {
                expect(() => stringToDatabaseEngineType(input)).not.toThrow();
                expect(stringToDatabaseEngineType(input)).toBe(DatabaseEngineType.MYSQL);
            });
        });
    });

    describe('Error Messages', () => {
        it('should include the invalid engine in error message', () => {
            const invalidEngine = 'invalidengine';
            expect(() => stringToDatabaseEngineType(invalidEngine))
                .toThrow(`Unsupported database engine: ${invalidEngine}`);
        });

        it('should handle special characters in error message', () => {
            const specialChars = ['mysql@', 'postgres#', 'sqlite$', 'mariadb%'];
            specialChars.forEach(char => {
                expect(() => stringToDatabaseEngineType(char))
                    .toThrow(`Unsupported database engine: ${char}`);
            });
        });
    });

    describe('Type Safety', () => {
        it('should return DatabaseEngineType enum value', () => {
            const result = stringToDatabaseEngineType('mysql');
            expect(Object.values(DatabaseEngineType)).toContain(result);
        });

        it('should handle all valid engine strings', () => {
            const validStrings = ['mysql', 'mariadb', 'postgres', 'sqlite', 'undefined'];
            validStrings.forEach(engineString => {
                expect(() => stringToDatabaseEngineType(engineString)).not.toThrow();
                expect(Object.values(DatabaseEngineType)).toContain(stringToDatabaseEngineType(engineString));
            });
        });
    });
});

describe('Round-trip Conversion Tests', () => {
    it('should maintain consistency between enum and string conversion', () => {
        Object.values(DatabaseEngineType).forEach(engine => {
            const stringRepresentation = databaseEngineTypeToString(engine);
            expect(stringRepresentation).toBe(engine);

            // Convert back using the string representation (lowercase)
            const backToEnum = stringToDatabaseEngineType(stringRepresentation.toLowerCase());
            expect(backToEnum).toBe(engine);
        });
    });

    it('should handle all database engine strings bidirectionally', () => {
        const testCases = [
            { string: 'mysql', enum: DatabaseEngineType.MYSQL, display: 'MySQL' },
            { string: 'mariadb', enum: DatabaseEngineType.MARIADB, display: 'MariaDB' },
            { string: 'postgres', enum: DatabaseEngineType.POSTGRES, display: 'Postgres' },
            { string: 'sqlite', enum: DatabaseEngineType.SQLITE, display: 'SQLite' },
            { string: 'undefined', enum: DatabaseEngineType.UNDEFINED, display: 'Undefined' }
        ];

        testCases.forEach(({ string, enum: engineEnum, display }) => {
            // String to enum
            expect(stringToDatabaseEngineType(string)).toBe(engineEnum);

            // Enum to string
            expect(databaseEngineTypeToString(engineEnum)).toBe(display);

            // Enum value should match display string
            expect(engineEnum).toBe(display);
        });
    });

    it('should handle case-insensitive round-trip conversion', () => {
        const testInputs = [
            'mysql', 'MySQL', 'MYSQL',
            'mariadb', 'MariaDB', 'MARIADB',
            'postgres', 'Postgres', 'POSTGRES',
            'sqlite', 'SQLite', 'SQLITE',
            'undefined', 'Undefined', 'UNDEFINED'
        ];

        testInputs.forEach(input => {
            const engine = stringToDatabaseEngineType(input);
            const backToString = databaseEngineTypeToString(engine);
            const finalEngine = stringToDatabaseEngineType(backToString.toLowerCase());
            expect(finalEngine).toBe(engine);
        });
    });
});

describe('Edge Cases and Performance', () => {
    it('should handle repeated calls efficiently', () => {
        const iterations = 1000;
        const startTime = performance.now();

        for (let i = 0; i < iterations; i++) {
            databaseEngineTypeToString(DatabaseEngineType.MYSQL);
            stringToDatabaseEngineType('mysql');
        }

        const endTime = performance.now();
        const executionTime = endTime - startTime;

        // Should complete within reasonable time (adjust threshold as needed)
        expect(executionTime).toBeLessThan(100); // 100ms for 1000 iterations
    });

    it('should handle all enum values in performance test', () => {
        const startTime = performance.now();

        for (let i = 0; i < 100; i++) {
            Object.values(DatabaseEngineType).forEach(engine => {
                databaseEngineTypeToString(engine);
                stringToDatabaseEngineType(engine.toLowerCase());
            });
        }

        const endTime = performance.now();
        const executionTime = endTime - startTime;

        expect(executionTime).toBeLessThan(50); // 50ms for 500 total operations
    });

    it('should maintain enum integrity', () => {
        // Ensure enum hasn't been accidentally modified
        expect(Object.keys(DatabaseEngineType)).toHaveLength(5);
        expect(Object.values(DatabaseEngineType)).toHaveLength(5);

        // Ensure no duplicates
        const values = Object.values(DatabaseEngineType);
        expect(new Set(values).size).toBe(values.length);
    });

    it('should handle database engine aliases consistently', () => {
        // Test common aliases/variations (should all throw errors)
        const aliases = [
            'postgresql', 'pg', 'psql',  // PostgreSQL aliases
            'mysql8', 'mysql5.7',       // MySQL versions
            'maria', 'mdb',             // MariaDB aliases
            'sqlite3', 'db',            // SQLite aliases
        ];

        aliases.forEach(alias => {
            expect(() => stringToDatabaseEngineType(alias))
                .toThrow(`Unsupported database engine: ${alias}`);
        });
    });
});

describe('Integration with Real-World Scenarios', () => {
    it('should handle connection string parsing scenarios', () => {
        const connectionStrings = [
            { input: 'mysql://localhost:3306/db', expected: DatabaseEngineType.MYSQL },
            { input: 'postgres://localhost:5432/db', expected: DatabaseEngineType.POSTGRES },
            { input: 'sqlite:///path/to/db.sqlite', expected: DatabaseEngineType.SQLITE },
            { input: 'mariadb://localhost:3306/db', expected: DatabaseEngineType.MARIADB }
        ];

        connectionStrings.forEach(({ input, expected }) => {
            const engineName = input.split('://')[0];
            if (['mysql', 'postgres', 'sqlite', 'mariadb'].includes(engineName)) {
                const result = stringToDatabaseEngineType(engineName);
                expect(result).toBe(expected);
            }
        });
    });

    it('should handle configuration file scenarios', () => {
        const configValues = [
            { config: { engine: 'MySQL' }, expected: DatabaseEngineType.MYSQL },
            { config: { engine: 'postgres' }, expected: DatabaseEngineType.POSTGRES },
            { config: { engine: 'SQLITE' }, expected: DatabaseEngineType.SQLITE },
            { config: { engine: 'mariadb' }, expected: DatabaseEngineType.MARIADB }
        ];

        configValues.forEach(({ config, expected }) => {
            const result = stringToDatabaseEngineType(config.engine);
            expect(result).toBe(expected);
        });
    });
});