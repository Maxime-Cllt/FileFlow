import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import AboutComponent from "@/components/fileflowui/other/About.tsx";
import LoadDataSql from "@/components/fileflowui/load/LoadDataSql.tsx";
import HelpComponent from "@/components/fileflowui/other/Help.tsx";
import Layout from "@/Layout.tsx";
import Insert from "./components/fileflowui/insert/Insert.tsx";


function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Layout/>}>
                    <Route path="" element={<Insert/>}/>
                    <Route path="load" element={<LoadDataSql/>}/>
                    <Route path="help" element={<HelpComponent/>}/>
                    <Route path="about" element={<AboutComponent/>}/>
                </Route>

            </Routes>
        </Router>
    );
}

export default App;
