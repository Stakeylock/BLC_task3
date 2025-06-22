use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
//use rand::Rng; 
use printpdf::{PdfDocument, PdfLayerReference, Point, Line, Mm};
#[derive(Debug)]
struct SubjectMarks {
    subject_name: String,
    first_term: f64,
    second_term: f64,
    third_term: f64,
    final_term: f64,
}

impl SubjectMarks {
    
    /*fn random_marks(subject_name: String) -> SubjectMarks {
        let mut rng = rand::thread_rng();
        SubjectMarks {
            subject_name,
            first_term: rng.gen_range(0.0..=100.0),
            second_term: rng.gen_range(0.0..=100.0),
            third_term: rng.gen_range(0.0..=100.0),
            final_term: rng.gen_range(0.0..=100.0),
        }
    }*/

    fn calculate_subject_average(&self) -> f64 {
        (self.first_term + self.second_term + self.third_term + self.final_term) / 4.0
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    class: String,
    section: String,
    subjects: Vec<SubjectMarks>,
    comment: String,
}

impl Student {
    fn new(name: String, class: String, section: String, subjects: Vec<SubjectMarks>, comment: String) -> Student {
        Student {
            name,
            class,
            section,
            subjects,
            comment,
        }
    }

    fn calculate_total_marks(&self) -> f64 {
        self.subjects.iter().map(|s| s.calculate_subject_average()).sum()
    }

    fn calculate_overall_average(&self) -> Option<f64> {
        if self.subjects.is_empty() {
            None
        } else {
            let total_subject_averages: f64 = self.subjects.iter().map(|s| s.calculate_subject_average()).sum();
            Some(total_subject_averages / self.subjects.len() as f64)
        }
    }

    fn assign_grade(&self) -> String {
        match self.calculate_overall_average() {
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
        println!("  Class: {} Section: {}", self.class, self.section);
        println!("\n  Subjects:");
        for subject in &self.subjects {
            println!(
                "    {:<10}: 1st Term: {:.2}, 2nd Term: {:.2}, 3rd Term: {:.2}, Final Term: {:.2}, Avg: {:.2}",
                subject.subject_name,
                subject.first_term,
                subject.second_term,
                subject.third_term,
                subject.final_term,
                subject.calculate_subject_average()
            );
        }
        println!("\n  Total Marks: {:.2}", self.calculate_total_marks());
        match self.calculate_overall_average() {
            Some(avg) => println!("  Overall Average: {:.2}", avg),
            None => println!("  Overall Average: N/A"),
        }
        println!("  Grade: {}", self.assign_grade());
        println!("  Comment: {}", self.comment);
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
        let header_font_size = 16.0;
        let detail_font_size = 12.0;
        let subject_header_font_size = 10.0;

        let mut current_y = 270.0;

        // School Name and Report Card Title
        current_layer.use_text(
            "School Name",
            title_font_size,
            Mm(80.0),
            Mm(current_y),
            &font,
        );
        current_y -= 15.0;
        current_layer.use_text(
            "REPORT CARD",
            header_font_size,
            Mm(80.0),
            Mm(current_y),
            &font,
        );
        current_y -= 25.0;

      
        fn draw_filled_stroked_rectangle(
            layer: &PdfLayerReference,
            x: Mm,
            y: Mm,
            width: Mm,
            height: Mm,
            fill_color: Color,
            outline_color: Color,
            outline_thickness: f64,
        ) {
            let points = vec![
                (Point::new(x, y), false),
                (Point::new(x + width, y), false),
                (Point::new(x + width, y + height), false),
                (Point::new(x, y + height), false),
            ];

            let line = Line {
                points,
                is_closed: true,
                has_fill: true,
                has_stroke: true,
                is_clipping_path: false,
            };

            layer.set_fill_color(fill_color);
            layer.set_outline_color(outline_color);
            layer.set_outline_thickness(outline_thickness);
            layer.add_shape(line);
        }


        // Student Info Box
        let rect_x = Mm(15.0);
        let rect_y = Mm(current_y - 30.0);
        let rect_width = Mm(180.0);
        let rect_height = Mm(35.0);

        draw_filled_stroked_rectangle(
            &current_layer,
            rect_x,
            rect_y,
            rect_width,
            rect_height,
            Color::Rgb(Rgb::new(0.9, 0.9, 0.9, None)),
            Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
            0.5,
        );

        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Reset fill color to black for text

        current_layer.use_text(
            format!("Student Name: {}", self.name),
            detail_font_size,
            Mm(20.0),
            Mm(current_y - 5.0),
            &font,
        );
        current_layer.use_text(
            format!("Class: {} Section: {}", self.class, self.section),
            detail_font_size,
            Mm(20.0),
            Mm(current_y - 20.0),
            &font,
        );
        current_y -= 40.0;

        // Subjects Header
        let rect_x = Mm(15.0);
        let rect_y = Mm(current_y - 10.0);
        let rect_width = Mm(180.0);
        let rect_height = Mm(10.0);

        draw_filled_stroked_rectangle(
            &current_layer,
            rect_x,
            rect_y,
            rect_width,
            rect_height,
            Color::Rgb(Rgb::new(0.3, 0.1, 0.6, None)),
            Color::Rgb(Rgb::new(0.3, 0.1, 0.6, None)),
            0.1, // Small thickness for header
        );

        current_layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None))); // White text

