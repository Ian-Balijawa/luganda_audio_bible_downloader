# 📖 Luganda Bible Audio Downloader

A Rust-based tool to automatically download the entire Luganda Bible in audio format (MP3).  
It fetches chapters book by book from [wordproaudio.net](https://www.wordproaudio.net/) and organizes them neatly into folders.

---

## ✨ Features

- 🚀 **Parallel downloads** with configurable concurrency  
- 📊 **Progress bar** with percentage, current book/chapter, and failed count  
- ⏭️ **Skip existing files** to resume interrupted downloads  
- 🗂️ **Organized structure**: each Bible book has its own folder with chapter files  
- 🛠️ **Error tracking**: keeps a record of failed downloads for retrying later  

---

## 📂 Output Structure



luganda_bible_audio/
├── Genesis/
│ ├── chapter-01.mp3
│ ├── chapter-02.mp3
│ └── ...
├── Exodus/
│ ├── chapter-01.mp3
│ └── ...
└── Revelation/
├── chapter-01.mp3
└── chapter-22.mp3


---

## ⚙️ Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/luganda-bible-audio-downloader.git
cd luganda-bible-audio-downloader

2. Install Rust (if you don’t have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Build the project
cargo build --release


The compiled binary will be available in target/release/.

▶️ Usage

Run the downloader from the command line:

cargo run --release -- [output_directory]


If no directory is provided, the default is ./luganda_bible_audio.

Example:
cargo run --release -- ./my_bible_audio

⚡ Configuration

Concurrency: Currently set to download up to 10 chapters in parallel.
You can adjust this in main.rs:

downloader.download_all(10).await?;


Timeout: Each HTTP request has a 30-second timeout (configurable in the reqwest::Client builder).

📊 Progress Display

The downloader shows a live updating progress bar:

📖 [██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 12.4% | 100/805 files | Current: Genesis Ch.5 | Failed: 0

❌ Failed Downloads

If some chapters fail to download, they are logged at the end:

❌ Failed downloads: 2 files

📋 Failed downloads:
   • Psalms chapter 23: HTTP error: 404 Not Found
   • John chapter 3: timeout


You can re-run the program, and it will skip already downloaded files while retrying missing ones.

📜 License

MIT License © 2025 [Ian Balijawa]

🙌 Acknowledgements

Audio source: wordproaudio.net

Built with ⚡ Rust, tokio, and reqwest.
