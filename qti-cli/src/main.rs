use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand};
use qti_lib::{Exporter, Generator, Parser};
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser, Debug)]
#[command(name = "qti")]
#[command(author = "Connor Hindley")]
#[command(version = "0.1.0")]
#[command(about = "A schema-driven QTI file generator", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse a text file and generate QTI XML
    Generate {
        /// Input text file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path (defaults to input name with .zip extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Include Canvas-specific extensions
        #[arg(long)]
        canvas: bool,

        /// Output XML only (no zip packaging)
        #[arg(long)]
        xml_only: bool,

        /// Skip validation
        #[arg(long)]
        skip_validation: bool,
    },

    /// Validate an existing QTI XML file
    Validate {
        /// XML file to validate
        #[arg(short, long)]
        file: PathBuf,
    },

    /// Show example input format
    Example,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Generate {
            input,
            output,
            canvas,
            xml_only,
            skip_validation,
        } => {
            generate_qti(input, output, canvas, xml_only, skip_validation)?;
        }
        Commands::Validate { file } => {
            validate_file(file)?;
        }
        Commands::Example => {
            show_example();
        }
    }

    Ok(())
}

fn generate_qti(
    input: PathBuf,
    output: Option<PathBuf>,
    canvas: bool,
    xml_only: bool,
    skip_validation: bool,
) -> Result<()> {
    println!("Reading input file: {}", input.display());

    let content = fs::read_to_string(&input)?;

    let parser = Parser::new();
    let mut assessment = parser.parse(&content)?;

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("assessment");
    assessment.identifier = stem.to_string();

    println!("Parsed {} questions", assessment.questions.len());

    if xml_only {
        let generator = if canvas {
            Generator::new().with_canvas_extensions()
        } else {
            Generator::new()
        };

        let xml = generator.generate(&assessment)?;

        let output_path = output.unwrap_or_else(|| input.with_extension("xml"));

        fs::write(&output_path, xml)?;
        println!("Generated QTI XML: {}", output_path.display());
    } else {
        let mut exporter = if canvas {
            Exporter::new().with_canvas_extensions()
        } else {
            Exporter::new()
        };

        if skip_validation {
            exporter = exporter.skip_validation();
        }

        let output_path = output.unwrap_or_else(|| input.with_extension("zip"));

        exporter.export_to_file(&assessment, &output_path)?;
        println!("Generated QTI package: {}", output_path.display());
    }

    Ok(())
}

fn validate_file(file: PathBuf) -> Result<()> {
    println!("Validating file: {}", file.display());

    let content = fs::read_to_string(&file)?;

    let validator = qti_lib::validator::Validator::new();
    validator.validate_xml(&content)?;

    println!("✓ Valid QTI XML");

    let element = xmltree::Element::parse(content.as_bytes())?;
    validator.validate_completeness(&element)?;

    println!("✓ All required elements present");

    Ok(())
}

fn show_example() {
    println!("QTI Generator - Example Input Format");
    println!("====================================\n");
    println!("title: Sample Quiz");
    println!();
    println!("1. What is 2 + 2?");
    println!("a) 3");
    println!("*b) 4");
    println!("c) 5");
    println!("d) 6");
    println!("feedback: Great job!");
    println!();
    println!("2. Select all prime numbers:");
    println!("[*] 2");
    println!("[*] 3");
    println!("[ ] 4");
    println!("[*] 5");
    println!("[ ] 6");
    println!();
    println!("3. What is the capital of France?");
    println!("* Paris");
    println!("* paris");
    println!();
    println!("4. What is π to 2 decimal places?");
    println!("= 3.14 ± 0.01");
    println!();
    println!("5. Explain the theory of relativity.");
    println!("___");
    println!();
    println!("6. Upload your assignment.");
    println!("^^^");
    println!();
    println!("Legend:");
    println!("-------");
    println!("*x) or *)     - Correct choice (multiple choice)");
    println!("x) or )       - Incorrect choice");
    println!("[*]           - Correct choice (multiple answer)");
    println!("[ ]           - Incorrect choice (multiple answer)");
    println!("* answer      - Acceptable answer (short answer)");
    println!("= num ± margin - Numerical answer with margin");
    println!("___           - Essay question");
    println!("^^^           - File upload");
}
