# PDF Text Extractor

A fast, native Node.js module to extract and process text from PDF files using Rust and N-API. Built with [Tokio](https://tokio.rs/), [`pdf-extract`](https://docs.rs/pdf-extract), and [`text-splitter`](https://crates.io/crates/text-splitter), this package provides efficient and easy-to-use async APIs to:

## Features
- High-performance native code (Rust)
- Asynchronous functions (non-blocking I/O)
- Useful for LLM pipelines and search indexing
- Extract cleaned text from PDF files
- Split PDF text into pages with automatic page number detection
- Generate overlapping text chunks with configurable sizes
- TypeScript support

## Installation

```bash
npm install pdf-text-extractor
# or
yarn add pdf-text-extractor
```

## Usage

### Basic Text Extraction

**JavaScript**
```javascript
const { extractTextFromPdf } = require('pdf-text-extractor');

async function main() {
  try {
    const text = await extractTextFromPdf('./document.pdf');
    console.log('Cleaned PDF text:', text);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

main();
```

**TypeScript**
```typescript
import { extractTextFromPdf } from 'pdf-text-extractor';

async function main() {
  try {
    const text: string = await extractTextFromPdf('./document.pdf');
    console.log('Cleaned PDF text:', text);
  } catch (error) {
    console.error('Error:', (error as Error).message);
  }
}

main();
```

### Page-based Extraction

**JavaScript**
```javascript
const { extractTextPages } = require('pdf-text-extractor');

async function extractPages() {
  try {
    const pages = await extractTextPages('./document.pdf');
    pages.forEach(page => {
      console.log(`Page ${page.page}:`);
      console.log(page.text);
      console.log('\n---\n');
    });
  } catch (error) {
    console.error('Error:', error.message);
  }
}

extractPages();
```
**TypeScript**
```typescript
import { extractTextPages, Page } from 'pdf-text-extractor';

async function extractPages() {
  try {
    const pages: Page[] = await extractTextPages('./document.pdf');
    pages.forEach((page: Page) => {
      console.log(`Page ${page.page}:`);
      console.log(page.text);
      console.log('\n---\n');
    });
  } catch (error) {
    console.error('Error:', (error as Error).message);
  }
}

extractPages();
```

### Text Chunking with Overlaps

**JavaScript**
```javascript
const { extractTextChunks } = require('pdf-text-extractor');

async function chunkText() {
  try {
    const chunks = await extractTextChunks('./document.pdf', 1000, 200);
    chunks.forEach(chunk => {
      console.log(`Chunk ${chunk.id}:`);
      console.log(chunk.text);
      console.log('\n=====\n');
    });
  } catch (error) {
    console.error('Error:', error.message);
  }
}

chunkText();
```

**TypeScript**
```typescript
import { extractTextChunks, TextChunk } from 'pdf-text-extractor';

async function chunkText() {
  try {
    const chunks: TextChunk[] = await extractTextChunks('./document.pdf', 1000, 200);
    chunks.forEach((chunk: TextChunk) => {
      console.log(`Chunk ${chunk.id}:`);
      console.log(chunk.text);
      console.log('\n=====\n');
    });
  } catch (error) {
    console.error('Error:', (error as Error).message);
  }
}

chunkText();
```

## Types

```typescript
type Page = {
  page: number;
  text: string;
};

type TextChunk = {
  id: number;
  text: string;
};
```

## API Documentation

### `extractTextFromPdf(path: string): Promise<string>`
Extracts and cleans text from a PDF file
- `path`: Path to PDF file
- Returns: Cleaned text with numeric-only lines removed

### `extractTextPages(path: string): Promise<Page[]>`
Extracts text split into pages
```typescript
interface Page {
  page: number;
  text: string;
}
```

### `extractTextChunks(path: string, chunkSize: number, chunkOverlap: number): Promise<TextChunk[]>`
Generates overlapping text chunks
```typescript
interface TextChunk {
  id: number;
  text: string;
}
```
- `chunkSize`: Target chunk size in characters
- `chunkOverlap`: Overlap between chunks (must be < chunkSize)

## Error Handling
All functions throw errors with descriptive messages for:
- File not found or read errors
- PDF parsing failures
- Invalid chunk configurations (overlap >= chunk size)

## Use Cases
- Document understanding and chunking for LLMs
- PDF content extraction for chatbots or search
- Indexing and pre-processing for embeddings

## Processing Details
1. **Text Cleaning**:
   - Removes lines containing only numeric characters
   - Preserves original line breaks and formatting

2. **Page Detection**:
   - Splits text at lines containing only page numbers
   - Handles variable page number positions

3. **Chunking**:
   - Uses semantic-aware splitting (paragraphs/sentences)
   - Maintains context with overlapping chunks
   - Configurable through simple parameters

## Requirements
- Node.js 16+
- Rust (for building from source)

## License
MIT