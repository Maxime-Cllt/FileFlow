import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import AboutComponent from "@/components/fileflowui/other/About.tsx";
import HelpComponent from "@/components/fileflowui/other/Help.tsx";
import Layout from "@/Layout.tsx";
import Insert from "@/components/fileflowui/load/insert/Insert.tsx";
import LoadDataSql from "@/components/fileflowui/load/upload/LoadDataSql.tsx";
import Download from "@/components/fileflowui/extract/download/Download.tsx";


function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Layout/>}>
                    <Route path="" element={<Insert/>}/>
                    <Route path="upload" element={<LoadDataSql/>}/>
                    <Route path="help" element={<HelpComponent/>}/>
                    <Route path="about" element={<AboutComponent/>}/>
                    <Route path="download" element={<Download/>}/>
                </Route>

            </Routes>
        </Router>
    );
}

export default App;
