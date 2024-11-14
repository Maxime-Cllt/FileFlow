import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import Home from './components/fileflowui/home/Home.tsx';
import AboutComponent from "@/components/fileflowui/About.tsx";
import LoadDataSql from "@/components/fileflowui/load/LoadDataSql.tsx";

function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Home/>}/>
                <Route path="/load" element={<LoadDataSql/>}/>
                <Route path="/about" element={<AboutComponent/>}/>
            </Routes>
        </Router>
    );
}

export default App;
