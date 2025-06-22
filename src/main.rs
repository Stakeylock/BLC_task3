
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
            },
            None => "N/A (No subjects)".to_string(),
        }
    }

    fn print_report_card(&self) {
        println!("\n--- Student Report Card ---");
        println!("Student Name: {}", self.name);
        println!("Total Marks: {:.2}", self.total_marks);
        println!("Number of Subjects: {}", self.num_subjects);

        match self.calculate_average() {
            Some(average) => {
                println!("Average Marks: {:.2}", average);
                println!("Grade: {}", self.assign_grade());
            },
            None => {
                println!("Average Marks: N/A");
                println!("Grade: N/A (No subjects)");
            },
        }
        println!("---------------------------\n");
    }
}

fn main() {
    println!("Rust Student Report Card Application\n");

    let student1 = Student::new(
        "Ramesh Mishra".to_string(),
        450.0,
        5,
    );
    student1.print_report_card();

    let student2 = Student::new(
        "Ketan Bohra".to_string(),
        320.0,
        5,
    );
    student2.print_report_card();

    let student3 = Student::new(
        "Narmada Chaudhary".to_string(),
        200.0,
        4,
    );
    student3.print_report_card();

    let student4 = Student::new(
        "Thomas Johnson".to_string(),
        0.0,
        0,
    );
    student4.print_report_card();
}
