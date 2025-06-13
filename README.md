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

## 2. ⚙️Architecture

## 3. 🖥️TUI & Terminal Window

## 4. ⌨️Inputs & Key Bindings

## 5. 🎬Getting Started

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
2. [Credit Honestly](https://github.com/hy-atharv/maclincomms#%EF%B8%8Fintroduction)
3. [How to Contribute?](https://github.com/hy-atharv/maclincomms#%EF%B8%8Fintroduction)

## 1. 🛠️Open Source, Free to explore & Build on

## 2. 🤝Credit Honestly

## 3. 🏷️How to Contribute?

