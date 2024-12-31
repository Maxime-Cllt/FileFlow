import React from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {SaveAll} from "lucide-react";

interface SaveConfigDialogProps {
    fonction: (e: React.FormEvent) => void;
    message_text: string;
    updateMessage: (message: string) => void;
}

const InputTextDialog: React.FC<SaveConfigDialogProps> = (props: SaveConfigDialogProps) => {

    const executeFunction = (e: React.MouseEvent) => {
        e.preventDefault();
        props.fonction(e);
    }

    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    <button
                        aria-label="Load Config"
                        title="Load Config"
                        className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-green-500 hover:bg-green-600 text-white"
                    >
                        <SaveAll className="w-5 h-5"/>
                    </button>
                </DialogTrigger>

                {/* Dialog Content */}
                <DialogContent className="sm:max-w-[700px]">
                    <DialogHeader>
                        <DialogTitle>Enter the configuration name</DialogTitle>
                        <DialogDescription>
                            Choose a name for the configuration you want to save
                        </DialogDescription>
                    </DialogHeader>

                    {/* Dialog Body */}
                    <div className="flex flex-col gap-4">
                        <input
                            type="text"
                            className="p-2 border border-gray-300 rounded-md focus:ring-4 focus:ring-blue-300"
                            placeholder="Configuration name"
                            value={props.message_text}
                            autoComplete={"off"}
                            onChange={(e) => props.updateMessage(e.target.value)}
                        />
                    </div>

                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4 mt-4">
                        <div>
                            <DialogClose>
                                <Button
                                    className="bg-red-500 hover:bg-red-600 text-white focus:ring-4 focus:ring-red-300"
                                    type={"button"}
                                >
                                    Cancel
                                </Button>
                            </DialogClose>
                        </div>

                        <div onClick={executeFunction}>
                            <DialogClose>
                                <div>
                                    <Button
                                        className="bg-blue-500 hover:bg-blue-600 text-white focus:ring-4 focus:ring-blue-300"
                                        type={"button"}
                                    >
                                        Save
                                    </Button>
                                </div>
                            </DialogClose>
                        </div>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default InputTextDialog;
