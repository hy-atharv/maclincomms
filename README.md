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

---

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


- **maclincomms** follows a unified Rust-based architecture, with both the client and server built in Rust for performance, safety, and seamless integration.   
- The server leverages **Actix Web** and **Tokio** to handle high-concurrency workloads efficiently, utilizing the `actix-ws` crate for WebSocket support.  
- On the client side, `tokio-tungstenite` crate and the **Tokio** async runtime enable non-blocking WebSocket communication for sending and receiving messages. The system adopts an **event-driven architecture**, using `mpsc` channels to propagate network events and coordinate application logic. Shared state is safely accessed across threads using `Arc` and `Mutex`.  
- All server endpoint requests are authenticated via **JWTs**, ensuring secure access.  
- For end-to-end encryption, maclincomms implements a lightweight custom version of the **Signal Protocol**, built from scratch with the help of [Rust Crypto](https://github.com/rustcrypto) libraries.   
- Additionally, real-time notifications are handled using **Redis PUB/SUB** and delivered via **Server-Sent Events (SSE)**, with offline messages temporarily queued in Redis lists for quick retrieval. 

This architecture strikes an effective balance between speed, scalability, and security — purpose-built for a modern, terminal-based communication platform.

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

### 6.1 Seamless Login Experience

Starting from version `2.0.0`, **maclincomms** introduces **persistent authentication**, removing the need to log in every time the app is launched. This improvement over version `1.0.0` significantly enhances usability, especially for frequent users who previously had to manually authenticate on every run.

---

### 6.2 How It Works

Behind the splash screen, maclincomms silently verifies the user using a **persistently stored access token**. This mechanism is powered by the [`disk-persist`](https://docs.rs/disk-persist/latest/disk_persist/) crate, which securely saves token data on the device.

If the stored access token is still valid, authentication proceeds automatically without user intervention. When the access token expires, maclincomms uses the associated **refresh token** to request a new access token from the server and then updates the stored token—completing the entire flow transparently.

---

### 6.3 Token Expiry & Re-authentication

While the persistent login flow works seamlessly during regular usage, **access and refresh tokens do expire after extended inactivity**. If you haven’t used maclincomms for a long time, the system will no longer be able to refresh tokens. In such cases, you’ll be prompted to re-enter your **username and password** to authenticate back into your existing account (on the same device).

This flow ensures a balance between **user convenience** and **account security**, making authentication automatic during active use and protective when dormant.


## 7. 🌏World Chat

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/488216c5-be6a-40e1-8561-16ed786b6cc6" align="center">
</p>
<p align="center">World Chat Screen</p>


### 7.1 Public Global Channel

**World Chat** is a free and open communication channel within **maclincomms**, where all users can join and engage in a public conversation. It provides a global, community-driven space to connect and chat in real-time with other maclincomms users.

Since World Chat is open to all registered users and allows anyone to join or leave at any time, **it is not end-to-end encrypted**. This design choice prioritizes openness and simplicity for the global chatroom while maintaining security in private and direct conversations elsewhere in the app.

To learn how to send **multi-line messages** in World Chat, refer to:  
[Message Input in Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#42-message-input-in-chats)

---

### 7.2 Message Acknowledgement

In **maclincomms**, every message—whether in World Chat, private rooms, or direct messages—displays a visual acknowledgement marker in the top-right corner of the chat bubble. These markers represent the delivery status of each message:

- `>` — **Server Acknowledgement**: The message has been successfully sent from your device and received by the server.
- `>>` — **Receiver Acknowledgement**: The message has also been delivered to the intended recipient's device.

In **World Chat**, messages currently only support **Server Acknowledgement (`>`)**, as there is no designated recipient to confirm delivery beyond the server.


## 8. 👥Add Users

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/30ac7fe2-657a-4399-9699-06499651f4ed" align="center">
</p>
<p align="center">Add User Screen</p>

In **maclincomms**, connecting with other users is simple and seamless through the **Add User** screen.

To send a friend request, navigate to the **Add User** screen from the main menu. Enter the **username** of the person you wish to add, along with an **optional message**. This message will be sent alongside the add request to provide context or a greeting.

Once submitted, the recipient will receive a notification of your request. They have the option to either **accept** or **ignore** it. If they choose to accept, you’ll receive a confirmation notification, and the user will automatically be added to your **DM User** list, enabling you to start a direct message conversation.

This flow ensures that user connections remain **intentional**, **consensual**, and **notified at every step**.

## 9. ✉️DM Chats

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/840ef5b1-9712-4297-a2da-a60e3ed3709c" align="center">
</p>
<p align="center">DMs List in DM User Screen</p>

### 9.1 DM User Screen

You can access all your direct message conversations through the **DM User** screen. This screen displays your **DMs List**, which contains all users you’ve connected with and had private conversations.

To start chatting, simply navigate to the desired user in the DMs List using the arrow keys and press `Enter`. This will instantly load your chat history and take you to the **DM Chat Screen** for a seamless messaging experience.

---

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/f5045ff0-10e4-48fc-8cc4-54af319b7b20" align="center">
</p>
<p align="center">DM Chat Screen</p>

### 9.2 Private DM Channel

Direct messages in **maclincomms** are designed for **private, secure, and end-to-end encrypted communication**.

Unlike traditional messaging apps that store chat history on-device, **maclincomms does not retain any DM chat data locally**. Instead, messages are securely stored in the cloud as **encrypted, sessioned chat segments**, which automatically expire and are deleted after **24 hours**.

This approach ensures your conversations remain **ephemeral, tamper-proof, and accessible only to the intended parties**. Whether you're collaborating with colleagues or talking to close friends, DMs provide the privacy and integrity you expect from a secure communication platform.

To learn how to send **multi-line messages** in DM Chat, refer to:  
[Message Input in Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#42-message-input-in-chats)

---

### 9.3 Message Acknowledgement

Just like in other chat modes, maclincomms shows **message acknowledgement indicators** in DM chats at the top-right of each chat bubble:

- `>` — **Server Acknowledgement**: The message has been successfully sent from your device and received by the server.
- `>>` — **Receiver Acknowledgement**: The message has been delivered to the recipient’s device.

In **DM chats**, maclincomms supports:
- **Receiver Acknowledgement (`>>`)** when the recipient is **online**
- **Server Acknowledgement (`>`)** when the recipient is **offline**

This gives you real-time feedback on your message delivery status, enhancing trust and clarity in your conversations.



## 10. 🔒DM Chats End-To-End Encryption

### 10.1 Overview

DM Chats in **maclincomms** are secured using end-to-end encryption powered by the **AES-GCM algorithm** with a **256-bit symmetric key** and a **96-bit nonce**, utilizing the [`aes-gcm`](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) crate for authenticated encryption.

Each user is issued a unique **Identity Key Pair** at the time of registration. This key pair is an **X25519 key pair** (32 bytes each for public and private key) generated by the **Curve25519** elliptic curve using the [`x25519-dalek`](https://github.com/dalek-cryptography/curve25519-dalek/tree/main/x25519-dalek) crate, and stored securely on the device.

When two users initiate a conversation, they perform a **Diffie-Hellman Key Exchange** using their identity key pairs to derive a shared secret and compute an initial **Root Key**.

maclincomms employs the **Double Ratchet Algorithm** to evolve the encryption keys over time:
- **Symmetric-Key Ratchet**: A new encryption key is derived for each message, preserving **forward secrecy**.
- **Diffie-Hellman Ratchet**: New key pairs are generated during conversational direction changes to ensure **post-compromise security**.

---


<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/f19f4e5a-fc46-4438-af52-55bf419cce5e" align="center">
</p>
<p align="center">Symmetric-Key Ratchet</p>

### 10.2 Symmetric-Key Ratchet

The **Symmetric-Key Ratchet** handles the frequent evolution of message encryption keys.

Each message is encrypted with a new **message key** derived from the **chain key** using the **HKDF** key derivation algorithm provided by the [`hkdf`](https://github.com/RustCrypto/KDFs/tree/master/hkdf) crate.

The Key derivation process is as follows:
```
Chain Key = HKDF(Root Key, 0x01)
Message Key = HKDF(Chain Key, 0x01)
Chain Key (new) = HKDF(Chain Key, 0x02)
```

This continual rotation of the chain key ensures that previously used message keys and even the keys that generated them are not recoverable, providing **forward secrecy**. Each new message uses a fresh key, which makes retroactive decryption impossible even if current keys are compromised.

---


<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/7c20b36f-d1e5-4e49-9b90-66d99dce015d" align="center">
</p>
<p align="center">Diffie-Hellman Ratchet</p>

### 10.3 Diffie-Hellman Ratchet

While the symmetric ratchet evolves message keys, the **Diffie-Hellman (DH) Ratchet** securely rotates the **root key** during conversational direction changes.

#### Initial Setup:
- When the first message in a DM is sent, the sender:
  - Generates a fresh X25519 DH Key Pair.
  - Uses the recipient’s **identity public key** and their own **DH private key** to derive a shared secret.
  - Derives a **root key**, **sending chain key**, and a **message key**.

The Key derivation process is as follows:
```
(DH_PUB_KEY, DH_PRIV_KEY) = X25519 Key Pair
Shared Secret = DH(Recipient_ID_PUB_KEY, DH_PRIV_KEY)
Root Key = HKDF(Shared Secret, 0x02)
Sending Chain Key = HKDF(Root Key, 0x01)
Message Key = HKDF(Sending Chain Key, 0x01)
```

The sender includes their **DH public key** in the message.

#### On Receiving:
- The recipient:
  - Extracts the sender’s DH public key.
  - Uses their **identity private key** to compute the shared secret.
  - Derives the root key, receiving chain key, and message key to decrypt the message.

The Key derivation process is as follows:
```
Shared Secret = DH(ID_PRIV_KEY, Sender_DH_PUB_KEY)
Root Key = HKDF(Shared Secret, 0x02)
Receiving Chain Key = HKDF(Root Key, 0x01)
Message Key = HKDF(Receiving Chain Key, 0x01)
```

#### Conversational Direction Change:
- When the recipient replies for the first time, they:
  - Generate a new DH Key Pair.
  - Use the sender’s DH public key to compute a new shared secret and root key.
  - Send their **DH public key** with the message.

Subsequent direction changes continue this pattern, rotating DH key pairs and providing **post-compromise security** by making previously compromised keys unusable for future message decryption.

---


<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/433ddcb8-0e3c-43b5-a02d-01edb4ced547" align="center">
</p>
<p align="center">Double Ratchet</p>

### 10.4 Double Ratcheting & Synchronization

The **Double Ratchet Algorithm** synergizes the strengths of both ratchets:
- The **Symmetric-Key Ratchet** ensures that each message uses a unique key.
- The **Diffie-Hellman Ratchet** ensures long-term security by evolving the root key with every direction change.

#### Symmetric Ratchet Key Synchronization

maclincomms maintains ratchet synchronization using **Receiver Acknowledgements**:

- Chain Keys are only rotated after receiving a **Receiver Acknowledgement (`>>`) or receiving a message**.
- If the recipient is **offline**, only a **Server Acknowledgement (`>`)** is received and the **message is queued in Redis**.
- When the recipient comes online and pulls queued messages, the **chain key is not rotated**, ensuring message decryption aligns.

#### Diffie-Hellman Key Synchronization

To track ratchet state, both sender and receiver maintain:

- A record of each other’s **latest Public DH Key** (initialized as a zeroes key).
- Their own **Private DH Key** (also initialized as zeroes key).

**Message Receipt Logic**:
- If recipient's DH Key = zeroes & sender’s Public DH Key ≠ zeroes → first message received.
- If both recipient and sender DH Keys = zeroes → first ever DM.
- If received DH key = stored DH key → sender repeated messages using same DH keys; rotate only receiving chain key.
- If received DH key ≠ stored DH key → conversational direction change; perform DH ratchet.

**Key Pairs are rotated only** when a **direction change** is detected, minimizing unnecessary overhead while maintaining strong security guarantees.

---

By combining **message-level key evolution** with **periodic root key replacement**, maclincomms ensures your conversations remain **private, ephemeral, and secure—even under compromise scenarios**.



## 11. ☁️Cloud-Stored Sessioned DM Chats

### 11.1 Device-Free Storage

In **maclincomms**, **DM chats are never stored on your device**. Instead, they are securely encrypted and stored in the cloud using a unique **Session Key** owned by each user. This approach guarantees that your private messages never leave a trace on your local machine, ensuring stronger privacy.

Whenever a user opens maclincomms, their **cloud-stored encrypted DM chats** are retrieved, decrypted locally, and then rendered in the chat UI when a DM with a specific user is opened.

---

### 11.2 Per-User Session Key Encryption

Each user maintains their **own Session Key**, which encrypts **all their outgoing and incoming DM chats** for cloud storage. This means:
- Chats you send and receive are encrypted with **your own Session Key**.
- Your Session Key is not shared with other users.
- You store **your own copy of the encrypted DM chat history** in the cloud, isolated from others.

This method ensures that even if you and another user are part of the same conversation, **each of you stores your own encrypted version** of the DM session.

---

### 11.3 Time-Bound Session Lifecycle

maclincomms adopts a **24-hour session window** for all DM chat backups. Each session is tagged with a **timestamp**, and the following mechanisms ensure that message data remains ephemeral:

- The **Session Key** is rotated after 24 hours.
- Chats encrypted with that key are also marked with the same session timestamp.
- Upon session expiration:
  - A new Session Key is generated.
  - Older chat backups with the expired timestamp are automatically **deleted from the cloud**.

This cycle leads to a natural expiration of messages and enforces **ephemeral messaging** without storing anything locally.

---

### 11.4 Disappearing Messages by Design

Thanks to session-based encryption and timestamped cloud storage:
- **DM chats vanish after each session ends**.
- You’re left with **no traces** of prior messages unless you've actively opened them within that 24-hour period.
- Even if someone gains access to your device, **there is no local chat data to retrieve**.

This method achieves the effect of **"Disappearing Messages"** without relying on manual deletion or device-based cleanup—everything is managed **securely, automatically, and invisibly** in the background.


## 12. 🚪Room Chats

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/89ea0965-c760-4a3d-8bc9-431d1077f598" align="center">
</p>
<p align="center">Room Chat Screen</p>

### 12.1 Private Ephemeral Room Channel

**Rooms** in maclincomms are private, secure, and ephemeral group channels that offer end-to-end encrypted communication. A room can be created with a **unique Room Name**, and access to it is protected by a **Room Key**.

Once created, the Room Owner can share this Room Key with trusted friends or colleagues, allowing them to join by entering the Room Name and Key.

However, rooms in maclincomms are **ephemeral by design**:
- When the **Room Owner exits the room or leaves maclincomms**, the entire room is **automatically deleted**.
- All members are removed instantly.
- No trace of the room or its chat history is retained, either locally or in the cloud.

This ensures that Room Chats remain **temporary and private**, lasting **only as long as the Room Owner is present**.

To learn how to send **multi-line messages** in Room Chat, refer to:  
[Message Input in Chats](https://github.com/hy-atharv/maclincomms/blob/main/README.md#42-message-input-in-chats)

---

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/2ce99cf6-0f88-4dae-98ae-ade3d718cea2" align="center">
</p>
<p align="center">Create Room Screen</p>

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/a85d993e-0cbd-4981-bece-765d2e434f3a" align="center">
</p>
<p align="center">Room Chat Screen displaying Room Name & Key to Owner</p>

### 12.2 Create Rooms

You can create a new room from the **Create Room Screen** by entering a **valid and available Room Name**. Once you hit `Enter`, the room is created and you're immediately joined as the **Room Owner**.

Upon entering, a **system message** from maclincomms is displayed containing the:
- **Room Name**
- **Room Key**

You can share these with others to allow them to join the room.

---

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/958a3bc0-e3c7-469a-a622-51cff9251dc1" align="center">
</p>
<p align="center">Join Room Screen</p>

### 12.3 Join Rooms

To join an existing room, head to the **Join Room Screen**, enter the **Room Name and Key**, and hit `Enter`.

If the provided Room Name and Key are valid and the room currently exists, you'll be **added to the room instantly**.

However, if the **Room Owner leaves the room**, it is:
- **Deleted on the spot**, and
- All remaining participants are **disconnected immediately**.

This keeps the room's existence tightly bound to the owner's presence, ensuring ephemerality.

---

### 12.4 Message Acknowledgement

Just like in other chat modes, maclincomms shows **message acknowledgement indicators** in Room chats at the top-right of each chat bubble:

- `>` — **Server Acknowledgement**: The message has been successfully sent from your device and received by the maclincomms server.
- `>>` — **Receiver Acknowledgement**: The message has been delivered to the intended recipient’s device.

In **Room Chats**, messages currently **only support Server Acknowledgement (`>`)**. Since there is no designated one-to-one recipient, **Receiver Acknowledgement from room members is not yet supported**.


## 13. 🔒Room Chats End-To-End Encryption

### 13.1 Overview

Room Chats in **maclincomms** are secured using **end-to-end encryption**, powered by the **AES-GCM algorithm** with a **256-bit symmetric key** and a **96-bit nonce**, utilizing the [`aes-gcm`](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) crate for authenticated encryption.

Each time the maclincomms app is opened, a **new random Signature Key Pair** is generated. This key pair is an **ED25519 key pair** (32 bytes each for public and private key), created using the **Curve25519 elliptic curve** with the [`ed25519-dalek`](https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek) crate. This key pair is **temporarily cached** for that session.

When a user joins a room:
- They send their [Sender Key](https://github.com/hy-atharv/maclincomms/blob/main/README.md#132-sender-key) to all other members using **pairwise encrypted messaging**, and
- Receive the Sender Keys of all room members individually using the same mechanism.

This pairwise key exchange follows the protocol explained in [**DM Chats End-to-End Encryption**](https://github.com/hy-atharv/maclincomms#10-dm-chats-end-to-end-encryption).

Your **Sender Key** is used to **encrypt and sign** your message.
The **Sender Keys** of others are used to **verify their message signatures** and **decrypt** the ciphertext.

**If any room member leaves the room**, the Sender Key exchange process is **repeated among all remaining members**, ensuring the encryption context remains valid and secure.

---

### 13.2 Sender Key

When a person joins a room:

- A **random 32-byte Chain Key** is generated, acting as the **Sending Chain Key**.
- The **ED25519 Signature Key Pair** generated during app startup is used to sign outgoing messages.

The **Sender Key Bytes** are composed by concatenating:
```
[32_BYTE_CHAIN_KEY][32_BYTE_PUB_SIGNATURE_KEY]
```


This combined Sender Key is encrypted and sent **individually to each room member** via **pairwise encrypted messaging**, as described in [**DM Chats End-to-End Encryption**](https://github.com/hy-atharv/maclincomms#10-dm-chats-end-to-end-encryption).

Simultaneously, the user receives **encrypted Sender Keys from each member**, which are temporarily stored and used to decrypt and verify incoming messages.

---

### 13.3 Subsequent Messages

Once Sender Keys are exchanged:

- A **Message Key** is derived from the Chain Key.
- The Chain Key is rotated after each message using:
```
Message Key = HKDF(Chain Key, 0x01)
Chain Key (new) = HKDF(Chain Key, 0x02)
```

To send a message:
1. The message is encrypted using the **derived Message Key**.
2. The ciphertext is **digitally signed** using his **Private Signature Key**.
3. The signed, encrypted message is sent to the server, which **fan-outs** the message to all connected room members.

On the recipient side:
1. The signature is verified using the **Public Signature Key** embedded in the sender's Sender Key recipient received before.
2. The **Message Key** is derived from the **Chain Key** embedded in the sender's Sender Key, acting as the **Receiving Chain Key**.
3. The message is decrypted, and the chain key is rotated.

This process ensures:
- **End-to-End Encryption**
- **Sender Authentication**
- **Forward Secrecy** via symmetric-key ratcheting
- **Secure key distribution** using pairwise encrypted Sender Key transfers

maclincomms also ensures **key synchronization**:
- Chain Keys are rotated **only after Server Acknowledgement** or **after a message is successfully received**, preventing ratchet mismatches in group settings.

## 14. 🤫Whisper Mode

### 14.1 Overview

**Whisper Mode** is the flagship innovation of **maclincomms**, offering a powerful, privacy-centric messaging feature within room chats.

Imagine being in a room with a few people you aren’t that close with—maybe acquaintances or colleagues—where you want to say something personal or casual that you’d normally only share with your close friends in that room. You can’t speak freely without worrying about others overhearing.

Now, imagine being in a collaborative room—say, a team of developers—where you want to share a database schema update only with the **Database Engineers**, without notifying everyone else. While direct messaging is an option, it becomes tedious and inefficient for a quick, one-time message.

To solve this, **maclincomms Whisper Mode** introduces a flexible, selective messaging mechanism within rooms, empowering users with granular control over who can or cannot receive a particular message.

With Whisper Mode:
- Messages are sent **only to one or more intended recipients**.
- **No one else in the room**, including the recipients, is made aware that Whisper Mode was used.
- The sender retains **complete control** over the message’s visibility, ensuring privacy and precision.

Whisper Mode supports two intuitive modes:
- `Hide From` — hide the message from specific users.
- `Share With` — share the message only with specific users.

**Whisper Mode** makes in-room conversations more dynamic, secure, and context-aware—giving users the ability to communicate with precision and discretion, without leaving the room or creating separate channels.

---

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/ccd2f9bd-4984-4d80-ab7a-46433ed48534" align="center">
</p>
<p align="center">Hide From</p>

### 14.2 Hide From

The **Hide From** mode enables users to exclude specific room members from seeing a message while sending it to everyone else.

To send a Whisper message using the **Hide From** option, use the following command format:
```
whisper --hf [username1,username2] <Message>
```

- Replace `username1`, `username2`, etc., with the usernames of the people you wish to **hide the message from**.
- The list can contain **one or more usernames**, separated by commas.
- All other members in the room **except** those in the list will receive the message.

---

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/355973c1-1db6-4001-90a7-9fec2369174a" align="center">
</p>
<p align="center">Share With</p>

### 14.3 Share With

The **Share With** mode enables users to send a message **only to specific room members**, while hiding it from everyone else.

To send a Whisper message using the **Share With** option, use the following command format:
```
whisper --sw [username1,username2] <Message>
```

- Replace `username1`, `username2`, etc., with the usernames of the people you wish to **share the message with**.
- The list can contain **one or more usernames**, separated by commas.
- **Only** the specified members in the list will receive the message.


## 15. 🔔Realtime Notifications

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/000fe5a6-f6c2-473e-b28e-ca70b9f6fbe2" align="center">
</p>
<p align="center">All Caught Up</p>

<table width="100%"><tr><td align="left"><img width="500" src="https://github.com/user-attachments/assets/7d0ef83e-67d6-464f-a74f-65c655559f27"></td><td align="right"><img width="500" src="https://github.com/user-attachments/assets/47c3f354-a6d7-4155-9a6d-86d06a4043a6"></td></tr></table>
<p align="center">Realtime Notifications</p>

**maclincomms**, like any modern chat platform, supports **realtime notifications** to ensure users are instantly updated on important events. These notifications include:

- **Add Request Received**
- **User Accepted Your Add Request**
- **New DM Message** (when you're not currently in the chat with that user)

maclincomms implements realtime notifications using the **Redis PUB/SUB model**.

Upon startup, maclincomms connects to a **Server-Sent Events (SSE)** `event-stream`, which remains open throughout the session until the user exits the application. A separate asynchronous thread continuously reads from this stream to detect any **server-sent notifications**.

On the backend, when the server receives an SSE connection from a client:
- It spawns a dedicated task that subscribes to all **notification-related Redis channels** for that user using a **unique channel pattern**.
- When a message is published to any of these channels, the task formats the message as a **Server-Sent Event** and pushes it into the client’s open stream.

If no client task is actively subscribed when a notification is published (i.e., **subscriber count is zero**), the message is not lost. Instead, it is queued into a **Redis List**, acting as a **fallback notification queue**, ensuring the user receives the pending notifications when they reconnect.

This architecture ensures that users receive real-time updates while also maintaining **delivery guarantees** when they’re offline.

## 16. 🔔Queued Notifications

When a user exits the **maclincomms** app, they are no longer connected to the realtime **SSE (Server-Sent Events)** stream. As a result, any realtime notifications—such as add requests or incoming messages—would typically be **lost**, since these events are **fired once and forgotten**.

To ensure no important information is missed, **maclincomms** implements **Queued Notifications** using **Redis Lists** that act as persistent notification queues.

Here’s how it works:
- When a notification is published to a user's **Redis notification channel**, the system checks the number of active subscribers.
- If the **subscriber count is zero** (i.e., the user is offline), the message is **not discarded**.
- Instead, it is **queued** by pushing the notification message into a **Redis List**, which serves as the user's personal notification queue.

When the user comes back online and reopens **maclincomms**:
- The app **retrieves all queued notifications** from the corresponding Redis List.
- Once retrieved, the queue is **cleared**, ensuring that notifications are not reprocessed or shown multiple times.

This mechanism guarantees **reliable delivery** of critical updates, even when the user is offline—bridging the gap between realtime and persistent notification handling.


## 17. 🚫Block/Unblock Users

<p align="center">
<img width=600 src="https://github.com/user-attachments/assets/2953bebf-8f57-48c6-ae0b-38a992227137" align="center">
</p>
<p align="center">Block/Unblock User Screen</p>

**maclincomms** allows users to maintain their privacy and control their interactions by enabling the option to **Block** or **Unblock** users.

If you’ve previously added someone but no longer wish to communicate with them, you can easily block them:

- Navigate to the **Block/Unblock User** screen.
- Enter the **username** of the user you wish to block.
- Press `Ctrl + B` to **block** the user.

If you decide to resume communication in the future:

- Go to the same **Block/Unblock User** screen.
- Enter the **username** of the user you want to unblock.
- Press `Ctrl + U` to **unblock** the user.

Blocking a user ensures they can no longer send you messages until they are unblocked and added again.



## 18. 🗄️Databases & Server

## 19. 🔄Project Maintenance & Future Updates


## 🌐Open Source & Contribution

1. [Open Source, Free to explore & Build on](https://github.com/hy-atharv/maclincomms#1-%EF%B8%8Fopen-source-free-to-explore--build-on)
2. [Credit Honestly](https://github.com/hy-atharv/maclincomms#2-credit-honestly)
3. [How to Contribute?](https://github.com/hy-atharv/maclincomms#3-%EF%B8%8Fhow-to-contribute)

## 1. 🛠️Open Source, Free to explore & Build on

## 2. 🤝Credit Honestly

## 3. 🏷️How to Contribute?

