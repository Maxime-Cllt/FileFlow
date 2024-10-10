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

const Menu: React.FC<MenuProps> = ({addLog, saveConfig, loadConfig}) => {

    const handleDeconnection = async (e: { preventDefault: () => void; }) => {
        e.preventDefault();
        try {
            const response = await invoke('disconnect_from_database');
            addLog(response as string);
        } catch (error) {
            addLog(`Erreur de connexion: ${error}`);
        }
    };

    return (
        <div>
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