import React, {useEffect, useState} from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog.tsx";
import {Button} from "@/components/ui/button.tsx";
import {ArrowDownFromLine, Trash2} from "lucide-react";
import {toast} from "sonner";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip.tsx";

interface ConfigItemListProps {
    title: string;
    description: string;
    list: Array<Item>;
    onItemSelect: (item: Item) => void;
    onItemDelete: (item: Item) => void;
}

const ConfigItemList: React.FC<ConfigItemListProps> = (props: ConfigItemListProps) => {
    const [items, setItems] = useState<Item[]>([]);

    useEffect(() => {
        if (Array.isArray(props.list)) {
            setItems(props.list);
        } else {
            toast.error('Error parsing saved configurations');
            setItems([]);
        }
    }, [props.list]);

    const handleDeleteItem = (event: React.MouseEvent, item: Item) => {
        event.stopPropagation();
        setItems(items.filter((i) => i.id !== item.id));
        props.onItemDelete(item);
    };

    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    <button
                        aria-label="Open Item List"
                        title="Open Item List"
                        type="button"
                        className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-blue-500 hover:bg-blue-600 text-white"
                    >
                        <ArrowDownFromLine className="w-5 h-5"/>
                    </button>
                </DialogTrigger>

                {/* Dialog Content */}
                <DialogContent className="sm:max-w-[700px]">
                    <DialogHeader>
                        <DialogTitle>{props.title}</DialogTitle>
                        <DialogDescription>
                            {props.description}
                        </DialogDescription>
                    </DialogHeader>

                    {/* Scrollable Item List */}
                    <div className="flex flex-col mt-4 max-h-64 overflow-y-auto border border-gray-300 rounded-md">
                        {items.length > 0 ? (
                            items.map((item) => (
                                <DialogClose asChild key={item.id}>
                                    <div
                                        key={item.id}
                                        className="flex justify-between items-center p-3 border-b last:border-b-0 hover:bg-indigo-50 cursor-pointer transition-colors"
                                        onClick={() => props.onItemSelect(item)}
                                    >
                                        <div className="flex-1">
                                            <span className="text-indigo-800 font-medium">{item.id}</span>
                                        </div>

                                        <TooltipProvider>
                                            <Tooltip>
                                                <TooltipTrigger>
                                                    <Button
                                                        variant="destructive"
                                                        size="icon"
                                                        onClick={(e) => handleDeleteItem(e, item)}
                                                        className="ml-2 bg-red-600 hover:bg-red-700 text-white"
                                                    >
                                                        <Trash2 className="w-4 h-4"/>
                                                    </Button>
                                                </TooltipTrigger>
                                                <TooltipContent>
                                                    Delete item "{item.id}"
                                                </TooltipContent>
                                            </Tooltip>
                                        </TooltipProvider>
                                    </div>
                                </DialogClose>
                            ))
                        ) : (
                            <p className="text-gray-500 text-center py-4">No configurations found.</p>
                        )}
                    </div>

                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4 mt-6 pt-4 border-t border-gray-100">
                        <DialogClose asChild>
                            <Button
                                variant="outline"
                                className="px-8 py-2.5 border border-slate-200 text-slate-600 hover:text-slate-800 hover:bg-slate-50 hover:border-slate-300 transition-all duration-200 font-medium rounded-lg shadow-sm"
                                type="button"
                            >
                                Close
                            </Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    );
};

export default ConfigItemList;
