#![deny(clippy::all)]

use tokio::{fs, task};
use text_splitter::{ChunkConfig, TextSplitter};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Page {
  pub page: u32,
  pub text: String,
}

#[napi(object)]
pub struct TextChunk {
  pub id: u32,
  pub text: String,
}

#[napi]
pub async fn extract_text_from_pdf(path: String) -> napi::Result<String> {
  let bytes = read_file_async(&path).await?;
  let text = extract_text_from_bytes(&path, bytes).await?;
  let cleaned = clean_text(&text);
  Ok(cleaned)
}

#[napi]
pub async fn extract_text_pages(path: String) -> napi::Result<Vec<Page>> {
  let bytes = read_file_async(&path).await?;
  let text = extract_text_from_bytes(&path, bytes).await?;
  Ok(split_text_into_pages(&text))
}

#[napi]
pub async fn extract_text_chunks(
  path: String,
  chunk_size: u32,
  chunk_overlap: u32,
) -> napi::Result<Vec<TextChunk>> {
  let bytes = read_file_async(&path).await?;
  let text = extract_text_from_bytes(&path, bytes).await?;
  let cleaned = clean_text(&text);

  let config = ChunkConfig::new(chunk_size as usize)
    .with_overlap(chunk_overlap as usize)
    .map_err(|e| napi::Error::from_reason(format!(
      "Invalid chunk config (chunk_size={}, overlap={}): {}",
      chunk_size, chunk_overlap, e
    )))?;

  let splitter = TextSplitter::new(config);
  let result = splitter
    .chunks(&cleaned)
    .into_iter()
    .enumerate()
    .map(|(i, text)| TextChunk {
      id: i as u32,
      text: text.to_string(),
    })
    .collect();

  Ok(result)
}

/// Async file read with descriptive error
async fn read_file_async(path: &str) -> napi::Result<Vec<u8>> {
  fs::read(path)
    .await
    .map_err(|e| napi::Error::from_reason(format!(
      "Failed to read file at '{}': {}",
      path, e
    )))
}

/// Offload PDF parsing to a blocking thread, with context
async fn extract_text_from_bytes(path: &str, bytes: Vec<u8>) -> napi::Result<String> {
  task::spawn_blocking(move || pdf_extract::extract_text_from_mem(&bytes))
    .await
    .map_err(|e| napi::Error::from_reason(format!(
      "PDF extraction thread panicked for '{}': {}",
      path, e
    )))?
    .map_err(|e| napi::Error::from_reason(format!(
      "Failed to extract PDF from '{}': {}",
      path, e
    )))
}

/// Clean text by removing numeric-only lines
fn clean_text(text: &str) -> String {
  text.lines()
    .filter(|line| !line.trim().chars().all(|c| c.is_numeric()))
    .collect::<Vec<_>>()
    .join("\n")
}

/// Parse pages by detecting numeric page numbers
fn split_text_into_pages(text: &str) -> Vec<Page> {
  let mut pages = Vec::new();
  let mut current_page = 0;
  let mut buffer = String::new();

  for (_i, line) in text.lines().enumerate() {
    let trimmed = line.trim();

    if let Ok(parsed_page) = trimmed.parse::<u32>() {
      if current_page > 0 {
        pages.push(Page {
          page: current_page,
          text: buffer.trim().to_string(),
        });
        buffer.clear();
      }
      current_page = parsed_page;
    } else {
      buffer.push_str(line);
    }
  }

  if current_page > 0 && !buffer.trim().is_empty() {
    pages.push(Page {
      page: current_page,
      text: buffer.trim().to_string(),
    });
  }

  pages
}
