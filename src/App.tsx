import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import AboutComponent from "@/components/fileflowui/other/About.tsx";
import Layout from "@/Layout.tsx";
import Insert from "@/components/fileflowui/load/insert/Insert.tsx";
import Download from "@/components/fileflowui/extract/download/Download.tsx";
import HelpInsert from "@/components/fileflowui/help/HelpInsert.tsx";
import HelpDownload from "@/components/fileflowui/help/HelpDownload.tsx";


function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Layout/>}>
                    <Route path="" element={<Insert/>}/>
                    <Route path="help-insert" element={<HelpInsert/>}/>
                    <Route path="help-download" element={<HelpDownload/>}/>
                    <Route path="about" element={<AboutComponent/>}/>
                    <Route path="download" element={<Download/>}/>
                </Route>

            </Routes>
        </Router>
    );
}

export default App;
