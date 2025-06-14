# 📺💬maclincomms (macOS & Linux Communications)💬
<p align="center">
<img width=700 src="https://github.com/hy-atharv/maclincomms/blob/76841e0ce9206b8703f185bee8efd2158fda9df3/maclincomms.png" align="center">
</p>

**maclincomms** (macOS & Linux Communications) is a **Lightweight, Fast, Secure, and Secretive** terminal-based app with a cool **retro-themed yet modern UI**. It runs **seamlessly right in your terminal**.
**End-to-end encrypted** conversations without the hassle of downloading a desktop application or navigating to a browser make it a **potential “WhatsApp in the Terminal"**.
It offers **public chats, ephemeral & secure rooms, secure DM chats, and notifications** — all **within your terminal window**.


## 𝌞Contents

1. [Introduction](https://github.com/hy-atharv/maclincomms#%EF%B8%8Fintroduction)
2. [Installation](https://github.com/hy-atharv/maclincomms/blob/main/README.md#installation)
3. [Documentation](https://github.com/hy-atharv/maclincomms/blob/main/README.md#documentation)
4. [Open Source & Contribution](https://github.com/hy-atharv/maclincomms/blob/main/README.md#open-source--contribution)

## ▶️Introduction
**Watch maclincomms light up the terminal!**

<p align="center">
  <a href="https://youtu.be/Vs1rYvz6bCc">
    <img src="https://github.com/user-attachments/assets/b58a4a5f-8acb-46b2-a784-f6432458e207" width="630" height="390">
  </a>
</p>
<p align="center">Click the image to watch</p>

## 💻Installation

**You can download maclincomms either with homebrew package manager OR by manually downloading the zipped binary, extracting it and setting up the `PATH` variable.**

### Method 1: Install with Homebrew

**You need to have homebrew package manager to install maclincomms by this method**.

[**Homebrew Installation Guide**](https://brew.sh)

**Open your terminal and paste these commands once when you have `brew` working**.
> ```
> brew tap hy-atharv/maclincomms
> ```
> ```
> brew install maclincomms
> ```

**After installation, Just type maclincomms anytime, anywhere**!
> ```
> maclincomms
> ```
---

### Method 2: Download zipped binary

**You can download the zipped binary for your system, as per your OS and processor architecture.**

| **OS**         | **Architecture** | **Tar Archive** |
|----------------|------------------|-----------------|
| macOS          | arm              | [download](https://github.com/hy-atharv/maclincomms/releases/download/v2.0.0/maclincomms-mac-arm.tar.gz)       |
| Linux Distros  | x86_64           | [download](https://github.com/hy-atharv/maclincomms/releases/download/v2.0.0/maclincomms-linux-x86_64.tar.gz)        |

**Extract the binary *maclincomms* from the downloaded tar archive**
> ```
> tar -xvzf <downloaded_filename_with_extension>
> ```

**Set up the $PATH variable for the maclincomms binary**

For Linux Distros:
> ```
> echo 'export PATH=$PATH:/path_to_directory_where_maclincomms_exists' >> ~/.bash_profile
> ```
> ```
> echo 'export PATH=$PATH:/path_to_directory_where_maclincomms_exists' >> ~/.bashrc
> ```

For macOS:
> ```
> echo 'export PATH="/path_to_directory_where_maclincomms_exists:$PATH"' >> ~/.zshrc
> ```
> ```
> echo 'export PATH="/path_to_directory_where_maclincomms_exists:$PATH"' >> ~/.bash_profile
> ```

**Just type maclincomms anytime, anywhere**!
> ```
> maclincomms
> ```

## 📜Documentation

1. [Overview](https://github.com/hy-atharv/maclincomms#1-overview)
2. [Architecture](https://github.com/hy-atharv/maclincomms#2-%EF%B8%8Farchitecture)
3. [TUI & Terminal Window](https://github.com/hy-atharv/maclincomms#3-%EF%B8%8Ftui--terminal-window)
4. [Inputs & Key Bindings](https://github.com/hy-atharv/maclincomms#4-%EF%B8%8Finputs--key-bindings)
5. [Getting Started](https://github.com/hy-atharv/maclincomms/blob/main/README.md#5-getting-started)
6. [Persistent Authentication](https://github.com/hy-atharv/maclincomms/blob/main/README.md#6-persistent-authentication)
7. [World Chat](https://github.com/hy-atharv/maclincomms/blob/main/README.md#7-world-chat)
8. [Add Users](https://github.com/hy-atharv/maclincomms/blob/main/README.md#8-add-users)
9. [DM Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#9-%EF%B8%8Fdm-chats)
10. [DM Chats End-To-End Encryption](https://github.com/hy-atharv/maclincomms/blob/main/README.md#10-dm-chats-end-to-end-encryption)
11. [Cloud-Stored Sessioned DM Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#11-%EF%B8%8Fcloud-stored-sessioned-dm-chats)
12. [Room Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#12-room-chats)
13. [Room Chats End-To-End Encryption](https://github.com/hy-atharv/maclincomms/blob/main/README.md#13-room-chats-end-to-end-encryption)
14. [Whisper Mode](https://github.com/hy-atharv/maclincomms/blob/main/README.md#14-whisper-mode)
15. [Realtime Notifications](https://github.com/hy-atharv/maclincomms/blob/main/README.md#15-realtime-notifications)
16. [Queued Notifications](https://github.com/hy-atharv/maclincomms/blob/main/README.md#16-queued-notifications)
17. [Block/Unblock Users](https://github.com/hy-atharv/maclincomms/blob/main/README.md#17-blockunblock-users)
18. [Databases & Server](https://github.com/hy-atharv/maclincomms#18-%EF%B8%8Fdatabases--server)
19. [Project Maintenance & Future Updates](https://github.com/hy-atharv/maclincomms/blob/main/README.md#19-project-maintenance--future-updates)


## 1. 🔮Overview

**maclincomms** is a modern, terminal-native chat application designed to deliver fast, secure, and efficient communication within a text-based environment. This documentation provides a comprehensive walkthrough of the system's architecture, core technologies, and design principles that power its development.

**Built entirely in Rust**, maclincomms leverages the language’s performance and memory safety guarantees to deliver a robust and reliable system. The terminal interface is crafted using [Ratatui](https://ratatui.rs), offering a clean, fluid, and visually appealing TUI — far beyond the aesthetics of traditional command-line tools.

Security lies at the heart of maclincomms. All communication is protected with **end-to-end encryption**, while access to server endpoints is safeguarded using **JWT-based authentication**, ensuring secure identity verification and private messaging.

The backend is powered by **Actix Web** and **Tokio**, combining high concurrency with asynchronous execution. It follows an **event-driven architecture**, using `mpsc` channels for internal message flow and `Arc` with `Mutex` for safe shared state management across threads.

Together, these design choices make maclincomms **lightweight**, **fast**, and **secure by design** — a serious step toward realizing a true **“WhatsApp in the Terminal”.**

## 2. ⚙️Architecture

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/67863eba-b1a8-428a-bdf3-3f4ff1d606ed" align="center">
</p>
<p align="center">Architecture Overview</p>


**maclincomms** follows a unified Rust-based architecture, with both the client and server built in Rust for performance, safety, and seamless integration. The server leverages **Actix Web** and **Tokio** to handle high-concurrency workloads efficiently, utilizing the `actix-ws` crate for WebSocket support. On the client side, `tokio-tungstenite` crate and the **Tokio** async runtime enable non-blocking WebSocket communication for sending and receiving messages. The system adopts an **event-driven architecture**, using `mpsc` channels to propagate network events and coordinate application logic. Shared state is safely accessed across threads using `Arc` and `Mutex`. All server endpoint requests are authenticated via **JWTs**, ensuring secure access. For end-to-end encryption, maclincomms implements a lightweight custom version of the **Signal Protocol**, built from scratch with the help of [Rust Crypto](https://github.com/rustcrypto) libraries. Additionally, real-time notifications are handled using **Redis PUB/SUB** and delivered via **Server-Sent Events (SSE)**, with offline messages temporarily queued in Redis lists for quick retrieval. This architecture strikes an effective balance between speed, scalability, and security — purpose-built for a modern, terminal-based communication platform.

## 3. 🖥️TUI & Terminal Window

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/8fabbba4-c572-4aad-aad3-c9696bd6b163" align="center">
</p>
<p align="center">Splash Screen</p>

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/3207999b-3522-4634-b1cf-baaac6e37995" align="center">
</p>
<p align="center">Home Screen</p>

### 3.1 What is a TUI?

A **Text-based User Interface (TUI)** provides an interactive UI experience within the terminal itself—without relying on graphical elements. TUIs offer a retro-inspired, keyboard-centric environment that feels snappy, distraction-free, and resource-efficient.

**maclincomms** uses [`ratatui`](https://github.com/ratatui-org/ratatui), a powerful TUI library in Rust, to build an interface that is clean, structured, and visually engaging—while still being entirely text-rendered within the bounds of the terminal. Unlike graphical interfaces, maclincomms's TUI delivers speed and control without ever leaving the terminal context.

---

### 3.2 Terminal Window Behavior

The TUI in maclincomms is rendered directly inside your existing terminal window—**no additional windows or GUI elements are opened**. Once launched, it fully occupies your terminal screen, and suspends normal shell interactions until you exit the program.

maclincomms does not accept mouse interactions. It is designed to be **entirely controlled via keyboard input**, leveraging the [`crossterm`](https://crates.io/crates/crossterm) library for cross-platform key event handling.

---

### 3.3 Best Usage Guidelines

To ensure the best experience using maclincomms, follow these guidelines:

- **Use a dedicated terminal app**, not embedded terminals in code editors or IDEs.
- **Always switch your terminal to fullscreen mode** before launching maclincomms. Small or constrained terminal dimensions can break layout rendering and cause the application to panic.
- **Expect minor differences in color rendering** depending on your terminal emulator, operating system, and theme (e.g., dark/light mode or ANSI color profile support). These variations are common and can slightly affect how elements appear.
- **To exit maclincomms at any time**, press `Control + E`—this shortcut works consistently across macOS, Linux, and Windows systems.



## 4. ⌨️Inputs & Key Bindings

### 4.1 Keyboard Control & Navigation

**maclincomms** exclusively uses keyboard inputs to drive its interface. Every screen clearly displays its relevant key bindings, allowing you to intuitively navigate and trigger UI elements. Throughout the application, the `^` symbol denotes the **Control key** (e.g., `^Q` means `Ctrl + Q`). Thanks to the cross-platform support of the [`crossterm`](https://crates.io/crates/crossterm) library, key inputs behave consistently across macOS, Linux, and Windows.

Each screen in maclincomms has dedicated input handling logic that emits events in line with the app's event-driven architecture. While contextual hints are shown inline, below is a complete reference of all global key bindings available throughout the application:

| **Key Event**   | **Actions Performed**                                                                 |
|-----------------|----------------------------------------------------------------------------------------|
| `Ctrl + Q`      | Exits maclincomms                                                                     |
| `Enter`         | - Selects an option from a menu  <br> - Submits a form  <br> - Sends a message <br> - Selects an item from a list |
| `Esc`           | - Closes an open menu  <br> - Navigates back from any panel                          |
| `Up / Down`     | - Navigates between menu options <br> - Scrolls through chat messages <br> - Moves through list items |
| `I`             | Ignores and removes a notification from the notifications list                        |
| `Ctrl + B`      | Blocks a user from the Block/Unblock User screen                                      |
| `Ctrl + U`      | Unblocks a user from the Block/Unblock User screen                                    |

---

### 4.2 Message Input in Chats

Due to the nature of terminal-based interfaces, text fields in **maclincomms** do not automatically wrap or support line breaks using combinations like `Shift + Enter`. Instead, to create **multi-line messages**, you can insert the escape sequence `\n` within your message. When sent, this will render as a new line inside the chat bubble, maintaining proper formatting within the TUI.

Example:   
```
Hey there!\nHow are you doing today?
```


Will appear as:  
```
Hey there!  
How are you doing today?
```


This method offers a reliable and platform-consistent way to send structured, multi-line messages — just like a boss.

## 5. 🎬Getting Started

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/410afe7f-15a4-4712-89e2-ed16a8ec4a47" align="center">
</p>
<p align="center">Running maclincomms for the first time</p>

### 5.1 First-Time Setup

When running `maclincomms` for the first time, you’ll begin by creating an account to access the system. For enhanced privacy and device-level security, **each instance of maclincomms supports only one registered account per device**. Once an account is created on a particular machine, it can only be accessed from that device. Likewise, attempting to log into the same account from another device is not supported. This model prevents account duplication and ensures that sensitive communication remains bound to a trusted environment.

--- 

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/35aa0086-1fd0-4c36-89b7-f04dfce0d5d8" align="center">
</p>
<p align="center">Registering with valid username and strong password</p>

### 5.2 Registering an Account

To register, you'll be prompted to enter a **valid and unique username** along with a strong password. Make sure to follow the username guidelines below:

- Must be between **5 and 15 characters** in length  
- Can contain **only letters (A–Z, a–z)** and **digits (0–9)**  
- **No spaces** or **special characters** are allowed

After entering and confirming your password, press `Enter` to complete the registration process. Once successfully registered, you’ll be redirected to the **Home Screen** of maclincomms where you can begin exploring its features.





## 6. 💻Persistent Authentication

## 7. 🌏World Chat

## 8. 👥Add Users

## 9. ✉️DM Chats

## 10. 🔒DM Chats End-To-End Encryption

## 11. ☁️Cloud-Stored Sessioned DM Chats

## 12. 🚪Room Chats

## 13. 🔒Room Chats End-To-End Encryption

## 14. 🤫Whisper Mode

## 15. 🔔Realtime Notifications

## 16. 🔔Queued Notifications

## 17. 🚫Block/Unblock Users

## 18. 🗄️Databases & Server

## 19. 🔄Project Maintenance & Future Updates


## 🌐Open Source & Contribution

1. [Open Source, Free to explore & Build on](https://github.com/hy-atharv/maclincomms#1-%EF%B8%8Fopen-source-free-to-explore--build-on)
2. [Credit Honestly](https://github.com/hy-atharv/maclincomms#2-credit-honestly)
3. [How to Contribute?](https://github.com/hy-atharv/maclincomms#3-%EF%B8%8Fhow-to-contribute)

## 1. 🛠️Open Source, Free to explore & Build on

## 2. 🤝Credit Honestly

## 3. 🏷️How to Contribute?

