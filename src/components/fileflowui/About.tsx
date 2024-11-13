import React from 'react';
import Menu from "@/components/fileflowui/Menu.tsx";
import {Card} from "@/components/ui/card.tsx";

const AboutComponent: React.FC = () => {
    return (
        <div className="bg-gray-100">
            {/* Menu */}
            <div className="fixed top-0 w-full bg-white shadow-md z-10">
                <Menu/>
            </div>

            {/* Hero Section */}
            <div
                className="flex items-center justify-center h-64 bg-gradient-to-r from-blue-600 to-purple-600 text-white">
                <h1 className="text-6xl font-bold">FileFlow</h1>
            </div>

            {/* About Section */}
            <div className="flex justify-center align-middle mt-12 px-4">

                <div className="w-full max-w-6xl space-y-8">
                    {/* Description Card */}
                    <Card className="p-8 shadow-lg bg-white">
                        <h2 className="text-3xl font-bold text-center mb-4">About FileFlow</h2>
                        <p className="text-lg text-gray-700 text-center">
                            FileFlow is a simple tool that allows you to insert data from a CSV file into a database
                            table,
                            built with Rust and Tauri framework. It's cross-platform and can be used on Windows, MacOS,
                            and Linux.
                        </p>
                    </Card>

                    {/* Features and Usage in Grid */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                        {/* Features Card */}
                        <Card className="p-6 shadow-md bg-white">
                            <h2 className="text-2xl font-semibold mb-4">Features</h2>
                            <ul className="list-disc list-inside text-gray-700 space-y-2">
                                <li>Insert data into a new table</li>
                                <li>Insert data into an existing table</li>
                                <li>Optimize the type of the columns (VARCHAR(MAX_LENGTH))</li>
                                <li>From CSV file</li>
                                <li>No privileges required to insert data</li>
                            </ul>
                        </Card>

                        {/* Usage Card */}
                        <Card className="p-6 shadow-md bg-white">
                            <h2 className="text-2xl font-semibold mb-4">Usage</h2>
                            <ol className="list-decimal list-inside text-gray-700 space-y-2">
                                <li>Select the CSV file you want to insert into the database</li>
                                <li>Select the target database</li>
                                <li>Click on the "Insert" button</li>
                                <li>Wait for the data to be inserted</li>
                                <li>Done!</li>
                            </ol>
                        </Card>
                    </div>

                    {/* Modes of Insertion */}
                    <Card className="p-6 shadow-md bg-white mb-16">
                        <h2 className="text-2xl font-semibold mb-4">Modes of Insertion</h2>
                        <ul className="list-disc list-inside text-gray-700 space-y-2">
                            <li><strong>Optimized mode:</strong> Inserts data into a new table with optimized column
                                types (VARCHAR(MAX_LENGTH))
                            </li>
                            <li><strong>Fast mode:</strong> Inserts data into an existing table with columns as defined
                                in the CSV
                            </li>
                        </ul>
                        <p className="text-sm text-gray-500 mt-4">
                            <strong>Note:</strong> Optimized mode may take longer, but ensures better data type
                            management.
                        </p>

                    </Card>


                    <div className={"h-8"}></div>

                </div>
            </div>
        </div>
    );
};

export default AboutComponent;
