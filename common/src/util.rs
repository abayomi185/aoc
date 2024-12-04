use std::{
    fs,
    ops::{Add, Sub},
    path::Path,
    process::{Command, Stdio},
};

pub fn print_green(message: &str) {
    println!("\x1b[32m{}\x1b[0m", message);
}

pub fn read_input(file_path: &str) -> String {
    let input_path = Path::new(file_path).parent().unwrap().join("input.txt");
    let cargo_output = Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute cargo metadata");
    let metadata: serde_json::Value =
        serde_json::from_slice(&cargo_output.stdout).expect("Failed to parse JSON");
    let workspace_root = metadata
        .get("workspace_root")
        .and_then(serde_json::Value::as_str)
        .expect("Workspace root not found in metadata");

    let full_path = Path::new(workspace_root).join(input_path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|_| panic!("Error reading input file: {}", full_path.display()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
