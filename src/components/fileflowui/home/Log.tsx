import React from 'react';

interface LogProps {
    histoLog: string;
}

const Log: React.FC<LogProps> = (props: LogProps) => {
    return (
        <div
            className="flex flex-col items-center justify-center h-auto max-h-full">
            <label htmlFor="logArea" className="text-lg font-semibold mb-2 text-gray-700"></label>
            <textarea
                id="logArea"
                disabled
                value={props.histoLog}
                className="w-full h-72 p-3 border border-gray-300 rounded-md bg-gray-50 text-gray-600 resize-none shadow-sm"
                placeholder="Log history will appear here..."
            ></textarea>
        </div>
    );
};

export default Log;
