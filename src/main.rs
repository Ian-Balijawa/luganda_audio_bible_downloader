use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest;
use tokio;

#[derive(Debug, Clone)]
struct BibleBook {
    name: String,
    chapters: u32,
}

#[derive(Debug)]
struct DownloadProgress {
    total_files: usize,
    downloaded: usize,
    current_book: String,
    current_chapter: u32,
    failed: Vec<String>,
}

pub struct BibleAudioDownloader {
    base_url: String,
    output_dir: String,
    bible_books: HashMap<u32, BibleBook>,
    client: reqwest::Client,
    progress: Arc<Mutex<DownloadProgress>>,
}

impl BibleAudioDownloader {
    pub fn new(output_dir: &str) -> Self {
        let mut bible_books = HashMap::new();
        
        // Old Testament
        bible_books.insert(1, BibleBook { name: "Genesis".to_string(), chapters: 50 });
        bible_books.insert(2, BibleBook { name: "Exodus".to_string(), chapters: 40 });
        bible_books.insert(3, BibleBook { name: "Leviticus".to_string(), chapters: 27 });
        bible_books.insert(4, BibleBook { name: "Numbers".to_string(), chapters: 36 });
        bible_books.insert(5, BibleBook { name: "Deuteronomy".to_string(), chapters: 34 });
        bible_books.insert(6, BibleBook { name: "Joshua".to_string(), chapters: 24 });
        bible_books.insert(7, BibleBook { name: "Judges".to_string(), chapters: 21 });
        bible_books.insert(8, BibleBook { name: "Ruth".to_string(), chapters: 4 });
        bible_books.insert(9, BibleBook { name: "1 Samuel".to_string(), chapters: 31 });
        bible_books.insert(10, BibleBook { name: "2 Samuel".to_string(), chapters: 24 });
        bible_books.insert(11, BibleBook { name: "1 Kings".to_string(), chapters: 22 });
        bible_books.insert(12, BibleBook { name: "2 Kings".to_string(), chapters: 25 });
        bible_books.insert(13, BibleBook { name: "1 Chronicles".to_string(), chapters: 29 });
        bible_books.insert(14, BibleBook { name: "2 Chronicles".to_string(), chapters: 36 });
        bible_books.insert(15, BibleBook { name: "Ezra".to_string(), chapters: 10 });
        bible_books.insert(16, BibleBook { name: "Nehemiah".to_string(), chapters: 13 });
        bible_books.insert(17, BibleBook { name: "Esther".to_string(), chapters: 10 });
        bible_books.insert(18, BibleBook { name: "Job".to_string(), chapters: 42 });
        bible_books.insert(19, BibleBook { name: "Psalms".to_string(), chapters: 150 });
        bible_books.insert(20, BibleBook { name: "Proverbs".to_string(), chapters: 31 });
        bible_books.insert(21, BibleBook { name: "Ecclesiastes".to_string(), chapters: 12 });
        bible_books.insert(22, BibleBook { name: "Song of Solomon".to_string(), chapters: 8 });
        bible_books.insert(23, BibleBook { name: "Isaiah".to_string(), chapters: 66 });
        bible_books.insert(24, BibleBook { name: "Jeremiah".to_string(), chapters: 52 });
        bible_books.insert(25, BibleBook { name: "Lamentations".to_string(), chapters: 5 });
        bible_books.insert(26, BibleBook { name: "Ezekiel".to_string(), chapters: 48 });
        bible_books.insert(27, BibleBook { name: "Daniel".to_string(), chapters: 12 });
        bible_books.insert(28, BibleBook { name: "Hosea".to_string(), chapters: 14 });
        bible_books.insert(29, BibleBook { name: "Joel".to_string(), chapters: 3 });
        bible_books.insert(30, BibleBook { name: "Amos".to_string(), chapters: 9 });
        bible_books.insert(31, BibleBook { name: "Obadiah".to_string(), chapters: 1 });
        bible_books.insert(32, BibleBook { name: "Jonah".to_string(), chapters: 4 });
        bible_books.insert(33, BibleBook { name: "Micah".to_string(), chapters: 7 });
        bible_books.insert(34, BibleBook { name: "Nahum".to_string(), chapters: 3 });
        bible_books.insert(35, BibleBook { name: "Habakkuk".to_string(), chapters: 3 });
        bible_books.insert(36, BibleBook { name: "Zephaniah".to_string(), chapters: 3 });
        bible_books.insert(37, BibleBook { name: "Haggai".to_string(), chapters: 2 });
        bible_books.insert(38, BibleBook { name: "Zechariah".to_string(), chapters: 14 });
        bible_books.insert(39, BibleBook { name: "Malachi".to_string(), chapters: 4 });
        
        // New Testament
        bible_books.insert(40, BibleBook { name: "Matthew".to_string(), chapters: 28 });
        bible_books.insert(41, BibleBook { name: "Mark".to_string(), chapters: 16 });
        bible_books.insert(42, BibleBook { name: "Luke".to_string(), chapters: 24 });
        bible_books.insert(43, BibleBook { name: "John".to_string(), chapters: 21 });
        bible_books.insert(44, BibleBook { name: "Acts".to_string(), chapters: 28 });
        bible_books.insert(45, BibleBook { name: "Romans".to_string(), chapters: 16 });
        bible_books.insert(46, BibleBook { name: "1 Corinthians".to_string(), chapters: 16 });
        bible_books.insert(47, BibleBook { name: "2 Corinthians".to_string(), chapters: 13 });
        bible_books.insert(48, BibleBook { name: "Galatians".to_string(), chapters: 6 });
        bible_books.insert(49, BibleBook { name: "Ephesians".to_string(), chapters: 6 });
        bible_books.insert(50, BibleBook { name: "Philippians".to_string(), chapters: 4 });
        bible_books.insert(51, BibleBook { name: "Colossians".to_string(), chapters: 4 });
        bible_books.insert(52, BibleBook { name: "1 Thessalonians".to_string(), chapters: 5 });
        bible_books.insert(53, BibleBook { name: "2 Thessalonians".to_string(), chapters: 3 });
        bible_books.insert(54, BibleBook { name: "1 Timothy".to_string(), chapters: 6 });
        bible_books.insert(55, BibleBook { name: "2 Timothy".to_string(), chapters: 4 });
        bible_books.insert(56, BibleBook { name: "Titus".to_string(), chapters: 3 });
        bible_books.insert(57, BibleBook { name: "Philemon".to_string(), chapters: 1 });
        bible_books.insert(58, BibleBook { name: "Hebrews".to_string(), chapters: 13 });
        bible_books.insert(59, BibleBook { name: "James".to_string(), chapters: 5 });
        bible_books.insert(60, BibleBook { name: "1 Peter".to_string(), chapters: 5 });
        bible_books.insert(61, BibleBook { name: "2 Peter".to_string(), chapters: 3 });
        bible_books.insert(62, BibleBook { name: "1 John".to_string(), chapters: 5 });
        bible_books.insert(63, BibleBook { name: "2 John".to_string(), chapters: 1 });
        bible_books.insert(64, BibleBook { name: "3 John".to_string(), chapters: 1 });
        bible_books.insert(65, BibleBook { name: "Jude".to_string(), chapters: 1 });
        bible_books.insert(66, BibleBook { name: "Revelation".to_string(), chapters: 22 });

        let total_files = bible_books.values().map(|book| book.chapters as usize).sum();

        Self {
            base_url: "https://www.wordproaudio.net/bibles/app/audio/42".to_string(),
            output_dir: output_dir.to_string(),
            bible_books,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            progress: Arc::new(Mutex::new(DownloadProgress {
                total_files,
                downloaded: 0,
                current_book: String::new(),
                current_chapter: 0,
                failed: Vec::new(),
            })),
        }
    }

    fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                _ => c,
            })
            .collect()
    }

    async fn download_chapter(&self, book_num: u32, chapter: u32, book_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}/{}.mp3", self.base_url, book_num, chapter);
        let sanitized_book_name = Self::sanitize_filename(book_name);
        let book_dir = Path::new(&self.output_dir).join(&sanitized_book_name);
        
        fs::create_dir_all(&book_dir)?;
        
        let filename = format!("chapter-{:02}.mp3", chapter);
        let file_path = book_dir.join(&filename);
        
        if file_path.exists() {
            println!("⏭️  Skipping {} {} (already exists)", book_name, chapter);
            return Ok(());
        }

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        
        let bytes = response.bytes().await?;
        let mut file = fs::File::create(&file_path)?;
        file.write_all(&bytes)?;
        
        Ok(())
    }

    fn update_progress(&self, book_name: String, chapter: u32, success: bool, error_msg: Option<String>) {
        let mut progress = self.progress.lock().unwrap();
        progress.current_book = book_name.clone();
        progress.current_chapter = chapter;
        
        if success {
            progress.downloaded += 1;
        } else if let Some(error) = error_msg {
            progress.failed.push(format!("{} chapter {}: {}", book_name, chapter, error));
        }
    }

    fn display_progress(&self) {
        let progress = self.progress.lock().unwrap();
        let percentage = (progress.downloaded as f64 / progress.total_files as f64) * 100.0;
        
        let progress_width = 50;
        let filled = (percentage / 100.0 * progress_width as f64) as usize;
        let empty = progress_width - filled;
        let bar = format!("{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        );
        
        print!("\r📖 [{}] {:.1}% | {}/{} files | Current: {} Ch.{} | Failed: {} ",
            bar,
            percentage,
            progress.downloaded,
            progress.total_files,
            progress.current_book,
            progress.current_chapter,
            progress.failed.len()
        );
        std::io::stdout().flush().unwrap();
    }



    pub async fn download_all(&self, max_concurrent: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting Luganda Bible audio download");
        println!("📁 Output directory: {}", self.output_dir);
        println!("📊 Total files to download: {}", self.progress.lock().unwrap().total_files);
        println!();

        fs::create_dir_all(&self.output_dir)?;

        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
        let mut tasks = Vec::new();

        for book_num in 1..=66 {
            if let Some(book) = self.bible_books.get(&book_num) {
                for chapter in 1..=book.chapters {
                    let semaphore = Arc::clone(&semaphore);
                    let downloader = self.clone();
                    let book_name = book.name.clone();
                    
                    let task = tokio::spawn(async move {
                        let _permit = semaphore.acquire().await.unwrap();
                        
                        let result = downloader.download_chapter(book_num, chapter, &book_name).await;
                        
                        match result {
                            Ok(_) => {
                                downloader.update_progress(book_name.clone(), chapter, true, None);
                                downloader.display_progress();
                            }
                            Err(e) => {
                                downloader.update_progress(book_name.clone(), chapter, false, Some(e.to_string()));
                                downloader.display_progress();
                                eprintln!("\n❌ Failed {} chapter {}: {}", book_name, chapter, e);
                            }
                        }
                    });
                    
                    tasks.push(task);
                }
            }
        }

        let progress_task = {
            let downloader_clone = self.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    downloader_clone.display_progress();
                    
                    let prog = downloader_clone.progress.lock().unwrap();
                    if prog.downloaded + prog.failed.len() >= prog.total_files {
                        break;
                    }
                }
            })
        };

        for task in tasks {
            task.await?;
        }

        progress_task.abort();
        
        let final_progress = self.progress.lock().unwrap();
        println!("\n\n🎉 Download completed!");
        println!("✅ Successfully downloaded: {} files", final_progress.downloaded);
        println!("❌ Failed downloads: {} files", final_progress.failed.len());
        
        if !final_progress.failed.is_empty() {
            println!("\n📋 Failed downloads:");
            for failure in &final_progress.failed {
                println!("   • {}", failure);
            }
        }

        Ok(())
    }
}

impl Clone for BibleAudioDownloader {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            output_dir: self.output_dir.clone(),
            bible_books: self.bible_books.clone(),
            client: self.client.clone(),
            progress: Arc::clone(&self.progress),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let output_dir = if args.len() > 1 {
        &args[1]
    } else {
        "./luganda_bible_audio"
    };

    let downloader = BibleAudioDownloader::new(output_dir);
    
    println!("🌍 Luganda Bible Audio Downloader");
    println!("📂 Files will be saved to: {}", output_dir);
    println!("🔗 Source: wordproaudio.net");
    println!();
    
    downloader.download_all(10).await?;
    
    println!("\n📖 Bible structure created:");
    println!("   📁 {}/", output_dir);
    println!("      📁 Genesis/");
    println!("         🎵 chapter-01.mp3");
    println!("         🎵 chapter-02.mp3");
    println!("         ...");
    println!("      📁 Exodus/");
    println!("         🎵 chapter-01.mp3");
    println!("         ...");
    println!("      📁 Revelation/");
    println!("         🎵 chapter-01.mp3");
    println!("         🎵 chapter-22.mp3");

    Ok(())
}