        current_layer.use_text("Subjects", subject_header_font_size, Mm(20.0), Mm(current_y - 5.0), &font);
        current_layer.use_text("1st Term", subject_header_font_size, Mm(60.0), Mm(current_y - 5.0), &font);
        current_layer.use_text("2nd Term", subject_header_font_size, Mm(90.0), Mm(current_y - 5.0), &font);
        current_layer.use_text("3rd Term", subject_header_font_size, Mm(120.0), Mm(current_y - 5.0), &font);
        current_layer.use_text("Final Term", subject_header_font_size, Mm(150.0), Mm(current_y - 5.0), &font);
        current_y -= 10.0;

        // Subject Marks
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Black text for marks
        for subject in &self.subjects {
            let rect_x = Mm(15.0);
            let rect_y = Mm(current_y - 10.0);
            let rect_width = Mm(180.0);
            let rect_height = Mm(10.0);

            draw_filled_stroked_rectangle(
                &current_layer,
                rect_x,
                rect_y,
                rect_width,
                rect_height,
                Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)),
                Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
                0.1,
            );

            current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Text color for subject marks

            current_layer.use_text(subject.subject_name.clone(), detail_font_size, Mm(20.0), Mm(current_y - 5.0), &font);
            current_layer.use_text(format!("{:.2}", subject.first_term), detail_font_size, Mm(60.0), Mm(current_y - 5.0), &font);
            current_layer.use_text(format!("{:.2}", subject.second_term), detail_font_size, Mm(90.0), Mm(current_y - 5.0), &font);
            current_layer.use_text(format!("{:.2}", subject.third_term), detail_font_size, Mm(120.0), Mm(current_y - 5.0), &font);
            current_layer.use_text(format!("{:.2}", subject.final_term), detail_font_size, Mm(150.0), Mm(current_y - 5.0), &font);
            current_y -= 10.0;
        }
        current_y -= 10.0;

        // Total Marks and Grade
        let rect_x = Mm(15.0);
        let rect_y = Mm(current_y - 10.0);
        let rect_width = Mm(180.0);
        let rect_height = Mm(10.0);

        draw_filled_stroked_rectangle(
            &current_layer,
            rect_x,
            rect_y,
            rect_width,
            rect_height,
            Color::Rgb(Rgb::new(0.9, 0.9, 0.9, None)),
            Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
            0.1,
        );
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Black text

        current_layer.use_text(
            format!("Total Marks: {:.2}", self.calculate_total_marks()),
            detail_font_size,
            Mm(20.0),
            Mm(current_y - 5.0),
            &font,
        );
        current_layer.use_text(
            format!("Grade: {}", self.assign_grade()),
            detail_font_size,
            Mm(120.0),
            Mm(current_y - 5.0),
            &font,
        );
        current_y -= 15.0;

        // Comment section
        let rect_x = Mm(15.0);
        let rect_y = Mm(current_y - 30.0);
        let rect_width = Mm(180.0);
        let rect_height = Mm(35.0);

        draw_filled_stroked_rectangle(
            &current_layer,
            rect_x,
            rect_y,
            rect_width,
            rect_height,
            Color::Rgb(Rgb::new(0.9, 0.9, 0.9, None)),
            Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
            0.1,
        );
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Black text

        current_layer.use_text(
            "Comment:",
            detail_font_size,
            Mm(20.0),
            Mm(current_y - 5.0),
            &font,
        );
        current_layer.use_text(
            &self.comment,
            detail_font_size,
            Mm(20.0),
            Mm(current_y - 20.0),
            &font,
        );
        current_y -= 50.0;

        // Signatures (placeholders)
        current_layer.use_text(
            "Teacher's Signature: __________________",
            detail_font_size,
            Mm(100.0),
            Mm(current_y),
            &font,
        );
        current_y -= 20.0;
        current_layer.use_text(
            "Parent's Signature: ___________________",
            detail_font_size,
            Mm(20.0),
            Mm(current_y),
            &font,
        );
        current_layer.use_text(
            "Principal Signature: ___________________",
            detail_font_size,
            Mm(100.0),
            Mm(current_y),
            &font,
        );

        let output_filename = format!("report_cards/{}_report_card.pdf", self.name.replace(" ", "_"));
        let mut file_buffer = BufWriter::new(File::create(&output_filename)
            .map_err(|e| format!("Failed to create PDF file '{}': {}", output_filename, e))?);
        doc.save(&mut file_buffer)
            .map_err(|e| format!("Failed to save PDF content: {}", e))?;

        println!("Report card saved to: {}", output_filename);
        Ok(())
    }
}

