import {Outlet} from "react-router-dom";
import Menu from "@/components/hooks/Menu.tsx";

const Layout = () => {
    return (
        <div className="min-h-screen flex flex-col">
            <Menu/>
            <main className="flex-1 pt-16 px-4 md:px-8 pb-8">
                <Outlet/>
            </main>
        </div>
    );
};

export default Layout;