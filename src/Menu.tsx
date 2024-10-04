import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarTrigger
} from "@/components/ui/menubar.tsx";
import React from "react";


interface MenuProps {
    handleDeconnection: (e: React.MouseEvent) => Promise<void>;
    saveConfig: (e: React.MouseEvent) => Promise<void>;
    loadConfig: (e: React.MouseEvent) => Promise<void>;
}

const Menu: React.FC<MenuProps> = ({handleDeconnection, saveConfig, loadConfig}) => {

    return (
        <div className="w-full">
            <Menubar>
                <MenubarMenu>
                    <MenubarTrigger>Databases</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={handleDeconnection}>Deconnection</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>Configuration</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={saveConfig}>Sauvegarder</MenubarItem>
                        <MenubarItem onClick={loadConfig}>Importer</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    );
};

export default Menu;