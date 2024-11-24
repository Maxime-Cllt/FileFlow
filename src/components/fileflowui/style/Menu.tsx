import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
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
                {/* Database Menu */}
                <MenubarMenu>
                    <MenubarTrigger>Databases</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={handleDeconnection}>Disconnect</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>

                {/* Configuration Menu */}
                <MenubarMenu>
                    <MenubarTrigger>Configuration</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={saveConfig}>Save this</MenubarItem>
                        <MenubarItem onClick={loadConfig}>Import</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>

                {/* Home Menu */}
                <MenubarMenu>
                    <MenubarTrigger>Home</MenubarTrigger>
                    <MenubarContent>
                        <Link to={"/"}>
                            <MenubarItem className="cursor-pointer">Go to Home</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>

                {/* Load Menu */}
                <MenubarMenu>
                    <MenubarTrigger>Load</MenubarTrigger>
                    <MenubarContent>
                        <Link to={"/load"}>
                            <MenubarItem className="cursor-pointer">Load Data</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>

                {/* Help Menu */}
                <MenubarMenu>
                    <MenubarTrigger>Help</MenubarTrigger>
                    <MenubarContent>
                        <Link to={"/help"}>
                            <MenubarItem className="cursor-pointer">Help Section</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>

                {/* About Menu */}
                <MenubarMenu>
                    <MenubarTrigger>About</MenubarTrigger>
                    <MenubarContent>
                        <Link to={"/about"}>
                            <MenubarItem className="cursor-pointer">About Us</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    );
};

export default Menu;
