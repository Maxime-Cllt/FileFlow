import {Outlet} from "react-router-dom";
import Menu from "@/hooks/common/Menu.tsx";

const Layout = () => {
    return (
        <>
            <Menu/>
            <Outlet/>
        </>
    );
};

export default Layout;