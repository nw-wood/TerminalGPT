# TerminalGPT

TerminalGPT is a simple command-line interface for interacting with OpenAI's GPT models directly from your terminal.

## Features

- Easy-to-use CLI interface
- Continuous conversation with GPT
- Uses environment variables for API key security

## Requirements

- Rust
- OpenAI API key

## Setup

1. Clone the repository
2. Create a `.env` file in the project root and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```
3. Run `cargo build` to compile the project

## Usage

1. Run the program with `cargo run`
2. Enter your messages when prompted
3. Type 'exit' to quit the program

## Note

This project is for educational purposes and personal use. Ensure you comply with OpenAI's use-case policy when using this tool.
