use crate::snippet::Snippet;
use std::fs;
use std::path::PathBuf;

fn storage_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".snip-stash")
}

fn storage_path() -> PathBuf {
    storage_dir().join("snippets.json")
}

pub fn load_snippets() -> Vec<Snippet> {
    let path = storage_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_snippets(snippets: &[Snippet]) -> std::io::Result<()> {
    let dir = storage_dir();
    fs::create_dir_all(&dir)?;
    let data = serde_json::to_string_pretty(snippets)?;
    fs::write(storage_path(), data)?;
    Ok(())
}

pub fn starter_snippets() -> Vec<Snippet> {
    vec![
        Snippet::new(
            "Rust Error Handling with ?".into(),
            "rust".into(),
            vec!["error-handling".into(), "basics".into()],
            r#"use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    let contents = fs::read_to_string("config.toml")?;
    Ok(contents)
}

fn main() {
    match read_config() {
        Ok(cfg) => println!("{cfg}"),
        Err(e) => eprintln!("Failed: {e}"),
    }
}"#.into(),
        ),
        Snippet::new(
            "Rust Iterator Chaining".into(),
            "rust".into(),
            vec!["iterators".into(), "functional".into()],
            r#"let names = vec!["alice", "bob", "charlie", "diana"];

let result: Vec<String> = names
    .iter()
    .filter(|n| n.len() > 3)
    .map(|n| n.to_uppercase())
    .collect();

// result: ["ALICE", "CHARLIE", "DIANA"]
println!("{result:?}");"#.into(),
        ),
        Snippet::new(
            "Rust Async Spawn".into(),
            "rust".into(),
            vec!["async".into(), "tokio".into()],
            r#"use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task done");
        42
    });

    let result = handle.await.unwrap();
    println!("Got: {result}");
}"#.into(),
        ),
        Snippet::new(
            "Rust Enum + Match".into(),
            "rust".into(),
            vec!["enums".into(), "pattern-matching".into()],
            r#"enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn handle(cmd: Command) {
    match cmd {
        Command::Quit => println!("Quitting"),
        Command::Echo(msg) => println!("{msg}"),
        Command::Move { x, y } => println!("Move to ({x}, {y})"),
    }
}"#.into(),
        ),
        Snippet::new(
            "Rust Trait Implementation".into(),
            "rust".into(),
            vec!["traits".into(), "oop".into()],
            r#"trait Greet {
    fn hello(&self) -> String;
}

struct User { name: String }

impl Greet for User {
    fn hello(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

let u = User { name: "raz".into() };
println!("{}", u.hello());"#.into(),
        ),
        Snippet::new(
            "Find Files by Pattern".into(),
            "bash".into(),
            vec!["find".into(), "search".into()],
            r#"# Find all .rs files modified in the last 7 days
find . -name '*.rs' -mtime -7 -type f

# Find files larger than 10MB
find /var/log -size +10M -type f

# Find and delete .tmp files (dry run first)
find . -name '*.tmp' -type f -print
# find . -name '*.tmp' -type f -delete"#.into(),
        ),
        Snippet::new(
            "Grep Patterns".into(),
            "bash".into(),
            vec!["grep".into(), "search".into(), "regex".into()],
            r#"# Recursive grep with line numbers
grep -rn 'TODO' --include='*.rs' .

# Count matches per file
grep -rc 'fn ' --include='*.rs' . | grep -v ':0$'

# Show context around matches
grep -B2 -A2 'panic!' src/

# Extract emails from a file
grep -oE '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}' file.txt"#.into(),
        ),
        Snippet::new(
            "Awk Column Processing".into(),
            "bash".into(),
            vec!["awk".into(), "text-processing".into()],
            r#"# Print second column
awk '{print $2}' data.txt

# Sum a numeric column
awk '{sum += $3} END {print sum}' data.txt

# Filter rows where col1 > 100
awk '$1 > 100 {print $0}' data.txt

# Custom delimiter (CSV)
awk -F',' '{print $1, $3}' data.csv"#.into(),
        ),
        Snippet::new(
            "Git Rebase & Fixup".into(),
            "git".into(),
            vec!["rebase".into(), "history".into()],
            r#"# Interactive rebase last 5 commits
git rebase -i HEAD~5

# Autosquash fixup commits
git commit --fixup=<sha>
git rebase -i --autosquash main

# Rebase onto updated main
git fetch origin
git rebase origin/main

# Abort if it goes wrong
git rebase --abort"#.into(),
        ),
        Snippet::new(
            "Git Stash Workflows".into(),
            "git".into(),
            vec!["stash".into(), "workflow".into()],
            r#"# Stash with a message
git stash push -m "WIP: feature X"

# List stashes
git stash list

# Apply most recent stash (keep in list)
git stash apply

# Pop and apply (remove from list)
git stash pop

# Apply a specific stash
git stash apply stash@{2}

# Stash only unstaged changes
git stash push --keep-index"#.into(),
        ),
        Snippet::new(
            "Systemd Service Template".into(),
            "config".into(),
            vec!["systemd".into(), "sysadmin".into()],
            r#"[Unit]
Description=My Custom Service
After=network.target

[Service]
Type=simple
User=raz
WorkingDirectory=/home/raz/project
ExecStart=/usr/bin/my-binary --flag
Restart=on-failure
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target"#.into(),
        ),
        Snippet::new(
            "Python List Comprehension Patterns".into(),
            "python".into(),
            vec!["comprehension".into(), "functional".into()],
            r#"# Nested comprehension — flatten
flat = [x for row in matrix for x in row]

# Dict comprehension
freq = {ch: s.count(ch) for ch in set(s)}

# Conditional filter + transform
evens_sq = [x**2 for x in range(20) if x % 2 == 0]

# Walrus operator in comprehension (3.8+)
results = [y for x in data if (y := process(x)) is not None]"#.into(),
        ),
        Snippet::new(
            "Disk & Memory Quick Checks".into(),
            "bash".into(),
            vec!["sysadmin".into(), "monitoring".into()],
            r#"# Disk usage top 10
du -sh /* 2>/dev/null | sort -rh | head -10

# Memory overview
free -h

# Per-process memory (top 10)
ps aux --sort=-%mem | head -11

# Filesystem usage
df -hT | grep -v tmpfs

# GPU VRAM (AMD)
cat /sys/class/drm/card*/device/mem_info_vram_used
cat /sys/class/drm/card*/device/mem_info_vram_total"#.into(),
        ),
        Snippet::new(
            "Tar & Compression".into(),
            "bash".into(),
            vec!["archive".into(), "sysadmin".into()],
            r#"# Create gzipped tar
tar czf archive.tar.gz dir/

# Extract
tar xzf archive.tar.gz

# List contents without extracting
tar tzf archive.tar.gz

# Create with zstd (faster)
tar --zstd -cf archive.tar.zst dir/

# Extract specific file
tar xzf archive.tar.gz path/to/file.txt"#.into(),
        ),
        Snippet::new(
            "Rust Struct Update Syntax".into(),
            "rust".into(),
            vec!["structs".into(), "basics".into()],
            r#"#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    verbose: bool,
    retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8080,
            verbose: false,
            retries: 3,
        }
    }
}

// Override only what you need
let cfg = Config {
    port: 9090,
    verbose: true,
    ..Config::default()
};"#.into(),
        ),
    ]
}
