import React, {useState} from "react";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {SaveAll} from "lucide-react";
import {log_error} from "@/components/hooks/utils.tsx";

interface SaveConfigDialogProps {
    fonction: (e: React.FormEvent) => void;
    message_text: string;
    updateMessage: (message: string) => void;
}

const InputTextDialog: React.FC<SaveConfigDialogProps> = (props: SaveConfigDialogProps) => {
    const [open, setOpen] = useState(false);

    const executeFunction = (e: React.MouseEvent) => {
        e.preventDefault();
        try {
            props.fonction(e);
            setOpen(false);
        } catch (error) {
            log_error(error);
        }
    };

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            {/* Trigger Button */}
            <DialogTrigger asChild>
                <button
                    aria-label="Save Config"
                    title="Save Config"
                    className="flex items-center justify-center p-3 rounded-full shadow-md transition duration-300 bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white"
                >
                    <SaveAll className="w-5 h-5"/>
                </button>
            </DialogTrigger>

            {/* Dialog Content */}
            <DialogContent className="sm:max-w-lg rounded-lg shadow-xl border border-gray-200">
                <DialogHeader className="border-b border-gray-200 pb-3 mb-4">
                    <DialogTitle className="text-xl font-semibold text-gray-800">
                        Save Configuration
                    </DialogTitle>
                    <DialogDescription className="text-gray-600">
                        Enter a name for the configuration you want to save.
                    </DialogDescription>
                </DialogHeader>

                {/* Dialog Body */}
                <div className="flex flex-col gap-4">
                    <input
                        type="text"
                        className="w-full p-3 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400 transition"
                        placeholder="Configuration name"
                        value={props.message_text}
                        autoComplete="off"
                        onChange={(e) => props.updateMessage(e.target.value)}
                    />
                </div>

                {/* Dialog Footer */}
                <DialogFooter className="flex justify-end mt-6 space-x-4">
                    <Button
                        variant="outline"
                        className="px-4 py-2 bg-red-500 hover:bg-red-600 text-white hover:text-white"
                        type="button"
                        onClick={() => setOpen(false)}
                    >
                        Cancel
                    </Button>
                    <Button
                        className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white"
                        type="button"
                        onClick={executeFunction}
                    >
                        Save
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
};

export default InputTextDialog;