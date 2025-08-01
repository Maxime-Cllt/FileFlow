<div align="center">
  <img src="/src-tauri/icons/icon.png" width="100px" height="100px" alt="FileFlow" align="center" />
  <h1>FileFlow</h1>
  <div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Tauri-ffc130?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri" />
    <img src="https://img.shields.io/badge/Version-1.0.4-7073f6?style=for-the-badge" alt="Version" />
  </div>
</div>

<div align="center" style="margin-top: 20px">
  <img src="/assets/Insert.png" alt="FileFlow Insert Mode" height="250px" width="auto" />
  <img src="/assets/Download.png" alt="FileFlow Download Mode" height="250px" width="auto" />
</div>

## 📖 About

FileFlow is a simple and easy-to-use tool that allows you to insert data from a CSV file directly into a database table.
With no special privileges required for data insertion, it streamlines the process while ensuring efficiency and
security.

Built with **Rust** and the **Tauri** framework, FileFlow is a **cross-platform** application available on **Windows**,
**macOS**, and **Linux**. 🚀

_Check out the [Release Section](#release) for the latest version of the application._

## 🌟 Features

- **Insert Data Easily**: Insert data into a **new table** or an **existing table** seamlessly.
- **Optimized Data Types**: Automatically optimize column types (e.g., `VARCHAR(MAX_LENGTH)`).
- **CSV File Support**: Directly insert data from CSV files.
- **No Privilege Required**: Operates without requiring any special database privileges.
- **Table Schema Export**: Download table schema as a CSV file for further analysis.

## 🗄️ Supported Databases

<div align="center">
  <img src="https://img.shields.io/badge/MySQL-00758F?style=for-the-badge&logo=mysql&logoColor=white" alt="MySQL" />
  <img src="https://img.shields.io/badge/MariaDB-003545?style=for-the-badge&logo=mariadb&logoColor=white" alt="MariaDB" />
  <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite" />
</div>

## ⚡ Installation

1. **Clone the Repository:**

```bash
git clone https://github.com/Maxime-Cllt/FileFlow.git
```

2. **Navigate to the Project Directory and Install Dependencies:**

```bash
cd FileFlow
pnpm install
```

3. **Build the Application:**

```bash
pnpm tauri build
```

4. **Run the Application in Development Mode:**

```bash
pnpm tauri dev
```

## 🚀 Getting Started

To quickly test FileFlow:

1. **Prepare Your CSV File**: Ensure your CSV file is formatted correctly.
2. **Configure Your Database Connection**: Use the built-in connection form to set up your database connection.
3. **Select Insertion Mode**: Choose between Optimized Mode and Fast Mode based on your needs.
4. **Upload and Insert**: Upload your CSV file and start the insertion process. Monitor progress with the on-screen
   loader.

## 🧪 Code Quality

### Unit Tests available

To run unit tests, use the following command:

```bash
cargo test
```

### Benchmarks available

To run benchmarks, use the following command:

```bash
cargo bench
```

## 🤝 Contributing

Contributions are welcome! To contribute:

- **Fork the Repository**
- **Create a Feature Branch**:
  ```bash
  git checkout -b feature/your-feature-name
