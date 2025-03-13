import React from 'react';
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";

const HelpInsert: React.FC = () => {
    return (
        <div className="h-full w-full">

            <div className="container mx-auto pt-8 px-4 md:px-8 mt-6 space-y-6">

                {/* Page Header with theme color */}
                <h2
                    className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text mb-8"> How
                    to Use the Insertion View
                </h2>

                <hr className="border-2 border-gray-200"/>

                <div className="space-y-6">
                    {/* Connection Mode Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl font-semibold">Connection Mode</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-700">
                                This is where you set up your database connection. Use the Connection Form to fill in
                                details like your database driver, host, port, username, and password. Once configured,
                                the
                                system will attempt to connect.
                            </p>
                        </CardContent>
                    </Card>

                    {/* Loader Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl font-semibold">Loader Indicator</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-700">
                                When an operation is in progress, a loader (spinner) appears at the center of the
                                screen.
                                This visual cue indicates that your request is being processed, so please wait until it
                                finishes.
                            </p>
                        </CardContent>
                    </Card>

                    {/* Insertion Configuration Section */}
                    <Card>
                        <CardHeader>
                            <div className="flex items-center justify-between border-b pb-4">
                                <CardTitle
                                    className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                                    Insertion Configuration
                                </CardTitle>
                            </div>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-700 mb-4">
                                In this section, you configure how your data will be inserted:
                            </p>
                            <ul className="list-disc ml-6 text-gray-700">
                                <li>
                                    <strong>File Upload:</strong> Click on the file upload area to select a file. The
                                    selected file’s path will be used to generate an initial table name.
                                </li>
                                <li>
                                    <strong>Table Name Input:</strong> Review the auto-populated table name (derived
                                    from
                                    your file name). You can manually edit this if needed.
                                </li>
                                <li>
                                    <strong>Mode Selection:</strong> Choose the mode (for example, "fast") for data
                                    insertion. This option adjusts how the insertion process behaves.
                                </li>
                                <li>
                                    <strong>Action Buttons:</strong> Use the buttons provided to initiate or manage the
                                    insertion process.
                                </li>
                            </ul>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    );
};

export default HelpInsert;
