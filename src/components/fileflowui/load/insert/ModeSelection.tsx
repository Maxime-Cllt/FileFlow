import React from 'react';
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip.tsx";
import {InsertionType} from "@/components/fileflowui/load/insert/Insert.tsx";

interface ModeSelectionProps {
    setMode: (mode: InsertionType) => void;
}

const ModeSelection: React.FC<ModeSelectionProps> = (props: ModeSelectionProps) => {
    return (
        <div className="flex justify-center mt-10">
            <RadioGroup defaultValue="fast" className="flex justify-center gap-10"
                        onValueChange={(e: string): void => {
                            if (e === InsertionType.Fast || e === InsertionType.Optimized) {
                                props.setMode(e);
                            }
                        }
                        }>
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <div className="space-x-2">
                                <RadioGroupItem value={InsertionType.Fast} id="r1"/>
                                <Label htmlFor="r1">Fast Insertion</Label>
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            Fast Insertion is the default mode for inserting data into the database. It is faster
                            but may not be as optimized as the optimized insertion mode.
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>

                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <div className="space-x-2">
                                <RadioGroupItem value={InsertionType.Optimized} id="r2"/>
                                <Label htmlFor="r2">Optimized Insertion</Label>
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            Optimized Insertion is a slower but more optimized mode for inserting data into the
                            database. It is recommended for large datasets.
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>

            </RadioGroup>
        </div>
    );
};

export default ModeSelection;
