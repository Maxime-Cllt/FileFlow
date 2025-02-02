<div align="center">
  <img src="/src-tauri/icons/icon.png" width="100px" height="100px" alt="FileFlow" align="center" />
  <h1>FileFlow</h1>
  <div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Tauri-ffc130?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri" />
    <img src="https://img.shields.io/badge/Version-1.0.0-7073f6?style=for-the-badge" alt="Version" />
  </div>
</div>

<div align="center" style="margin-top: 20px">
  <img src="/assets/FileFlowDemo.png" alt="FileFlow" height="250px" width="auto" />
  <img src="/assets/Load_data.png" alt="FileFlow" height="250px" width="auto" />
</div>

## 📄 Description

FileFlow is a simple and easy-to-use tool that allows you to insert data from a CSV file directly into a database table.
With no privilege required for insertion, it simplifies the process while offering efficiency.

Built with **Rust** and the **Tauri** framework, FileFlow is a **cross-platform** application available on **Windows**,
**MacOS**, and **Linux**. 🚀

Check the **Release Section** for the latest version of the application.

## 🌟 Features

- ✅ Insert data into a **new table**
- ✅ Insert data into an **existing table**
- ✅ **Optimize** column types (e.g., `VARCHAR(MAX_LENGTH)`)
- ✅ Insert data from a **CSV file**
- ✅ Does **NOT** require **ANY** privileges for data insertion
- ✅ Generate **LOAD DATA** SQL query for **faster insertion**

## 🛠 Supported Databases

<div align="center">
  <img src="https://img.shields.io/badge/MySQL-00758F?style=for-the-badge&logo=mysql&logoColor=white" alt="MySQL" />
  <img src="https://img.shields.io/badge/MariaDB-003545?style=for-the-badge&logo=mariadb&logoColor=white" alt="MariaDB" />
  <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite" />
</div>

## 📝 Requirements

- **Rust**
- **Cargo**
- **pnpm**

You can follow the instructions on the [Tauri website](https://tauri.app/) to install **Rust** and **Cargo**.

## 📥 Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/Maxime-Cllt/FileFlow.git
    ```

2. Install the dependencies:

    ```bash
    cd FileFlow
    pnpm install
    ```

3. Build the application:

    ```bash
    pnpm tauri build
    ```

4. Run the application:

    ```bash
    pnpm tauri dev
    ```

## 💡 Usage

### 1. Insert Data

- 📁 Select the **CSV file** you want to insert into the database.
- 💻 Choose the **database** where you want to insert the data.
- ➡️ Click the **"Insert"** button.
- ⏳ Wait for the data to be inserted.
- ✅ **Done!**

### 2. Generate Load Data

- 📁 Select the **CSV file** you want to insert into the database.
- 🖱️ Click the **"Load"** button in the menu.
- 📋 Copy the generated **SQL query**.
- 🏃‍♂️ Run the query if your database supports it.
- ✅ **Done!**

## ⚙️ Modes of Data Insertion

There are two modes for inserting data into the database:

- **Optimized Mode**: Inserts data into a new table with optimized column types (e.g., `VARCHAR(MAX_LENGTH)`). 🛠️ This
  mode may take longer but ensures the table structure is optimized.

- **Fast Mode**: Inserts data into an existing table, keeping the original column types as they are in the CSV file. ⚡
  This mode is quicker but may not optimize the column types.

Additionally, you can generate a **LOAD DATA** SQL query for faster insertion if your database supports it. This method
is much quicker than row-by-row insertion! 🚀

## 🤝 Contributing

Want to contribute to the project? Feel free to open an issue or submit a pull request. We welcome contributions and
ideas from the community! 💡

