import * as React from "react"
import {ChevronsUpDown} from "lucide-react"

import {Button} from "@/components/ui/button.tsx"
import {Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,} from "@/components/ui/command.tsx"
import {Popover, PopoverContent, PopoverTrigger,} from "@/components/ui/popover.tsx"
import {Checkbox} from "@/components/ui/checkbox"

interface ComboItem {
    value: string
    label: string
}

interface CheckBoxComboProps {
    lists: Array<ComboItem>
    setSelectedValue: (values: string[]) => void
}

export function CheckBoxCombo(props: CheckBoxComboProps) {
    const [open, setOpen] = React.useState(false)
    const [selectedValues, setSelectedValues] = React.useState<string[]>([])

    const handleSelect = (currentValue: string) => {
        let updatedValues: string[]
        if (selectedValues.includes(currentValue)) {
            updatedValues = selectedValues.filter((val) => val !== currentValue)
        } else {
            updatedValues = [...selectedValues, currentValue]
        }
        setSelectedValues(updatedValues)
        props.setSelectedValue(updatedValues)
    }

    const updateText = (): string => {
        if (selectedValues.length <= 0) {
            return "Select items"
        }
        if (selectedValues.length === 1) {
            const selectedItem: ComboItem | undefined = props.lists.find(
                (item): boolean => item.value === selectedValues[0]
            )
            return selectedItem ? selectedItem.label : "Select items"
        }
        if (selectedValues.length > 1) {
            return `${selectedValues.length} items selected`
        }
        return ""
    }

    const deselectAll = () => {
        setSelectedValues([])
        props.setSelectedValue([])
    }

    const selectAll = () => {
        const allValues: string[] = props.lists.map((item) => item.value)
        setSelectedValues(allValues)
        props.setSelectedValue(allValues)
    }

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[250px] justify-between"
                >
                    {updateText()}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[400px] p-0">
                <Command>
                    <CommandInput placeholder="Search item..."/>
                    <CommandList>
                        <CommandEmpty>No item found.</CommandEmpty>
                        <CommandGroup>
                            {props.lists.map((item) => (
                                <CommandItem
                                    key={item.value}
                                    value={item.value}
                                    onSelect={(currentValue) => handleSelect(currentValue)}
                                >
                                    <Checkbox
                                        checked={selectedValues.includes(item.value)}
                                        onCheckedChange={() => handleSelect(item.value)}
                                        className="mr-2"
                                    />
                                    {item.label}
                                </CommandItem>
                            ))}
                        </CommandGroup>
                    </CommandList>
                    <div className="p-2 border-t border-gray-200 flex justify-center">
                        <Button variant="ghost" size="sm" onClick={deselectAll}>
                            Deselect All
                        </Button>
                        <Button variant="ghost" size="sm" onClick={selectAll}>
                            Select All
                        </Button>
                    </div>
                </Command>
            </PopoverContent>
        </Popover>
    )
}
