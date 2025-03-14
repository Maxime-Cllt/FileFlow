import React, {useEffect, useState} from 'react';
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import SelectDatabaseConfig from "@/components/hooks/database/SelectDatabaseConfig.tsx";
import DatabaseForm, {DatabaseFormProps} from "@/components/hooks/database/DatabaseForm.tsx";
import {
    connect_to_database,
    disconnect_from_database,
    getAllConfigs,
    is_connected,
    log_error
} from "@/components/hooks/utils.tsx";
import {Tabs, TabsContent, TabsList, TabsTrigger} from "@/components/ui/tabs.tsx";
import ButtonConfigComponent from "@/components/fileflowui/load/insert/ButtonConfig.tsx";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip.tsx";
import {Database} from "lucide-react";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger
} from "@/components/ui/dropdown-menu.tsx";

const ConnectionForm: React.FC<DatabaseFormProps> = (props: DatabaseFormProps) => {

        const [configName, setConfigName] = React.useState<string>('');
        const [configNameList, setConfigNameList] = useState<Array<Item>>([]);
        const [attemptedConnection, setAttemptedConnection] = useState<boolean>(false);

        const handleConnect = async (): Promise<void> => {
            try {
                setAttemptedConnection(true);

                if (props.dbConfig.db_driver !== 'sqlite' && (!props.dbConfig.db_driver || !props.dbConfig.db_host || !props.dbConfig.port || !props.dbConfig.username || !props.dbConfig.db_name)) {
                    throw new Error('Please fill all fields');
                } else if (props.dbConfig.db_driver === 'sqlite' && !props.dbConfig.sqlite_file_path) {
                    throw new Error('Please fill all fields');
                }

                const connect_to_database_response: boolean | void = await connect_to_database(
                    props.dbConfig.db_driver,
                    props.dbConfig.db_host,
                    props.dbConfig.port,
                    props.dbConfig.username,
                    props.dbConfig.password,
                    props.dbConfig.db_name,
                    props.dbConfig.sqlite_file_path
                );

                if (typeof connect_to_database_response !== "boolean") {
                    throw new Error("Failed to connect to database, please check your connection details");
                }

                if (!connect_to_database_response) {
                    throw new Error("Failed to connect to database, please check your connection");
                }

                props.updateDbConfigField('is_connected', true);

            } catch
                (error) {
                log_error(error);
            }
            setAttemptedConnection(false);
        }

        useEffect(() => {
            getAllConfigs().then(
                (configs: Array<Item>) => {
                    setConfigNameList(configs);
                }
            );

            is_connected().then(
                (connected: string | boolean) => {
                    if (typeof connected !== "string" || !connected) {
                        props.updateDbConfigField('is_connected', false);
                        return;
                    }
                    const loadDbConfig = JSON.parse(connected);
                    Object.keys(loadDbConfig).forEach((key) => {
                        props.updateDbConfigField(key as keyof typeof props.dbConfig, loadDbConfig[key]);
                    });
                    props.updateDbConfigField('is_connected', true);
                }
            )

        }, []);

        return (
            <div className="container mx-auto pt-8 px-4 md:px-8 mt-6 space-y-6">

                {/* Connection Mode Section */}
                <Card>
                    <CardHeader>
                        <div className="flex items-center space-x-2 mb-6">
                            <CardTitle
                                className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                                Database Configuration
                            </CardTitle>
                            <ButtonConfigComponent
                                dbConfig={props.dbConfig}
                                updateDbConfigField={props.updateDbConfigField}
                            />
                        </div>
                    </CardHeader>
                    <CardContent>
                        <Tabs defaultValue="predefined" className="space-y-6">
                            <div className="flex justify-center">
                                <TabsList>
                                    <TabsTrigger value="predefined">Saved</TabsTrigger>
                                    <TabsTrigger value="manual">Manual</TabsTrigger>
                                </TabsList>
                            </div>
                            <TabsContent value="predefined">
                                <div className="p-6 mr-10 ml-10">
                                    <SelectDatabaseConfig
                                        updateConfigName={setConfigName}
                                        configName={configName}
                                        configNameList={configNameList}
                                        updateDbConfigField={props.updateDbConfigField}
                                    />
                                </div>
                            </TabsContent>
                            <TabsContent value="manual">
                                <DatabaseForm
                                    dbConfig={props.dbConfig}
                                    updateDbConfigField={props.updateDbConfigField}
                                />
                            </TabsContent>
                        </Tabs>

                        {/* Database Connection Button */}
                        <div className="flex justify-center">
                            <DropdownMenu>
                                <DropdownMenuTrigger>
                                    <TooltipProvider>
                                        <Tooltip>
                                            <TooltipTrigger>
                                                <button
                                                    disabled={attemptedConnection}
                                                    className={`flex items-center justify-center p-3 rounded-full shadow-md transition duration-300  focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                                                    ${
                                                        props.dbConfig.is_connected
                                                            ? "bg-green-500 hover:bg-green-600 text-white"
                                                            : "bg-yellow-500 hover:bg-yellow-600 text-white"
                                                    }`}
                                                    title={props.dbConfig.is_connected ? "Disconnect from database" : "Connect to database"}
                                                >
                                                    <Database className="w-6 h-6"/>
                                                </button>
                                            </TooltipTrigger>
                                            <TooltipContent>
                                                Connect or disconnect from database
                                            </TooltipContent>
                                        </Tooltip>
                                    </TooltipProvider>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent>
                                    <DropdownMenuItem onClick={handleConnect}
                                                      disabled={props.dbConfig.is_connected}>Connect</DropdownMenuItem>
                                    <DropdownMenuItem onClick={async function () {
                                        const response = await disconnect_from_database();
                                        if (response) {
                                            props.updateDbConfigField('is_connected', false);
                                        }
                                    }} disabled={!props.dbConfig.is_connected}>Disconnect</DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </div>
                    </CardContent>
                </Card>

            </div>
        );
    }
;

export default ConnectionForm;
