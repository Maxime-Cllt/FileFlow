import {BrowserRouter as Router, Route, Routes,} from 'react-router-dom';
import Home from './Home';
import AboutComponent from "@/components/fileflowui/About.tsx";

function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Home/>}/>
                <Route path="/about" element={<AboutComponent/>}/>
            </Routes>
        </Router>
    );
}

export default App;
