import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarTrigger
} from "@/components/ui/menubar.tsx";
import React from "react";
import {invoke} from "@tauri-apps/api/tauri";


interface MenuProps {
    addLog: (message: string) => void;
    saveConfig: (e: React.MouseEvent) => Promise<void>;
    loadConfig: (e: React.MouseEvent) => Promise<void>;
}

const Menu: React.FC<MenuProps> = (props: MenuProps) => {

    const handleDeconnection = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response: unknown = await invoke('disconnect_from_database');
            props.addLog(response as string);
        } catch (error) {
            props.addLog(error as string);
        }
    };

    return (
        <div>
            <Menubar>
                <MenubarMenu>
                    <MenubarTrigger>Databases</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={handleDeconnection}>Disconnect</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>Configuration</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={props.saveConfig}>Save this</MenubarItem>
                        <MenubarItem onClick={props.loadConfig}>Import</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>Other</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={(): string => window.location.href = "/"}>Home</MenubarItem>
                        <MenubarItem onClick={(): string => window.location.href = "/about"}>About</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    );
};

export default Menu;