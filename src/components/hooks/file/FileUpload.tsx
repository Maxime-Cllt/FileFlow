import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {FileArchive} from "lucide-react";
import {invoke} from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog"
import {toast} from "sonner";
import {getFileNameFromPath} from "@/components/hooks/utils.tsx";

interface FileUploadProps {
    filePath: string;
    setFilePath: (path: string) => void;
}

const FileUpload: React.FC<FileUploadProps> = (props: FileUploadProps) => {

    const [fileSize, setFileSize] = React.useState<string>('');

    const [fileName, setFileName] = React.useState<string>('');

    const openFileDialog = async () => {
        try {
            const selectedFilePath = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: false,
                directory: false,
            });

            setFileName(getFileNameFromPath(selectedFilePath?.toString() || ''));

            if (selectedFilePath && selectedFilePath !== fileName) {
                const path: string = selectedFilePath?.toString();

                props.setFilePath(path)

                const response = await invoke<string | boolean>('get_size_of_file', {filePath: path});

                if (typeof response !== 'string') {
                    throw new Error('Error getting file size');
                }

                setFileSize(response);
            }
        } catch (error) {
            toast.error(`Error opening file`);
        }
    };

    return (
        <div className="flex items-center justify-between space-x-4 p-4 ">
            {/* File Upload Button */}
            <Button
                type="button"
                onClick={openFileDialog}
                className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md shadow-md flex items-center gap-2"
            >
                <FileArchive className="w-5 h-5"/>
                {'Select CSV'}
            </Button>

            {/* Display file info */}
            <div className="flex-1 ml-4">
                <Input
                    type="text"
                    value={fileName ? `${fileName} (${fileSize})` : ''}
                    placeholder={'Select a CSV file'}
                    disabled
                    className="w-full bg-gray-100 border border-gray-300 rounded-md p-3 text-gray-700"
                />
            </div>
        </div>
    );
};

export default FileUpload;
