import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
import React from "react";
import {Link} from "react-router-dom";


const Menu: React.FC = () => {
    return (
        <div className={'fixed top-0 w-full z-50'}>
            <Menubar>

                {/* Upload Menu */}
                <MenubarMenu>
                    <Link to={"/download"}>
                    <MenubarTrigger className={"cursor-pointer"}>Extract</MenubarTrigger>
                    </Link>
                    {/*<MenubarContent>*/}
                    {/*    <Link to={"/download"}>*/}
                    {/*        <MenubarItem>Download</MenubarItem>*/}
                    {/*    </Link>*/}
                    {/*</MenubarContent>*/}
                </MenubarMenu>

                {/* Upload Menu */}
                <MenubarMenu>
                    <Link to={"/"}>
                        <MenubarTrigger className={"cursor-pointer"}>Load</MenubarTrigger>
                    </Link>
                    {/*<MenubarContent>*/}
                    {/*    /!* Download menu items *!/*/}
                    {/*    <Link to={"/"}>*/}
                    {/*        <MenubarItem>Upload</MenubarItem>*/}
                    {/*    </Link>*/}
                    {/*</MenubarContent>*/}
                </MenubarMenu>

                {/* Help Menu */}
                <MenubarMenu>
                    <MenubarTrigger className={"cursor-pointer"}>Help</MenubarTrigger>
                    <MenubarContent>

                        <Link to={"/help-insert"}>
                            <MenubarItem>Help for Insert</MenubarItem>
                        </Link>

                        <Link to={"/help-download"}>
                            <MenubarItem>Help for Download</MenubarItem>
                        </Link>
                    </MenubarContent>
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