fn main() {
    println!("Welcome to the Rust Student Report Card Application!\n");

    let common_subjects = vec![
        "English".to_string(),
        "Maths".to_string(),
        "Science".to_string(),
        "Art".to_string(),
        "Reading".to_string(),
        "Writing".to_string(),
        "History".to_string(),
    ];

    // Student for Grade A (90+ overall average)
    let mut student_a_subjects = Vec::new();
    for sub_name in &common_subjects {
        student_a_subjects.push(SubjectMarks {
            subject_name: sub_name.clone(),
            first_term: 95.0,
            second_term: 92.0,
            third_term: 98.0,
            final_term: 93.0,
        });
    }
    let student_a = Student::new(
        "Excellent Emily".to_string(),
        "10A".to_string(),
        "Alpha".to_string(),
        student_a_subjects,
        "An outstanding student with exceptional performance.".to_string(),
    );
    student_a.print_report_card();
    if let Err(e) = student_a.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student_a.name, e);
    }

    // Student for Grade B (75-89 overall average)
    let mut student_b_subjects = Vec::new();
    for sub_name in &common_subjects {
        student_b_subjects.push(SubjectMarks {
            subject_name: sub_name.clone(),
            first_term: 80.0,
            second_term: 78.0,
            third_term: 85.0,
            final_term: 82.0,
        });
    }
    let student_b = Student::new(
        "Bright Ben".to_string(),
        "10B".to_string(),
        "Beta".to_string(),
        student_b_subjects,
        "Consistently performing well, showing good understanding.".to_string(),
    );
    student_b.print_report_card();
    if let Err(e) = student_b.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student_b.name, e);
    }

    // Student for Grade C (60-74 overall average)
    let mut student_c_subjects = Vec::new();
    for sub_name in &common_subjects {
        student_c_subjects.push(SubjectMarks {
            subject_name: sub_name.clone(),
            first_term: 65.0,
            second_term: 60.0,
            third_term: 70.0,
            final_term: 68.0,
        });
    }
    let student_c = Student::new(
        "Average Alex".to_string(),
        "10C".to_string(),
        "Gamma".to_string(),
        student_c_subjects,
        "Meets expectations, with room for improvement in some areas.".to_string(),
    );
    student_c.print_report_card();
    if let Err(e) = student_c.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student_c.name, e);
    }

    // Student for Grade D (Below 60 overall average)
    let mut student_d_subjects = Vec::new();
    for sub_name in &common_subjects {
        student_d_subjects.push(SubjectMarks {
            subject_name: sub_name.clone(),
            first_term: 50.0,
            second_term: 45.0,
            third_term: 55.0,
            final_term: 52.0,
        });
    }
    let student_d = Student::new(
        "Developing David".to_string(),
        "10A".to_string(),
        "Delta".to_string(),
        student_d_subjects,
        "Requires significant support and focused effort to improve grades.".to_string(),
    );
    student_d.print_report_card();
    if let Err(e) = student_d.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student_d.name, e);
    }

    // Student with no subjects (for "N/A" grade)
    let student_no_subjects = Student::new(
        "New Nora".to_string(),
        "10Z".to_string(),
        "Zeta".to_string(),
        vec![], // No subjects
        "New enrollment, awaiting subject assignments.".to_string(),
    );
    student_no_subjects.print_report_card();
    if let Err(e) = student_no_subjects.save_report_card_as_pdf() {
        eprintln!("Error saving PDF for {}: {}", student_no_subjects.name, e);
    }
}
