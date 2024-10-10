import React from 'react';
import { Textarea } from "@/components/ui/textarea.tsx";

interface LogProps {
    histoLog: string;
}

const LogComponent: React.FC<LogProps> = ({ histoLog }) => {
    return (
        <div className="flex flex-col mt-4 align-center h-60 min-h-4 max-h-full">
            <Textarea disabled value={histoLog} className="w-full h-72" />
        </div>
    );
};

export default LogComponent;
