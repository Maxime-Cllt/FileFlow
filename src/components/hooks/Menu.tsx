import {Menubar, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
import React from "react";
import {Link} from "react-router-dom";


const Menu: React.FC = () => {
    return (
        <div className={'fixed top-0 w-full z-50'}>
            <Menubar>

                {/* Home Menu */}
                <MenubarMenu>
                    <Link to={"/"}>
                        <MenubarTrigger className={"cursor-pointer"}>Insert</MenubarTrigger>
                    </Link>
                </MenubarMenu>

                {/* Load Menu */}
                <MenubarMenu>
                    <Link to={"/load"}>
                        <MenubarTrigger className={"cursor-pointer"}>Load</MenubarTrigger>
                    </Link>
                </MenubarMenu>

                {/* Load Menu */}
                <MenubarMenu>
                    <Link to={"/copy"}>
                        <MenubarTrigger className={"cursor-pointer"}>Copy</MenubarTrigger>
                    </Link>
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
