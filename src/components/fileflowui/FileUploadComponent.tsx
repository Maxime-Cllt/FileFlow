import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {FileArchive} from "lucide-react";
import {dialog} from "@tauri-apps/api";
import {invoke} from "@tauri-apps/api/tauri";

interface FileUploadProps {
    fileName: string;
    fileSize: string;
    setFilePath: (filePath: string | null) => void;
    setFileName: (name: string) => void;
    setFileSize: (size: string) => void;
    setTableName: (tableName: string) => void;
    addLog: (message: string) => void;
}

const FileUploadComponent: React.FC<FileUploadProps> = ({
                                                            fileName,
                                                            fileSize,
                                                            setFilePath,
                                                            setFileName,
                                                            setFileSize,
                                                            setTableName,
                                                            addLog
                                                        }) => {
    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: false
            });

            if (selectedFilePath) {
                const path = selectedFilePath.toString();
                setTableName(path.split('/').pop()?.split('.')[0]?.toLowerCase() || '');
                setFileName(path.split('/').pop() || '');
                setFilePath(path);

                const response = await invoke('get_size_of_file', {filePath: path});
                setFileSize(typeof response === 'string' ? response : '');
            }
        } catch (error) {
            addLog(`Erreur lors de la sélection du fichier: ${error}`);
        }
    };

    return (
        <div className="flex items-center gap-4">
            <Button onClick={openFileDialog} className="bg-blue-500 hover:bg-blue-600">
                <FileArchive/>
            </Button>
            <Input
                type="text"
                value={fileName ? `${fileName} (${fileSize})` : ''}
                placeholder="Fichier CSV"
                disabled
                className="w-full"
            />
        </div>
    );
};

export default FileUploadComponent;
