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

const FileUploadComponent: React.FC<FileUploadProps> = (props: FileUploadProps) => {

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: false
            });

            if (selectedFilePath) {
                const path = selectedFilePath.toString();
                const normalizedTableName = getNormalizedTableName(path);

                props.setFileName(getFileNameFromPath(path));
                props.setFilePath(path);
                props.setTableName(normalizedTableName);

                const response = await invoke('get_size_of_file', {filePath: path});
                props.setFileSize(typeof response === 'string' ? response : '');
            }
        } catch (error) {
            props.addLog(`Error opening file: ${error}`);
        }
    };

    const getFileNameFromPath = (path: string) => path.split('/').pop() || '';

    const getNormalizedTableName = (path: string) => {
        const fileName = getFileNameFromPath(path).split('.').shift() || '';
        return fileName
            .replace(/[^a-zA-Z0-9_]/g, '')
            .replace(/^_/, '');
    };

    return (
        <div className="flex items-center gap-4">
            <Button onClick={openFileDialog} className="bg-blue-500 hover:bg-blue-600">
                <FileArchive/>
            </Button>
            <Input
                type="text"
                value={props.fileName ? `${props.fileName} (${props.fileSize})` : ''}
                placeholder="Select a CSV file"
                disabled
                className="w-full"
            />
        </div>
    );
};

export default FileUploadComponent;
