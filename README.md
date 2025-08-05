# Discord Custom RPC

A modern, cross-platform desktop application for creating and managing custom Discord Rich Presence activities. Built with Tauri, Vue 3, and Rust.

![Discord Custom RPC](https://img.shields.io/badge/Discord-Custom%20RPC-5865F2?style=for-the-badge&logo=discord&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vue.js&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## ✨ Features

### 🎮 Rich Presence Management
- **Custom Activity Details**: Set custom details and state text for your Discord presence
- **Image Support**: Upload and use custom large and small images with hover text
- **Interactive Buttons**: Add up to 2 clickable buttons with custom labels and URLs
- **Timestamp Control**: Configure elapsed time, remaining time, or no time display
- **Real-time Updates**: Instantly update your Discord presence without reconnecting

### 🖼️ Image Handling
- **Automatic Image Upload**: Upload images directly from your device
- **Image Optimization**: Automatically resize images to Discord's 512x512 requirement
- **Multiple Formats**: Support for PNG, JPG, and other common image formats
- **Cloud Storage**: Images are uploaded to a reliable hosting service for Discord compatibility

### 🌐 User Experience
- **Bilingual Interface**: Full support for English and Turkish languages
- **Modern UI**: Clean, responsive interface built with Tailwind CSS
- **Toast Notifications**: Real-time feedback for all operations
- **Advanced Settings**: Collapsible advanced options for power users
- **Connection Status**: Clear indication of Discord RPC connection status

### 🔧 Technical Features
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Lightweight**: Small footprint with fast startup times
- **Secure**: Built with Tauri's security-first approach
- **No Dependencies**: Standalone executable with no external requirements

## 🚀 Getting Started

### Prerequisites

1. **Discord Application**: You need to create a Discord application to get an Application ID
   - Go to [Discord Developer Portal](https://discord.com/developers/applications)
   - Click "New Application" and give it a name
   - Copy the "Application ID" from the General Information page

2. **Discord Desktop**: Make sure Discord desktop application is running

### Installation

#### Option 1: Download Pre-built Binary
1. Go to the [Releases](https://github.com/sw3do/discord-custom-rpc/releases) page
2. Download the appropriate version for your operating system
3. Run the installer or extract the portable version

#### Option 2: Build from Source

**Prerequisites:**
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

**Build Steps:**
```bash
# Clone the repository
git clone https://github.com/sw3do/discord-custom-rpc.git
cd discord-custom-rpc

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📖 Usage Guide

### 1. Connect to Discord
1. Launch the application
2. Enter your Discord Application ID in the "Application ID" field
3. Click "Connect" to establish connection with Discord
4. Wait for the "Connected" status confirmation

### 2. Configure Your Presence

#### Basic Settings
- **Details**: Main activity text (e.g., "Playing a game")
- **State**: Secondary activity text (e.g., "In a match")

#### Images
- **Large Image**: Main image displayed in the presence
- **Small Image**: Small overlay image (appears in bottom-right of large image)
- **Image Text**: Hover tooltips for both images

#### Time Display
- **No Time**: Don't show any timestamp
- **Elapsed Time**: Show time since activity started
- **Remaining Time**: Show countdown timer (specify duration)

#### Buttons (Advanced)
- Add up to 2 clickable buttons
- Each button needs a label and a valid URL
- Buttons appear at the bottom of the presence

### 3. Update Your Presence
1. Fill in the desired fields
2. Click "Update Presence" to apply changes
3. Check Discord to see your custom presence
4. Use "Clear Presence" to remove the activity

## 🛠️ Development

### Project Structure
```
discord-custom-rpc/
├── src/                    # Vue.js frontend
│   ├── App.vue            # Main application component
│   ├── main.ts            # Application entry point
│   └── style.css          # Global styles
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Main Rust library
│   │   └── main.rs        # Tauri application entry
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── public/                # Static assets
└── package.json           # Node.js dependencies
```

### Available Scripts

```bash
# Development
npm run dev          # Start Vite dev server
npm run tauri dev    # Start Tauri development mode

# Building
npm run build        # Build frontend
npm run tauri build  # Build complete application

# Code Quality
cargo clippy         # Run Rust linter (in src-tauri/)
cargo fmt           # Format Rust code (in src-tauri/)
```

### Backend API Commands

The Rust backend exposes these Tauri commands:

- `connect_discord_rpc(client_id: String)` - Connect to Discord RPC
- `set_activity(activity: ActivityData)` - Update Discord presence
- `clear_activity()` - Clear current presence
- `disconnect_discord_rpc()` - Disconnect from Discord
- `get_connection_status()` - Check connection status
- `upload_image_to_api(image_data: String)` - Upload and resize images
- `resize_image_for_discord(image_data: String)` - Resize images locally

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines
1. Follow the existing code style
2. Add comments for complex logic
3. Test your changes thoroughly
4. Update documentation as needed

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Discord Rich Presence](https://discord.com/developers/docs/rich-presence/how-to) - Official Discord RPC documentation
- [Tauri](https://tauri.app/) - The framework that makes this application possible
- [Vue.js](https://vuejs.org/) - The progressive JavaScript framework
- [Tailwind CSS](https://tailwindcss.com/) - For the beautiful UI styling

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/sw3do/discord-custom-rpc/issues) page for existing solutions
2. Create a new issue with detailed information about your problem
3. Join our community discussions

## 🔄 Changelog

### v0.1.0 (Current)
- Initial release
- Basic Discord RPC functionality
- Image upload and management
- Bilingual interface (English/Turkish)
- Cross-platform support
- Advanced timestamp controls
- Interactive buttons support

---

**Made with ❤️ by [sw3do](https://github.com/sw3do)**
