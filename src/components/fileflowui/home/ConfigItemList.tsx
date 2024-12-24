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
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {ArrowDownFromLine, Trash2} from "lucide-react";
import {toast} from "sonner";

interface ConfigItemListProps {
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
            toast.error("Error parsing saved configurations");
            setItems([]);
        }
    }, [props.list]);

    const deleteItem = (event: React.MouseEvent, item: Item) => {
        event.stopPropagation();
        setItems(items.filter((i: Item) => i.id !== item.id));
        props.onItemDelete(item);
    }


    return (
        <div>
            <Dialog>
                {/* Trigger Button */}
                <DialogTrigger asChild>
                    <button
                        aria-label="Open Item List"
                        title="Open Item List"
                        type={"button"}
                        className="flex items-center justify-center p-3 rounded-full shadow-lg transition duration-300 bg-blue-500 hover:bg-blue-600 text-white"
                    >
                        <ArrowDownFromLine className="w-5 h-5"/>
                    </button>
                </DialogTrigger>

                {/* Dialog Content */}
                <DialogContent className="sm:max-w-[700px]">
                    <DialogHeader>
                        <DialogTitle>Your Saved Configs</DialogTitle>
                        <DialogDescription>
                            Select a config to load or manage your saved configurations.
                        </DialogDescription>
                    </DialogHeader>

                    {/* Item List */}
                    <div className="flex flex-col gap-4 mt-4">
                        {items.length > 0 ? (
                            items.map((item) => (
                                <DialogClose asChild>

                                    <div
                                        key={item.id}
                                        className="flex justify-between items-center p-3 border border-gray-300 rounded-md hover:bg-gray-100"
                                    >
                                        {/* Make the clickable part separate */}
                                        <div
                                            className="flex-1 cursor-pointer"
                                            onClick={() => props.onItemSelect(item)}
                                        >
                                            <span>{item.id}</span>
                                        </div>

                                        {/* Delete button */}
                                        <Button
                                            className="bg-red-500 hover:bg-red-600 text-white focus:ring-4 focus:ring-red-300"
                                            onClick={(e) => deleteItem(e, item)}
                                            type={"button"}
                                        >
                                            <Trash2 className="w-4 h-4 mr-2"/>
                                            Delete
                                        </Button>
                                    </div>
                                </DialogClose>
                            ))
                        ) : (
                            <p className="text-gray-500">No configurations found.</p>
                        )}
                    </div>


                    {/* Dialog Footer */}
                    <DialogFooter className="flex justify-end gap-4 mt-4">
                        <DialogClose asChild>
                            <Button
                                className="bg-gray-500 hover:bg-gray-600 text-white focus:ring-4 focus:ring-gray-300"
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
