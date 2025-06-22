use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;

#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f64,
    num_subjects: u32,
}

impl Student {
    fn new(name: String, total_marks: f64, num_subjects: u32) -> Student {
        Student {
            name,
            total_marks,
            num_subjects,
        }
    }

    fn calculate_average(&self) -> Option<f64> {
        if self.num_subjects > 0 {
            Some(self.total_marks / self.num_subjects as f64)
        } else {
            None
        }
    }

    fn assign_grade(&self) -> String {
        match self.calculate_average() {
            Some(average) => {
                if average >= 90.0 {
                    "A".to_string()
                } else if average >= 75.0 {
                    "B".to_string()
                } else if average >= 60.0 {
                    "C".to_string()
                } else {
                    "D".to_string()
                }
            }
            None => "N/A (No subjects)".to_string(),
        }
    }

    fn print_report_card(&self) {
        println!("\n--- Student Report Card ---");
        println!("  Student Name: {}", self.name);
        println!("  Total Marks: {:.2}", self.total_marks);
        println!("  Number of Subjects: {}", self.num_subjects);

        match self.calculate_average() {
            Some(average) => {
                println!("  Average Marks: {:.2}", average);
                println!("  Grade: {}", self.assign_grade());
            }
            None => {
                println!("  Average Marks: N/A");
                println!("  Grade: N/A (No subjects)");
            }
        }
        println!("---------------------------\n");
    }

    fn save_report_card_as_pdf(&self) -> Result<(), Box<dyn Error>> {
        let (doc, page1, layer1) = PdfDocument::new(
            &format!("{} Report Card", self.name),
            Mm(210.0),
            Mm(297.0),
            "Layer 1",
        );
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
            .map_err(|e| format!("Failed to add built-in font to PDF: {}", e))?;

        let title_font_size = 24.0;
        let detail_font_size = 14.0;
        let grade_font_size = 18.0;

        current_layer.use_text(
            "--- Student Report Card ---",
            title_font_size,
            Mm(20.0),
            Mm(270.0),
            &font,
        );

        current_layer.use_text(
            format!("Student Name: {}", self.name),
            detail_font_size,
            Mm(20.0),
            Mm(250.0),
            &font,
        );
        current_layer.use_text(
            format!("Total Marks: {:.2}", self.total_marks),
            detail_font_size,
            Mm(20.0),
            Mm(240.0),
            &font,
        );
        current_layer.use_text(
            format!("Number of Subjects: {}", self.num_subjects),
            detail_font_size,
            Mm(20.0),
            Mm(230.0),
            &font,
        );

        match self.calculate_average() {
            Some(average) => {
                current_layer.use_text(
                    format!("Average Marks: {:.2}", average),
                    detail_font_size,
                    Mm(20.0),
                    Mm(210.0),
                    &font,
                );
                current_layer.use_text(
                    format!("Grade: {}", self.assign_grade()),
                    grade_font_size,
                    Mm(20.0),
                    Mm(200.0),
                    &font,
                );
            }
            None => {
                current_layer.use_text(
                    "Average Marks: N/A",
                    detail_font_size,
                    Mm(20.0),
                    Mm(210.0),
                    &font,
                );
                current_layer.use_text(
                    "Grade: N/A (No subjects)",
                    grade_font_size,
                    Mm(20.0),
                    Mm(200.0),
                    &font,
                );
            }
        }

        let output_filename = format!("{}_report_card.pdf", self.name.replace(" ", "_"));
        let mut file_buffer = BufWriter::new(File::create(&output_filename)
            .map_err(|e| format!("Failed to create PDF file '{}': {}", output_filename, e))?);
        doc.save(&mut file_buffer)
            .map_err(|e| format!("Failed to save PDF content: {}", e))?;

        println!("Report card saved to: {}", output_filename);
        Ok(())
    }
}

fn main() {
    println!("Student report card app\n");

    let student1 = Student::new(
        "Alice Smith".to_string(),
        450.0,
        5,
    );
    student1.print_report_card();
    if let Err(e) = student1.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student1.name, e);
    }


    let student2 = Student::new(
        "Bob Johnson".to_string(),
        380.0,
        5,
    );
    student2.print_report_card();
    if let Err(e) = student2.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student2.name, e);
    }

    let student3 = Student::new(
        "Charlie Brown".to_string(),
        280.0,
        4,
    );
    student3.print_report_card();
    if let Err(e) = student3.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student3.name, e);
    }

    let student4 = Student::new(
        "Diana Prince".to_string(),
        190.0,
        5,
    );
    student4.print_report_card();
    if let Err(e) = student4.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student4.name, e);
    }

    let student5 = Student::new(
        "Eve Adams".to_string(),
        0.0,
        0,
    );
    student5.print_report_card();
    if let Err(e) = student5.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student5.name, e);
    }
}
