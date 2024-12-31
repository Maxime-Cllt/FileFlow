import React from 'react';
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group.tsx";
import {Label} from "@/components/ui/label.tsx";

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
                <div className="space-x-2">
                    <RadioGroupItem value="fast" id="r1"/>
                    <Label htmlFor="r1">Fast Insertion</Label>
                </div>
                <div className="space-x-2">
                    <RadioGroupItem value="optimized" id="r2"/>
                    <Label htmlFor="r2">Optimized Insertion</Label>
                </div>
            </RadioGroup>
        </div>
    );
};

export default ModeSelection;
