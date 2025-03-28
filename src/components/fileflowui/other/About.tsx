const AboutPage = () => {

    return (
        <div className="h-full w-full">

            <div
                className="container mx-auto pt-8 px-4 md:px-8 mt-16 space-y-6 bg-white shadow-2xl rounded-xl overflow-hidden">

                {/* Header Section */}
                <div className="text-center">
                    <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 text-transparent bg-clip-text">
                        About FileFlow
                    </h1>
                    <p className="mt-4 text-lg text-gray-600">Version: 1.0.1</p>
                </div>

                {/* Features & Roadmap Section */}
                <div className="border-t border-gray-200">
                    <div className="p-8 grid grid-cols-1 md:grid-cols-2 gap-8">
                        {/* Features Section */}
                        <div>
                            <h2 className="text-2xl font-semibold text-gray-800 mb-4">Features</h2>
                            <ul className="list-disc ml-6 text-gray-700 space-y-2">
                                <li>Fast and secure data processing powered by Tauri.</li>
                                <li>User-friendly interface with intuitive navigation.</li>
                                <li>Seamless integration with modern web technologies.</li>
                                <li>Customizable settings to match your workflow.</li>
                                <li>Efficient performance and low resource usage.</li>
                            </ul>
                        </div>

                        {/* Roadmap Section */}
                        <div>
                            <h2 className="text-2xl font-semibold text-gray-800 mb-4">Roadmap</h2>
                            <p className="text-gray-700">
                                We are constantly evolving! Upcoming enhancements include:
                            </p>
                            <ul className="list-disc ml-6 text-gray-700 mt-2 space-y-2">
                                <li>More customization and personalization options.</li>
                                <li>Improved performance and stability features.</li>
                                <li>Integration with additional third-party services.</li>
                                <li>New user-requested functionalities and optimizations.</li>
                            </ul>
                        </div>
                    </div>
                </div>

                {/* Credits & Contact Section */}
                <div className="border-t border-gray-200">
                    <div className="p-8 grid grid-cols-1 gap-8">
                        {/* Credits Section */}
                        <div>
                            <h2 className="text-2xl font-semibold text-gray-800 mb-4">Credits</h2>
                            <p className="text-gray-700">
                                This application is built by a dedicated team of developers and the open-source
                                community.
                                Special thanks to everyone who contributed to its development.
                            </p>
                        </div>
                    </div>
                </div>

                {/* Resources Section */}
                <div className="border-t border-gray-200">
                    <div className="p-8 text-center">
                        <h2 className="text-2xl font-semibold text-gray-800 mb-4">Learn More</h2>
                        <div className="space-y-2">
                            <a
                                href="https://tauri.app/"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-blue-600 hover:underline block"
                            >
                                Tauri Documentation
                            </a>
                            <a
                                href="https://github.com/Maxime-Cllt/FileFlow"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-blue-600 hover:underline block"
                            >
                                GitHub Repository
                            </a>
                            <a
                                href="https://opensource.org/licenses/MIT"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-blue-600 hover:underline block"
                            >
                                License Information (MIT)
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AboutPage;
