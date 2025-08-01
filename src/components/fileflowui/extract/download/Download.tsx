import React, {useEffect, useState} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {Label} from "@/components/ui/label.tsx";
import {log_error, requestAllTablesFromConnection} from "@/components/hooks/utils.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger} from "@/components/ui/select.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {toast} from "sonner";
import DirectoryUpload from "@/components/hooks/file/DirectoryUpload.tsx";
import Loader from "@/components/hooks/Loader.tsx";
import ConnectionForm from "@/components/hooks/database/ConnectionForm.tsx";
import {AnimatePresence, motion} from 'framer-motion';
import {DatabaseConfig} from "@/interfaces/DatabaseConfig.tsx";
import {CheckBoxCombo} from "@/components/hooks/component/CheckBoxCombo.tsx";
import {SeparatorType, separatorTypeToString} from "@/state/SeparatorType.tsx";

const Download: React.FC = () => {

        const [dbConfig, setDbConfig] = useState<DatabaseConfig>({
            configName: '',
            dbDriver: '',
            dbHost: '',
            dbName: '',
            password: '',
            port: '',
            sqliteFilePath: '',
            username: '',
            isConnected: false
        });

        const [tables, setTables] = useState<Array<ComboItem>>([]);
        const [selectedTables, setSelectedTables] = useState<string[]>([]);
        const [separator, setSeparator] = useState<SeparatorType>(SeparatorType.COMMA);
        const [absolutePath, setAbsolutePath] = useState<string>('');
        const [showLoader, setShowLoader] = useState<boolean>(false);

        const updateDbConfigField = (field: keyof DatabaseConfig, value: DatabaseConfig[keyof DatabaseConfig]) => {
            setDbConfig(prev => ({...prev, [field]: value}));
        }

        const retrieveTables = async (): Promise<void> => {
            try {

                const parsedData: ComboItem[] | boolean = await requestAllTablesFromConnection();

                if (typeof parsedData === "boolean") {
                    throw new Error("Failed to retrieve tables");
                }

                setTables(parsedData);
            } catch (error) {
                log_error(error)
            }
        }

        const handleDownload = async () => {
            try {

                if (!selectedTables && absolutePath === "") {
                    throw new Error('Please fill in all required fields');
                }

                setShowLoader(true);

                const download_table_response: string = await invoke<string>('download_table', {
                    config: {
                        table_name_list: selectedTables,
                        location: absolutePath,
                        separator: separatorTypeToString(separator).toLocaleLowerCase()
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
            if (dbConfig.isConnected) {
                retrieveTables().then()
            }
        }, [dbConfig.isConnected]);

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
                            <div className="flex items-center justify-between border-b pb-4">
                                <CardTitle
                                    className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                                    Export Configuration
                                </CardTitle>
                            </div>
                        </CardHeader>
                        <CardContent>
                            <div className="space-y-8">
                                {/* Tables Display Section */}
                                <AnimatePresence>
                                    {(tables.length > 0 && dbConfig.isConnected) && (
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
                                                <CheckBoxCombo lists={tables} setSelectedValue={setSelectedTables}/>
                                            </div>
                                        </motion.section>
                                    )}
                                </AnimatePresence>

                                {/* Export Configuration Section */}
                                <div className="pt-6 space-y-4">

                                    <div className="flex justify-center gap-6 align-middle">

                                        {/* File Format */}
                                        <div className="flex flex-col w-1/2">
                                            <DirectoryUpload absolutePath={absolutePath}
                                                             setAbsolutePath={setAbsolutePath}/>
                                        </div>
                                        {/* Separator Format */}
                                        <div className="flex flex-col">
                                            <Label htmlFor="separator" className="text-sm font-medium text-gray-700">
                                                Separator:
                                            </Label>
                                            <Select
                                                value={separator}
                                                onValueChange={(value) => setSeparator(value as SeparatorType)}
                                            >
                                                <SelectTrigger id="separator" className="w-32">
                                                    {separatorTypeToString(separator)}
                                                </SelectTrigger>
                                                <SelectContent>
                                                    <SelectItem value=",">Comma</SelectItem>
                                                    <SelectItem value=";">Semicolon</SelectItem>
                                                    <SelectItem value="\t">Tab</SelectItem>
                                                    <SelectItem value="|">Pipe</SelectItem>
                                                    <SelectItem value=" ">Space</SelectItem>
                                                </SelectContent>
                                            </Select>
                                        </div>

                                    </div>

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
        );
    }
;

export default Download;
