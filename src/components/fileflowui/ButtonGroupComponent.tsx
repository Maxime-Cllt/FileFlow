import React from 'react';
import {Database, Eraser, Upload} from "lucide-react";

interface ButtonGroupProps {
    handleSubmit: (e: React.FormEvent) => void;
    handleInsert: (e: React.FormEvent) => void;
    handleReset: () => void;
}

const ButtonGroupComponent: React.FC<ButtonGroupProps> = (props: ButtonGroupProps) => {
    return (
        <div className="flex items-center justify-center gap-6 mb-6 p-4">
            <button
                onClick={props.handleSubmit}
                className="flex items-center justify-center bg-green-600 hover:bg-green-700 text-white p-3 rounded-full shadow-lg transition duration-300"
                aria-label="Submit to Database"
            >
                <Database className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleInsert}
                className="flex items-center justify-center bg-blue-500 hover:bg-blue-600 text-white p-3 rounded-full shadow-lg transition duration-300"
                aria-label="Insert Data"
            >
                <Upload className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleReset}
                className="flex items-center justify-center bg-red-500 hover:bg-red-600 text-white p-3 rounded-full shadow-lg transition duration-300"
                aria-label="Reset"
            >
                <Eraser className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default ButtonGroupComponent;
