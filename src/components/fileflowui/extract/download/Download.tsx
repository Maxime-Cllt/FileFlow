import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {Label} from "@/components/ui/label.tsx";
import {ComboboxComponent} from "@/components/hooks/component/ComboboxComponent.tsx";
import {log_error} from "@/components/hooks/utils.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "@/components/ui/select.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {toast} from "sonner";
import DirectoryUpload from "@/components/hooks/file/DirectoryUpload.tsx";
import Loader from "@/components/hooks/Loader.tsx";
import ConnectionForm from "@/components/hooks/database/ConnectionForm.tsx";
import {AnimatePresence, motion} from 'framer-motion';

const Download: React.FC = () => {

        const [dbConfig, setDbConfig] = useState({
            db_driver: '',
            db_host: '',
            port: '',
            username: '',
            password: '',
            db_name: '',
            tableName: '',
            sqlite_file_path: '',
            is_connected: false
        });
        const [tables, setTables] = useState<Array<ComboItem>>([]);
        const [selectedTable, setSelectedTable] = useState<string | null>(null);
        const [exportFormat, setExportFormat] = useState<'csv' | 'json'>('csv');
        const [separator, setSeparator] = useState<',' | ';' | '\t'>(',');
        const [absolutePath, setAbsolutePath] = useState<string>('');
        const [showLoader, setShowLoader] = useState<boolean>(false);


        const updateDbConfigField = (field: keyof typeof dbConfig, value: any) => {
            setDbConfig(prev => ({...prev, [field]: value}));
        }

        const retrieveTables = async () => {
            try {
                const get_table_list_response: boolean | ComboItem[] = await invoke<Array<ComboItem> | boolean>('get_table_list');

                if (typeof get_table_list_response === "boolean") {
                    throw new Error('Failed to get table list');
                }

                const parsedData: ComboItem[] = get_table_list_response.map(item => ({
                    value: item.value,
                    label: item.label
                }));

                setTables(parsedData);
            } catch (error) {
                log_error(error)
            }
        }

        const handleDownload = async () => {
            try {

                if (!selectedTable && absolutePath === "") {
                    throw new Error('Please fill in all required fields');
                }

                setShowLoader(true);

                const download_table_response: string = await invoke<string>('download_table', {
                    config: {
                        table_name: selectedTable,
                        location: absolutePath,
                        separator: separator === ',' ? ',' : separator === ';' ? ';' : '\t'
                    }
                });

                if (!download_table_response.startsWith("Table")) {
                    throw new Error(download_table_response);
                }

                toast.success(download_table_response);
            } catch (error) {
                log_error(error)
            }

            setShowLoader(false);
        }

        useEffect(() => {
            if (dbConfig.is_connected) {
                retrieveTables().then()
            }
        }, [dbConfig.is_connected]);

        return (
            <div className="h-full w-full">

                {/* Connection Mode Section */}
                <ConnectionForm dbConfig={dbConfig} updateDbConfigField={updateDbConfigField}/>

                {/* Loader */}
                {showLoader && (
                    <div className="flex justify-center mt-6">
                        <Loader/>
                    </div>
                )}


                {/* Download Section */}
                <div className="container mx-auto pt-8 px-4 md:px-8 space-y-6">
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
                                <AnimatePresence>
                                    {(tables.length > 0 && dbConfig.is_connected) && (
                                        <motion.section
                                            initial={{opacity: 0, y: -10}}
                                            animate={{opacity: 1, y: 0}}
                                            exit={{opacity: 0, y: -10}}
                                            transition={{duration: 0.3}}
                                        >
                                            <h2 className="text-xl font-semibold text-gray-700 text-center mb-4">
                                                Tables available ({tables.length})
                                            </h2>
                                            <div className="flex justify-center">
                                                <ComboboxComponent lists={tables} setSelectedValue={setSelectedTable}/>
                                            </div>
                                        </motion.section>
                                    )}
                                </AnimatePresence>

                                {/* Export Configuration Section */}
                                <div className="border-t pt-6 space-y-4">

                                    <div className="max-w-md mx-auto">
                                        <div className="grid grid-cols-2 gap-4">

                                            {/* File Format */}
                                            <div className="flex flex-col">
                                                <Label htmlFor="export-format"
                                                       className="text-sm font-medium text-gray-700">
                                                    File Format:
                                                </Label>
                                                <Select
                                                    value={exportFormat}
                                                    onValueChange={(value) => setExportFormat(value as 'csv' | 'json')}
                                                >
                                                    <SelectTrigger id="export-format" className="w-32">
                                                        {exportFormat.toUpperCase()}
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectItem value="csv">CSV</SelectItem>
                                                        <SelectItem value="json">JSON</SelectItem>
                                                    </SelectContent>
                                                </Select>
                                            </div>

                                            {/* Separator Format */}
                                            <div className="flex flex-col">
                                                <Label htmlFor="separator" className="text-sm font-medium text-gray-700">
                                                    Separator:
                                                </Label>
                                                <Select
                                                    value={separator}
                                                    onValueChange={(value) => setSeparator(value as ',' | ';' | '\t')}
                                                >
                                                    <SelectTrigger id="separator" className="w-32">
                                                        {separator === ',' ? 'Comma' : separator === ';' ? 'Semicolon' : 'Tab'}
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        <SelectItem value=",">Comma</SelectItem>
                                                        <SelectItem value=";">Semicolon</SelectItem>
                                                        <SelectItem value="\t">Tab</SelectItem>
                                                    </SelectContent>
                                                </Select>
                                            </div>
                                        </div>
                                    </div>

                                    <DirectoryUpload absolutePath={absolutePath} setAbsolutePath={setAbsolutePath}/>
                                </div>

                                {/* Download Button Section */}
                                <section className="flex justify-center pt-4">
                                    <button
                                        onClick={handleDownload}
                                        disabled={showLoader}
                                        className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-8 rounded focus:outline-none focus:ring focus:ring-green-300 disabled:opacity-50"
                                    >
                                        {showLoader ? "Downloading..." : "Download Table"}
                                    </button>
                                </section>
                            </div>
                        </CardContent>
                    </Card>
                </div>

            </div>
        )
            ;
    }
;

export default Download;
