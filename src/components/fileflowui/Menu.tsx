import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarTrigger
} from "@/components/ui/menubar.tsx";
import React from "react";

interface MenuProps {
    saveConfig?: (e: React.MouseEvent) => Promise<void>;
    loadConfig?: (e: React.MouseEvent) => Promise<void>;
    handleDeconnection?: (e: React.MouseEvent) => Promise<void>;
}

const Menu: React.FC<MenuProps> = ({
                                       saveConfig = async () => {
                                       },
                                       loadConfig = async () => {
                                       },
                                       handleDeconnection = async () => {
                                       },

                                   }) => {

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
                        <MenubarItem onClick={saveConfig}>Save this</MenubarItem>
                        <MenubarItem onClick={loadConfig}>Import</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>Other</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={() => (window.location.href = "/")}>Home</MenubarItem>
                        <MenubarItem onClick={() => (window.location.href = "/about")}>About</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    );
};

export default Menu;