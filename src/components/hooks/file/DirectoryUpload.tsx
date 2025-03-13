import React from 'react';
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {FileArchive} from "lucide-react";
import * as dialog from "@tauri-apps/plugin-dialog"
import {toast} from "sonner";

interface FileUploadProps {
    absolutePath: string;
    setAbsolutePath: (path: string) => void;
}

const DirectoryUpload: React.FC<FileUploadProps> = (props: FileUploadProps) => {


    const openFolderDialog = async () => {
        try {
            const selectedDirectoryPath = await dialog.open({
                multiple: false,
                directory: true,
            });


            if (selectedDirectoryPath && selectedDirectoryPath !== props.absolutePath) {
                const path: string = selectedDirectoryPath.toString();
                props.setAbsolutePath(path);
            }
        } catch (error) {
            toast.error(`Error opening directory`);
        }
    };

    return (
        <div className="flex items-center justify-between space-x-4 p-4 ">
            {/* Folder Upload Button */}
            <Button
                type="button"
                onClick={openFolderDialog}
                className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md shadow-md flex items-center gap-2"
            >
                <FileArchive className="w-5 h-5"/>
                {'Choose folder'}
            </Button>

            {/* Display file info */}
            <div className="flex-1 ml-4">
                <Input
                    type="text"
                    value={props.absolutePath}
                    placeholder={'Select a directory'}
                    className="w-full bg-gray-100 border border-gray-300 rounded-md p-3 text-gray-700"
                />
            </div>
        </div>
    );
};

export default DirectoryUpload;
