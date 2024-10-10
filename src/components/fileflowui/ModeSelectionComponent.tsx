import React from 'react';
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group.tsx";
import {Label} from "@/components/ui/label.tsx";

interface ModeSelectionProps {
    setMode: (mode: string) => void;
}

const ModeSelectionComponent: React.FC<ModeSelectionProps> = ({setMode}) => {
    return (
        <div className="flex justify-center mt-10">
            <RadioGroup defaultValue="fast" className="flex justify-center gap-10"
                        onChange={(e) => {
                            // @ts-ignore
                            setMode(e.target.value as string);
                        }}>
                <div className="space-x-2">
                    <RadioGroupItem value="fast" id="r1"/>
                    <Label htmlFor="r1">Insertion rapide</Label>
                </div>
                <div className="space-x-2">
                    <RadioGroupItem value="optimized" id="r2"/>
                    <Label htmlFor="r2">Insertion optimisée</Label>
                </div>
            </RadioGroup>
        </div>
    );
};

export default ModeSelectionComponent;
