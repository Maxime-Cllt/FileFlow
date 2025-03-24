import React from 'react';
import {Card, CardContent, CardHeader, CardTitle} from '@/components/ui/card.tsx';

const HelpDownload: React.FC = () => {
    return (
        <div className="h-full w-full">

            <div className="container mx-auto pt-8 px-4 md:px-8 mt-6 space-y-6">
                {/* Page Header with your gradient color */}
                <h2 className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text mb-8">
                    How to Use the Export Configuration
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
                                Start by configuring your database connection with the Connection Form. This step is
                                crucial to access available tables.
                            </p>
                        </CardContent>
                    </Card>

                    {/* Loader Explanation */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-2xl font-semibold">Loader Indicator</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-700">
                                A loader (spinner) appears while the system is processing an operation (like fetching
                                tables or exporting data). Please wait until the loader disappears.
                            </p>
                        </CardContent>
                    </Card>

                    {/* Export Configuration Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle
                                className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                                Export Configuration
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div className="space-y-8">
                                {/* Tables Display Section */}
                                <div>
                                    <h2 className="text-xl font-semibold text-gray-700 text-center mb-4">
                                        Tables Available
                                    </h2>
                                    <p className="text-gray-700">
                                        If your database connection is active and tables are available, they will be
                                        listed here with an animated effect. Use the dropdown menu to select the table
                                        you want to export.
                                    </p>
                                </div>

                                {/* Export Options Section */}
                                <div className="border-t pt-6 space-y-4">
                                    <p className="text-gray-700">
                                        Configure your export options below:
                                    </p>
                                    <ul className="list-disc ml-6 text-gray-700">
                                        <li>
                                            <strong>File Format:</strong> Choose your export file format (CSV or JSON).
                                        </li>
                                        <li>
                                            <strong>Separator:</strong> For CSV files, select a separator — options
                                            include comma, semicolon, or tab.
                                        </li>
                                    </ul>
                                    <p className="text-gray-700">
                                        Additionally, you can set the directory where the exported file will be saved.
                                    </p>
                                </div>

                                {/* Download Button Section */}
                                <div>
                                    <h2 className="text-xl font-semibold text-gray-700 text-center mb-4">
                                        Download Your Exported Table
                                    </h2>
                                    <p className="text-gray-700">
                                        When you're ready, click the <span
                                        className="font-bold">Download Table</span> button. The button will display
                                        "Downloading..." during the export process.
                                    </p>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    );
};

export default HelpDownload;
