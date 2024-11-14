import React from 'react';
import {Copy, Eraser, Upload} from "lucide-react";

interface LoadButtonGroupProps {
    handleGenerate: () => void;
    handleCopy: () => void;
    handleReset: () => void;
}

const LoadButtonGroupComponent: React.FC<LoadButtonGroupProps> = (props: LoadButtonGroupProps) => {
    return (
        <div className="flex justify-center mb-6 space-x-6">
            <button
                onClick={props.handleGenerate}
                type={"button"}
                aria-label="Submit to Database"
                title="Submit data to the specified database"
                className="p-3 rounded-full shadow-lg bg-green-500 hover:bg-green-600 text-white transition duration-300"
            >
                <Upload className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleCopy}
                type={"button"}
                aria-label="Copy to Clipboard"
                title="Copy the generated SQL to clipboard"
                className="p-3 rounded-full shadow-lg bg-blue-500 hover:bg-blue-600 text-white transition duration-300"
            >
                <Copy className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleReset}
                aria-label="Reset"
                type={"button"}
                title="Reset the form"
                className="p-3 rounded-full shadow-lg bg-yellow-500 hover:bg-yellow-600 text-white transition duration-300"
            >
                <Eraser className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default LoadButtonGroupComponent;
