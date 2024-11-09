<div align=center>
<img src="https://github.com/Maxime-Cllt/DataStorm/blob/main/assets/datastorm.png" width="100px" height="100px"  alt="FileFlow" align="center" />
<h1>FileFlow</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Tauri-ffc130?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri" />
    <img src="https://img.shields.io/badge/Version-1.0.2-7073f6?style=for-the-badge" alt="Version" />
</div>

<div align=center style="margin-top: 20px">
<img src="/assets/FileFlowDemo.png" alt="FileFlow" width="65%" height="50%" />
</div>

## Description

FileFlow is a simple tool that allows you to insert data from a CSV file into a table in a database. It is designed to
be simple to use and to require no privilege to insert the data.
Build with Rust and the Tauri framework, it is a cross-platform application that can be used on Windows, MacOS and
Linux.

See in the release section for the latest version of the application.

## Features

<label>
<input type="checkbox" style="margin-right: 10px" checked>
</label> Insert data into a new table <br>
<label>
<input type="checkbox" style="margin-right: 10px" checked>
</label> Insert data into an existing table <br>
<label>
<input type="checkbox" style="margin-right: 10px" checked>
</label> Optimise the type of the columns (VARCHAR(MAX_LENGHT)) <br>
<label>
<input type="checkbox" style="margin-right: 10px" checked>
</label> From CSV file <br>
<label>
<input type="checkbox" style="margin-right: 10px" checked>
</label> Don't require <span style="font-weight: bold;">ANY</span> privilege to insert the data <br>

## Supported databases

<div align=center>

![MySQL](https://img.shields.io/badge/MySQL-00758F?style=for-the-badge&logo=mysql&logoColor=white)
![MySQL](https://img.shields.io/badge/MariaDB-003545?style=for-the-badge&logo=mariadb&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)

</div>

## Requirements

- Rust
- Cargo
- pnpm

To install Rust and Cargo, you can follow the instructions on the Tauri website

## Installation

1. Clone the repository

```bash
git clone https://github.com/Maxime-Cllt/FileFlow.git
```

2. Install the dependencies

```bash
cd FileFlow
```

```bash
pnpm install
```

3. Build the application

```bash
pnpm tauri build
```

4. Run the application

```bash
pnpm tauri dev
```

## Usage

1. Select the CSV file you want to insert into the database
2. Select the database you want to insert the data into
3. Click on the "Insert" button
4. Wait for the data to be inserted
5. Done!

## Information

There are two mode to insert the data into the database:

- **Optimised mode**: The data is inserted into a new table with the type of the columns optimised (VARCHAR(MAX_LENGHT))
- **Fast mode**: The data is inserted into an existing table with the type of the columns as they are in the CSV file

The optimised mode might take longer to insert the data but it will create a table with the type of the columns
optimised.


