import React from 'react';
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Tooltip, TooltipContent, TooltipProvider, TooltipTrigger} from "@/components/ui/tooltip.tsx";

interface ModeSelectionProps {
    setMode: (mode: string) => void;
}

const ModeSelection: React.FC<ModeSelectionProps> = (props: ModeSelectionProps) => {
    return (
        <div className="flex justify-center mt-10">
            <RadioGroup defaultValue="fast" className="flex justify-center gap-10"
                        onValueChange={(e: string): void => {
                            if (e === "fast" || e === "optimized") {
                                props.setMode(e);
                            }
                        }
                        }>
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>
                            <div className="space-x-2">
                                <RadioGroupItem value="fast" id="r1"/>
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
                                <RadioGroupItem value="optimized" id="r2"/>
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
