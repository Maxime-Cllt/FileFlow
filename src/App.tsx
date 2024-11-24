import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import Home from './components/fileflowui/home/Home.tsx';
import AboutComponent from "@/components/fileflowui/other/About.tsx";
import LoadDataSql from "@/components/fileflowui/load/LoadDataSql.tsx";
import HelpComponent from "@/components/fileflowui/other/Help.tsx";
import Layout from "@/Layout.tsx";


function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Layout/>}>
                    <Route path="" element={<Home/>}/>
                    <Route path="load" element={<LoadDataSql/>}/>
                    <Route path="help" element={<HelpComponent/>}/>
                    <Route path="about" element={<AboutComponent/>}/>
                </Route>

            </Routes>
        </Router>
    );
}

export default App;
