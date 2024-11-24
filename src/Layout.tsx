import {Outlet} from "react-router-dom";
import Menu from "@/components/fileflowui/style/Menu.tsx";

const Layout = () => {
    return (
        <>
            <Menu/>
            <Outlet/>
        </>
    );
};

export default Layout;