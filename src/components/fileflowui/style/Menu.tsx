import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarTrigger
} from "@/components/ui/menubar.tsx";
import React from "react";
import {Link} from "react-router-dom";
import {toast} from "sonner";

interface MenuProps {
    saveConfig?: (e: React.MouseEvent) => Promise<void>;
    loadConfig?: (e: React.MouseEvent) => Promise<void>;
    handleDeconnection?: (e: React.MouseEvent) => Promise<void>;
}

const Menu: React.FC<MenuProps> = ({
                                       saveConfig = async () => {
                                             toast.warning("No configuration to save here");
                                       },
                                       loadConfig = async () => {
                                           toast.warning("No configuration to load here");
                                       },
                                       handleDeconnection = async () => {
                                           toast.warning("You are not connected to any database");
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
                        <Link to={"/"}>
                            <MenubarItem className="cursor-pointer">Home</MenubarItem>
                        </Link>
                        <Link to={"/load"}>
                            <MenubarItem className="cursor-pointer">Load</MenubarItem>
                        </Link>
                        <Link to={"/help"}>
                            <MenubarItem className="cursor-pointer">Help</MenubarItem>
                        </Link>
                        <Link to={"/about"}>
                            <MenubarItem className="cursor-pointer">About</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>

            </Menubar>
        </div>
    );
};

export default Menu;