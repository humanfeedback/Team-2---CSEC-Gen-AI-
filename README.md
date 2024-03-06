# Keylogger Project Documentation

## Introduction

This project outlines the development and deployment of a basic keylogger application written in Rust. The keylogger is designed to capture and transmit keystrokes from a victim's machine to a server for collection. This document provides a step-by-step guide on setting up and running the keylogger, along with necessary precautions and requirements.

**Note: the final files in the proper Rust project directory format is in the `final` directory. The Rust project for listener and the keylogger parts are in there. The rest is just code generated as a means to get to a final product.**

## Prerequisites

Before proceeding, ensure you have the following installed on your development machine:

- Rust programming language setup, including Cargo (Rust's package manager and build system).
- A TCP listener tool (e.g., Netcat) for receiving the keystroke data.
- Access to both a server machine (to collect keystroke data) and a victim machine (to deploy the keylogger).

**Note:** This software is intended for educational purposes or authorized penetration testing only. Unauthorized use on machines without explicit permission is illegal and unethical.

## Setup Instructions

### Configuring the Server Address

1. Open the `main.rs` file in your preferred text editor.
2. Locate the line containing a hardcoded IP address assignment. Replace the existing IP address with the IP address of your server machine.

### Building the Keylogger

1. In your terminal, navigate to the project directory containing the modified `main.rs` file.
2. Run the following command to build the keylogger executable:
   ```shell
   cargo run
   ```
3. Once the build is complete, locate the `keylogger.exe` file in the target directory.

### Setting Up the Server Listener

1. On your server machine, open a terminal.
2. Start a TCP listener on port 4444 (or your chosen port) using Netcat or an equivalent tool:
   ```shell
   nc -lvp 4444
   ```
   This command will initiate a listening state on the specified port, ready to receive keystroke data.

## Usage

To deploy the keylogger:

1. Transfer the `keylogger.exe` file to the victim machine.
2. Execute the keylogger on the victim machine. This action may require disabling anti-virus software temporarily to avoid detection.
3. Once the keylogger is running, it will establish a connection to the server listener and begin transmitting keystroke data.

**Warning:** Be mindful of anti-virus and security software on the victim machine that may block or detect the keylogger's operation. Disabling such software poses significant security risks and should be done with caution and appropriate permissions.


### Data Storage

#### Log File Details

- **Location**: The keystroke data is logged to a file named `keystrokes_log.txt`, which is located in the same directory as the application's executable.
- **Format**: Each keystroke is logged in plain text, alongside a timestamp indicating when the key was pressed. The data format is designed to be easily readable and accessible for analysis.

### Managing Logged Data

#### Accessing Logged Data

Users can access the keystroke log by navigating to the application directory and opening the `keystrokes_log.txt` file with any standard text editor. This allows for easy review and analysis of typing patterns and productivity.

#### Data Deletion

To delete the logged keystroke data, users can simply remove the `keystrokes_log.txt` file from the application directory. It's recommended to regularly monitor and clear this file to manage data accumulation and maintain privacy.

### Security and Privacy Considerations

#### Data Security

While the keystroke log is stored locally to prioritize user privacy, it may contain sensitive information. Users are advised to take the following steps to secure their data:

- Store the log file in a secure location, preferably on an encrypted drive.
- Regularly review the keystroke log to identify and remove any sensitive information.

#### Privacy Implications

Given the potential sensitivity of keystroke data, it's crucial to manage the log file responsibly. Users should:

- Regularly delete the keystroke log to prevent unnecessary data retention.
- Ensure that the application is used in a manner that complies with local privacy laws and regulations, particularly regarding consent.


## Legal and Ethical Warning

This tool is provided for educational purposes and authorized security testing only. Unauthorized use of this software to intercept or collect data from individuals without their explicit consent is illegal and unethical. It is the user's responsibility to comply with all applicable laws and ethics guidelines when using or deploying this software.

## Conclusion

This documentation provides a comprehensive guide to setting up, deploying, and using the keylogger project. By following the outlined steps, users can successfully capture and analyze keystroke data for educational or authorized security testing purposes.
