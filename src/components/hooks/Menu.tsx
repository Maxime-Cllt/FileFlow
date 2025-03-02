import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
import React from "react";
import {Link} from "react-router-dom";


const Menu: React.FC = () => {
    return (
        <div className={'fixed top-0 w-full z-50'}>
            <Menubar>

                {/* Upload Menu */}
                <MenubarMenu>
                    <MenubarTrigger className={"cursor-pointer"}>Extract</MenubarTrigger>
                    <MenubarContent>

                        <Link to={"/download"}>
                            <MenubarItem>Download</MenubarItem>
                        </Link>
                    </MenubarContent>
                </MenubarMenu>

                {/* Upload Menu */}
                <MenubarMenu>
                    <MenubarTrigger className={"cursor-pointer"}>Load</MenubarTrigger>
                    <MenubarContent>
                        {/* Download menu items */}
                        <Link to={"/"}>
                            <MenubarItem>Upload</MenubarItem>
                        </Link>

                        {/* Load menu items */}
                        <Link to={"/upload"}>
                            <MenubarItem>Load</MenubarItem>
                        </Link>

                    </MenubarContent>
                </MenubarMenu>

                {/* Help Menu */}
                <MenubarMenu>
                    <Link to={"/help"}>
                        <MenubarTrigger className={"cursor-pointer"}>Help</MenubarTrigger>
                    </Link>
                </MenubarMenu>

                {/* About Menu */}
                <MenubarMenu>
                    <Link to={"/about"}>
                        <MenubarTrigger className={"cursor-pointer"}>About</MenubarTrigger>
                    </Link>
                </MenubarMenu>
            </Menubar>
        </div>
    );
};

export default Menu;
