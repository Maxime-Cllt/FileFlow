import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {FileArchive} from "lucide-react";
import {invoke} from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog"
import {getFileNameFromPath, log_error} from "@/components/hooks/utils.tsx";

interface FileUploadProps {
    filesPath: string[];
    setFilePath: (path: string[]) => void
    multiple?: boolean;
}

const FileUpload: React.FC<FileUploadProps> = (props: FileUploadProps) => {

    const [message, setMessage] = React.useState<string>('');

    const openFileDialog = async (): Promise<void> => {
        try {
            const selectedFilePath: string | null = await dialog.open({
                filters: [{name: 'CSV Files', extensions: ['csv']}],
                multiple: props.multiple || false,
                directory: false,
            });

            if (!selectedFilePath) {
                return;
            }

            if (Array.isArray(selectedFilePath)) {
                let message: string = '';
                for (const filePath of selectedFilePath) {

                    const fileName: string = getFileNameFromPath(filePath?.toString() || '');
                    const fileSize: string = await invoke<string>('get_size_of_file', {filePath: filePath?.toString() || ''});
                    message += `${fileName} (${fileSize})`;

                    if (filePath !== selectedFilePath[selectedFilePath.length - 1]) {
                        message += ', ';
                    }
                }
                setMessage(message);
                props.setFilePath(selectedFilePath);
            } else {
                const fileName: string = getFileNameFromPath(selectedFilePath);
                const fileSize: string = await invoke<string>('get_size_of_file', {filePath: selectedFilePath});
                setMessage(`${fileName} (${fileSize})`);
                props.setFilePath([selectedFilePath]);
            }
        } catch (error) {
            log_error(error);
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
                    value={message}
                    placeholder={'Select a CSV file'}
                    disabled
                    className="w-full bg-gray-100 border border-gray-300 rounded-md p-3 text-gray-700"
                />
            </div>
        </div>
    );
};

export default FileUpload;
