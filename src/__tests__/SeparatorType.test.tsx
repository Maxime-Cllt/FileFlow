import {charToSeparatorType, SeparatorType, separatorTypeToString} from "../state/SeparatorType";

describe('SeparatorType Enum', () => {
    describe('Enum Values', () => {
        it('should have correct string values for each separator type', () => {
            expect(SeparatorType.COMMA).toBe(',');
            expect(SeparatorType.SEMICOLON).toBe(';');
            expect(SeparatorType.SPACE).toBe(' ');
            expect(SeparatorType.PIPE).toBe('|');
        });

        it('should have exactly 4 separator types', () => {
            const separatorCount = Object.keys(SeparatorType).length;
            expect(separatorCount).toBe(4);
        });

        it('should contain all expected separator types', () => {
            const expectedTypes = ['COMMA', 'SEMICOLON', 'SPACE', 'PIPE'];
            const actualTypes = Object.keys(SeparatorType);
            expect(actualTypes).toEqual(expect.arrayContaining(expectedTypes));
        });
    });
});

describe('separatorTypeToString', () => {
    describe('Valid Separator Types', () => {
        it('should convert COMMA to "Comma"', () => {
            const result = separatorTypeToString(SeparatorType.COMMA);
            expect(result).toBe('Comma');
        });

        it('should convert SEMICOLON to "Semicolon"', () => {
            const result = separatorTypeToString(SeparatorType.SEMICOLON);
            expect(result).toBe('Semicolon');
        });

        it('should convert SPACE to "Space"', () => {
            const result = separatorTypeToString(SeparatorType.SPACE);
            expect(result).toBe('Space');
        });

        it('should convert PIPE to "Pipe"', () => {
            const result = separatorTypeToString(SeparatorType.PIPE);
            expect(result).toBe('Pipe');
        });
    });

    describe('Invalid Separator Types', () => {
        it('should return "Unknown" for invalid separator type', () => {
            // Force TypeScript to accept an invalid value for testing
            const invalidSeparator = 'invalid' as SeparatorType;
            const result = separatorTypeToString(invalidSeparator);
            expect(result).toBe('Unknown');
        });

        it('should return "Unknown" for null value', () => {
            const result = separatorTypeToString(null as any);
            expect(result).toBe('Unknown');
        });

        it('should return "Unknown" for undefined value', () => {
            const result = separatorTypeToString(undefined as any);
            expect(result).toBe('Unknown');
        });
    });

    describe('Type Safety', () => {
        it('should return string type', () => {
            const result = separatorTypeToString(SeparatorType.COMMA);
            expect(typeof result).toBe('string');
        });

        it('should handle all enum values without compilation errors', () => {
            // This test ensures all enum values are handled in the switch statement
            Object.values(SeparatorType).forEach(separator => {
                expect(() => separatorTypeToString(separator)).not.toThrow();
            });
        });
    });
});

