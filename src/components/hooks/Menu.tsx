import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "@/components/ui/menubar.tsx";
import {Sheet, SheetContent, SheetTrigger} from "@/components/ui/sheet";
import {Menu as MenuIcon} from "lucide-react";
import {Button} from "@/components/ui/button";
import React, {useState} from "react";
import {Link} from "react-router-dom";

const Menu: React.FC = () => {
    const [open, setOpen] = useState(false);

    return (
        <>
            {/* Desktop Menu */}
            <div className="hidden md:block fixed top-0 w-full z-50 border-b bg-background">
                <Menubar className="rounded-none border-none">
                    <MenubarMenu>
                        <Link to={"/download"}>
                            <MenubarTrigger className={"cursor-pointer"}>Extract</MenubarTrigger>
                        </Link>
                    </MenubarMenu>

                    <MenubarMenu>
                        <Link to={"/"}>
                            <MenubarTrigger className={"cursor-pointer"}>Load</MenubarTrigger>
                        </Link>
                    </MenubarMenu>

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

                    <MenubarMenu>
                        <Link to={"/about"}>
                            <MenubarTrigger className={"cursor-pointer"}>About</MenubarTrigger>
                        </Link>
                    </MenubarMenu>
                </Menubar>
            </div>

            {/* Mobile Menu */}
            <div className="md:hidden fixed top-0 w-full z-50 border-b bg-background px-4 py-3">
                <div className="flex items-center justify-between">
                    <span className="text-lg font-semibold">FileFlow</span>
                    <Sheet open={open} onOpenChange={setOpen}>
                        <SheetTrigger asChild>
                            <Button variant="ghost" size="icon">
                                <MenuIcon className="h-6 w-6" />
                                <span className="sr-only">Toggle menu</span>
                            </Button>
                        </SheetTrigger>
                        <SheetContent side="left" className="w-64">
                            <nav className="flex flex-col space-y-4 mt-8">
                                <Link
                                    to="/"
                                    onClick={() => setOpen(false)}
                                    className="text-lg font-medium hover:text-primary transition-colors"
                                >
                                    Load
                                </Link>
                                <Link
                                    to="/download"
                                    onClick={() => setOpen(false)}
                                    className="text-lg font-medium hover:text-primary transition-colors"
                                >
                                    Extract
                                </Link>

                                <div className="border-t pt-4">
                                    <p className="text-sm font-semibold text-muted-foreground mb-2">Help</p>
                                    <Link
                                        to="/help-insert"
                                        onClick={() => setOpen(false)}
                                        className="block py-2 text-base hover:text-primary transition-colors"
                                    >
                                        Help for Insert
                                    </Link>
                                    <Link
                                        to="/help-download"
                                        onClick={() => setOpen(false)}
                                        className="block py-2 text-base hover:text-primary transition-colors"
                                    >
                                        Help for Download
                                    </Link>
                                </div>

                                <Link
                                    to="/about"
                                    onClick={() => setOpen(false)}
                                    className="text-lg font-medium hover:text-primary transition-colors pt-2"
                                >
                                    About
                                </Link>
                            </nav>
                        </SheetContent>
                    </Sheet>
                </div>
            </div>
        </>
    );
};

export default Menu;
