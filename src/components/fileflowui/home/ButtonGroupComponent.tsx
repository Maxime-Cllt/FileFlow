import React from 'react';
import {Database, Eraser, Upload} from "lucide-react";

interface ButtonGroupProps {
    handleSubmit: (e: React.FormEvent) => void;
    handleInsert: (e: React.FormEvent) => void;
    handleReset: () => void;
    is_connected: boolean;
}

const ButtonGroupComponent: React.FC<ButtonGroupProps> = (props: ButtonGroupProps) => {
    return (
        <div className="flex items-center justify-center gap-6 mb-6 p-4">
            <button
                onClick={props.handleSubmit}
                className={`flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 ${
                    props.is_connected ? 'bg-green-500 hover:bg-green-600 text-white' : 'bg-red-500 text-white'
                }`}
                aria-label="Submit to Database"

                title={props.is_connected ? "Submit data to the specified database" : "Connect to a database first"}
            >
                <Database className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleInsert}
                className={`flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 ${
                    props.is_connected ? 'bg-blue-500 hover:bg-blue-600 text-white' : 'bg-gray-500 text-gray-700'
                }`}
                aria-label="Insert Data"

                title={props.is_connected ? "Insert data to the specified database" : "Connect to a database first"}
            >
                <Upload className="w-5 h-5"/>
            </button>

            <button
                onClick={props.handleReset}
                className="flex items-center justify-center bg-yellow-500 hover:bg-yellow-600 text-white p-3 rounded-full shadow-lg transition duration-300"
                aria-label="Reset" title={"Reset the form"}
            >
                <Eraser className="w-5 h-5"/>
            </button>
        </div>

    );
};

export default ButtonGroupComponent;
