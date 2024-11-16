import {getName, getVersion} from "@tauri-apps/api/app";
import {invoke} from "@tauri-apps/api/core";
import Menu from "@/components/fileflowui/style/Menu.tsx";
import {useEffect, useState} from "react";

const AboutPage = () => {
    const [appName, setAppName] = useState("");
    const [appVersion, setAppVersion] = useState("");
    const [aboutInfo, setAboutInfo] = useState("");

    useEffect(() => {
        const fetchAppInfo = async () => {
            const name = await getName();
            const version = await getVersion();

            setAppName(name);
            setAppVersion(version);

            const info = await invoke("get_about_info");

            if (info) {
                setAboutInfo(info as string);
            }
        };

        fetchAppInfo().then(r => r);
    }, []);

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 text-gray-800 px-4">

            {/* Menu */}
            <div className="fixed top-0 w-full bg-white shadow-md z-10">
                <Menu/>
            </div>


            <div className="w-full max-w-2xl text-center">
                {/* App Title */}
                <h1 className="text-4xl font-bold mb-4">About {appName}</h1>
                <p className="text-lg mb-2">Version: {appVersion}</p>

                {/* About Information */}
                <p className="text-lg mb-6">{aboutInfo}</p>

                {/* Links to Resources */}
                <section className="mb-8">
                    <h2 className="text-2xl font-semibold mb-3">Learn More</h2>
                    <div className="space-y-2">
                        <a
                            href="https://tauri.app/"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:underline"
                        >
                            Tauri Documentation
                        </a>
                        <br/>
                        <a
                            href="https://github.com/Maxime-Cllt/FileFlow"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-blue-600 hover:underline"
                        >
                            GitHub Repository
                        </a>
                    </div>
                </section>
            </div>
        </div>
    );
};

export default AboutPage;
