<div align=center>
<img src="https://github.com/Maxime-Cllt/DataStorm/blob/main/assets/datastorm.png" width="100px" height="100px"  alt="DataStorm" align="center" />
<h1>FileFlow</h1>
</div>

## Description

FileFlow is a simple tool that allows you to insert data from a CSV file into a table in a database. It is designed to
be simple to use and to require no privilege to insert the data.
Build with Rust and the Tauri framework, it is a cross-platform application that can be used on Windows, MacOS and
Linux.

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

## Installation

1. Clone the repository

```bash
git clone
```

2. Install the dependencies

```bash
cd FileFlow
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



