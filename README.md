<div align=center>
<img src="https://github.com/Maxime-Cllt/DataStorm/blob/main/assets/datastorm.png" width="100px" height="100px"  alt="DataStorm" align="center" />
<h1>FileFlow</h1>
</div>

<div align=center>
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


