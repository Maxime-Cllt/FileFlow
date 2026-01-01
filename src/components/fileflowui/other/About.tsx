import {useEffect, useState} from "react";
import {get_app_version} from "@/components/hooks/utils.tsx";

const AboutPage = () => {
    const [appVersion, setAppVersion] = useState<string>('');

    useEffect(() => {
        const fetchVersion = async () => {
            const version = await get_app_version();
            setAppVersion(version);
        };
        fetchVersion();
    }, []);

    return (
        <div className="min-h-screen bg-gradient-to-b from-background to-muted/20">
            <div className="max-w-6xl mx-auto space-y-8">

                {/* Header Section */}
                <div className="text-center space-y-4 animate-fade-in">
                    <h1 className="text-5xl font-bold bg-gradient-to-r from-primary to-primary/60 text-transparent bg-clip-text">
                        FileFlow
                    </h1>
                    <p className="text-sm text-muted-foreground">Version {appVersion}</p>
                </div>

                {/* Features & Roadmap */}
                <div className="grid md:grid-cols-2 gap-6 animate-slide-up">
                    {/* Features Card */}
                    <div className="bg-card rounded-lg p-6 shadow-sm border border-border hover:shadow-md transition-shadow">
                        <h2 className="text-2xl font-semibold mb-4 text-foreground">Features</h2>
                        <ul className="space-y-3 text-muted-foreground">
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Fast and secure data processing powered by Tauri</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>User-friendly interface with intuitive navigation</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Seamless integration with modern web technologies</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Customizable settings to match your workflow</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Efficient performance and low resource usage</span>
                            </li>
                        </ul>
                    </div>

                    {/* Roadmap Card */}
                    <div className="bg-card rounded-lg p-6 shadow-sm border border-border hover:shadow-md transition-shadow">
                        <h2 className="text-2xl font-semibold mb-4 text-foreground">Roadmap</h2>
                        <p className="text-muted-foreground mb-3">
                            We are constantly evolving! Upcoming enhancements include:
                        </p>
                        <ul className="space-y-3 text-muted-foreground">
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>More customization and personalization options</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Improved performance and stability features</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>Integration with additional third-party services</span>
                            </li>
                            <li className="flex items-start">
                                <span className="text-primary mr-2">•</span>
                                <span>New user-requested functionalities and optimizations</span>
                            </li>
                        </ul>
                    </div>
                </div>

                {/* Credits */}
                <div className="bg-card rounded-lg p-6 shadow-sm border border-border">
                    <h2 className="text-2xl font-semibold mb-4 text-foreground">Credits</h2>
                    <p className="text-muted-foreground leading-relaxed">
                        This application is built by a dedicated team of developers and the open-source
                        community. Special thanks to everyone who contributed to its development.
                    </p>
                </div>

                {/* Resources */}
                <div className="bg-card rounded-lg p-6 shadow-sm border border-border">
                    <h2 className="text-2xl font-semibold mb-4 text-center text-foreground">Learn More</h2>
                    <div className="flex flex-wrap justify-center gap-4">
                        <a
                            href="https://tauri.app/"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-primary hover:underline transition-colors"
                        >
                            Tauri Documentation
                        </a>
                        <span className="text-muted-foreground">•</span>
                        <a
                            href="https://github.com/Maxime-Cllt/FileFlow"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-primary hover:underline transition-colors"
                        >
                            GitHub Repository
                        </a>
                        <span className="text-muted-foreground">•</span>
                        <a
                            href="https://opensource.org/licenses/MIT"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-primary hover:underline transition-colors"
                        >
                            MIT License
                        </a>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AboutPage;
