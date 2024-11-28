import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
import React from "react";
import {Link} from "react-router-dom";


const Menu: React.FC = () => {
    return (
        <div>
            <Menubar>

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
