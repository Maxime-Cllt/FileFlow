import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {FileArchive} from "lucide-react";
import {invoke} from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog"
import {toast} from "sonner";

interface FileUploadProps {
    fileName: string;
    tableName: string;
    updateDbConfigField: (field: any, value: any) => void;
    updateUiStateField: (field: any, value: any) => void;
}

const FileUpload: React.FC<FileUploadProps> = (props: FileUploadProps) => {

    const [fileSize, setFileSize] = React.useState<string>('');

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: false,
                directory: false,
            });

            if (selectedFilePath && selectedFilePath !== props.fileName) {
                const path: string = selectedFilePath?.toString();
                const normalizedTableName: string = getNormalizedTableName(path);

                props.updateUiStateField('fileName', getFileNameFromPath(path));
                props.updateUiStateField('filePath', path);
                props.updateDbConfigField('tableName', normalizedTableName);

                const response = await invoke('get_size_of_file', {filePath: path});
                setFileSize(typeof response === 'string' ? response : '');
            }
        } catch (error) {
            toast.error(`Error opening file`);
        }
    };

    const getFileNameFromPath = (path: string) => path.split('/').pop() || '';

    const getNormalizedTableName = (path: string) => {
        const fileName = getFileNameFromPath(path).split('.').shift() || '';
        return fileName
            .replace(/[^a-zA-Z0-9_]/g, '')
            .replace(/^_/, '').toLowerCase();
    };

    return (
        <div className="flex items-center gap-4">
            <Button type="button" onClick={openFileDialog} className="bg-blue-500 hover:bg-blue-600">
                <FileArchive/>
            </Button>
            <Input
                type="text"
                value={props.fileName ? `${props.fileName} (${fileSize})` : ''}
                placeholder="Select a CSV file"
                disabled
                className="w-full"
            />
        </div>
    );
};

export default FileUpload;
