import React from 'react';
import {Button} from "@/components/ui/button.tsx";
import {Database, Eraser, Upload} from "lucide-react";

interface ButtonGroupProps {
    handleSubmit: (e: React.FormEvent) => void;
    handleInsert: (e: React.FormEvent) => void;
    handleReset: () => void;
}

const ButtonGroupComponent: React.FC<ButtonGroupProps> = ({handleSubmit, handleInsert, handleReset}) => {
    return (
        <div className="flex items-center justify-center col-span-2 gap-4 mb-4">
            <Button onClick={handleSubmit} className=" mx-auto bg-green-600 hover:bg-green-700">
                <Database/>
            </Button>
            <Button onClick={handleInsert} className=" mx-auto bg-blue-500 hover:bg-blue-600">
                <Upload/>
            </Button>
            <Button onClick={handleReset} className=" mx-auto bg-red-500 hover:bg-red-600">
                <Eraser/>
            </Button>
        </div>
    );
};

export default ButtonGroupComponent;
