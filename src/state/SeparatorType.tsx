/**
 * Enum representing different types of separators used in data processing.
 * Each enum value corresponds to a specific character used to separate values.
 */
export enum SeparatorType {
    COMMA = ',',
    SEMICOLON = ';',
    SPACE = ' ',
    PIPE = '|'
}


/**
 * Converts a SeparatorType enum value to its string representation.
 * @param separator - The SeparatorType enum value to convert.
 * @returns The string representation of the separator type.
 */
export function separatorTypeToString(separator: SeparatorType): string {
    switch (separator) {
        case SeparatorType.COMMA:
            return 'Comma';
        case SeparatorType.SEMICOLON:
            return 'Semicolon';
        case SeparatorType.SPACE:
            return 'Space';
        case SeparatorType.PIPE:
            return 'Pipe';
        default:
            return 'Unknown';
    }
}

/**
 * Converts a character to its corresponding SeparatorType enum value.
 * @param separator - The character representing the separator.
 * @returns The corresponding SeparatorType enum value.
 * @throws Error if the character is not a valid separator.
 */
export function charToSeparatorType(separator: string): SeparatorType {
    switch (separator) {
        case ',':
            return SeparatorType.COMMA;
        case ';':
            return SeparatorType.SEMICOLON;
        case ' ':
            return SeparatorType.SPACE;
        case '|':
            return SeparatorType.PIPE;
        default:
            throw new Error(`Unsupported separator: ${separator}`);
    }
}
