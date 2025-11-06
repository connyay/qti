# QTI Generator

A schema-driven QTI (Question and Test Interoperability) file generator written in Rust. This tool converts simple markdown-style text files into valid QTI 1.2 XML packages that can be imported into learning management systems like Canvas.

## Features

- **Type-safe XML generation**: Uses Rust's type system to ensure valid QTI structure
- **Multiple question types**: Supports multiple choice, true/false, multiple answer, short answer, numerical, essay, and file upload questions
- **Schema validation**: Validates generated XML against QTI 1.2 specifications
- **Canvas extensions**: Optional Canvas-specific metadata fields
- **Simple input format**: Uses an intuitive text-based format similar to text2qti
- **Package generation**: Creates complete QTI zip packages with manifest files

## Installation

```bash
cargo build --release
```

## Usage

### Show example input format

```bash
cargo run -- example
```

### Generate QTI package from text file

```bash
# Generate complete QTI zip package
cargo run -- generate --input quiz.txt

# Generate XML only (no zip)
cargo run -- generate --input quiz.txt --xml-only

# Include Canvas-specific extensions
cargo run -- generate --input quiz.txt --canvas

# Skip validation
cargo run -- generate --input quiz.txt --skip-validation
```

### Validate existing QTI XML

```bash
cargo run -- validate --file quiz.xml
```

## Input Format

The input format uses simple markdown-style syntax inspired by text2qti:

```
title: Sample Quiz

1. What is 2 + 2?
a) 3
*b) 4
c) 5
d) 6

2. Select all prime numbers:
[*] 2
[*] 3
[ ] 4
[*] 5

3. What is the capital of France?
* Paris
* paris

4. What is π to 2 decimal places?
= 3.14 ± 0.01

5. Explain quantum mechanics.
___

6. Upload your assignment.
^^^
```

### Question Type Syntax

- **Multiple Choice**: `*a)` for correct, `a)` for incorrect
- **Multiple Answer**: `[*]` for correct, `[ ]` for incorrect
- **Short Answer**: `* answer` (multiple acceptable answers allowed)
- **Numerical**: `= value ± margin`
- **Essay**: `___` (3+ underscores)
- **File Upload**: `^^^` (3+ carets)

## Architecture

The project is structured with clean separation of concerns:

- `parser`: Parses text input into internal representation
- `types`: Core data structures for assessments and questions
- `builder`: Type-safe XML element builders
- `generator`: Converts assessments to QTI XML
- `validator`: Schema validation against QTI 1.2
- `exporter`: Creates QTI packages with manifest
- `schema`: XSD-based schema definitions

## Key Improvements Over text2qti

This implementation improves upon the text2qti approach by:

1. **Type Safety**: Using Rust's type system instead of string templates
2. **Schema Validation**: Validates against QTI 1.2 XSD specifications
3. **Better Error Messages**: Clear error reporting with line numbers
4. **Performance**: Rust's speed for large assessment generation
5. **Maintainability**: Clean module separation and builder pattern

## Example

```bash
# Create a quiz file
echo "title: My Quiz

1. What is 2 + 2?
*a) 4
b) 5

2. Name a primary color.
* red
* blue
* yellow" > my_quiz.txt

# Generate QTI package
cargo run -- generate --input my_quiz.txt

# Result: my_quiz.zip ready for LMS import
```

## License

MIT

## Acknowledgments

This project was inspired by Geoffrey Poore's text2qti project, reimplemented in Rust with a focus on type safety and schema validation.