describe('charToSeparatorType', () => {
    describe('Valid Characters', () => {
        it('should convert "," to SeparatorType.COMMA', () => {
            const result = charToSeparatorType(',');
            expect(result).toBe(SeparatorType.COMMA);
        });

        it('should convert ";" to SeparatorType.SEMICOLON', () => {
            const result = charToSeparatorType(';');
            expect(result).toBe(SeparatorType.SEMICOLON);
        });

        it('should convert " " to SeparatorType.SPACE', () => {
            const result = charToSeparatorType(' ');
            expect(result).toBe(SeparatorType.SPACE);
        });

        it('should convert "|" to SeparatorType.PIPE', () => {
            const result = charToSeparatorType('|');
            expect(result).toBe(SeparatorType.PIPE);
        });
    });

    describe('Invalid Characters', () => {
        it('should throw error for unsupported character "."', () => {
            expect(() => charToSeparatorType('.')).toThrow('Unsupported separator: .');
        });

        it('should throw error for unsupported character "/"', () => {
            expect(() => charToSeparatorType('/')).toThrow('Unsupported separator: /');
        });

        it('should throw error for unsupported character "\t"', () => {
            expect(() => charToSeparatorType('\t')).toThrow('Unsupported separator: \t');
        });

        it('should throw error for empty string', () => {
            expect(() => charToSeparatorType('')).toThrow('Unsupported separator: ');
        });

        it('should throw error for multi-character string', () => {
            expect(() => charToSeparatorType(',;')).toThrow('Unsupported separator: ,;');
        });

        it('should throw error for null value', () => {
            expect(() => charToSeparatorType(null as any)).toThrow('Unsupported separator: null');
        });

        it('should throw error for undefined value', () => {
            expect(() => charToSeparatorType(undefined as any)).toThrow('Unsupported separator: undefined');
        });

        it('should throw error for numeric input', () => {
            expect(() => charToSeparatorType('1')).toThrow('Unsupported separator: 1');
        });

        it('should throw error for alphabetic character', () => {
            expect(() => charToSeparatorType('a')).toThrow('Unsupported separator: a');
        });
    });

    describe('Error Messages', () => {
        it('should include the invalid character in error message', () => {
            const invalidChar = '#';
            expect(() => charToSeparatorType(invalidChar))
                .toThrow(`Unsupported separator: ${invalidChar}`);
        });

        it('should handle special characters in error message', () => {
            const specialChars = ['\n', '\r', '\t', '\\'];
            specialChars.forEach(char => {
                expect(() => charToSeparatorType(char))
                    .toThrow(`Unsupported separator: ${char}`);
            });
        });
    });

    describe('Type Safety', () => {
        it('should return SeparatorType enum value', () => {
            const result = charToSeparatorType(',');
            expect(Object.values(SeparatorType)).toContain(result);
        });

        it('should handle all valid separator characters', () => {
            const validChars = [',', ';', ' ', '|'];
            validChars.forEach(char => {
                expect(() => charToSeparatorType(char)).not.toThrow();
                expect(Object.values(SeparatorType)).toContain(charToSeparatorType(char));
            });
        });
    });
});

describe('Round-trip Conversion Tests', () => {
    it('should maintain consistency between enum and string conversion', () => {
        // Test that converting enum to string and back to enum works correctly
        Object.values(SeparatorType).forEach(separator => {
            const stringRepresentation = separatorTypeToString(separator);
            expect(stringRepresentation).not.toBe('Unknown');

            // Convert back using the actual separator character
            const backToEnum = charToSeparatorType(separator);
            expect(backToEnum).toBe(separator);
        });
    });

    it('should handle all separator characters bidirectionally', () => {
        const testCases = [
            {char: ',', enum: SeparatorType.COMMA, name: 'Comma'},
            {char: ';', enum: SeparatorType.SEMICOLON, name: 'Semicolon'},
            {char: ' ', enum: SeparatorType.SPACE, name: 'Space'},
            {char: '|', enum: SeparatorType.PIPE, name: 'Pipe'}
        ];

        testCases.forEach(({char, enum: separatorEnum, name}) => {
            // Character to enum
            expect(charToSeparatorType(char)).toBe(separatorEnum);

            // Enum to string name
            expect(separatorTypeToString(separatorEnum)).toBe(name);

            // Enum value should match character
            expect(separatorEnum).toBe(char);
        });
    });
});

describe('Edge Cases and Performance', () => {
    it('should handle repeated calls efficiently', () => {
        // Performance test - should not degrade with repeated calls
        const iterations = 1000;
        const startTime = performance.now();

        for (let i = 0; i < iterations; i++) {
            separatorTypeToString(SeparatorType.COMMA);
            charToSeparatorType(',');
        }

        const endTime = performance.now();
        const executionTime = endTime - startTime;

        // Should complete within reasonable time (adjust threshold as needed)
        expect(executionTime).toBeLessThan(100); // 100ms for 1000 iterations
    });

    it('should be case-sensitive for character conversion', () => {
        // Uppercase versions should throw errors
        expect(() => charToSeparatorType('A')).toThrow();
        expect(() => charToSeparatorType('COMMA')).toThrow();
    });

    it('should handle Unicode characters appropriately', () => {
        const unicodeChars = ['，', '；', '｜', '　']; // Similar Unicode characters
        unicodeChars.forEach(char => {
            expect(() => charToSeparatorType(char)).toThrow();
        });
    });
});