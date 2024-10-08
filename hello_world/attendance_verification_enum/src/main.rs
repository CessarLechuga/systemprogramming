#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student {
    name: String,
    grade: GradeLevel,
    major: Major
}

impl Student {
    fn new(name: String, grade: GradeLevel, major: Major) -> Self {
        Student {
            name,
            grade,
            major,
        }
    }

    fn introduce_yourself(&self) {
        let grade_str = match self.grade {
            GradeLevel::Bachelor => "Bachelor's",
            GradeLevel::Master => "Master's",
            GradeLevel::PhD => "PhD",
        };

        let major_str = match self.major {
            Major::ComputerScience => "Computer Science",
            Major::ElectricalEngineering => "Electrical Engineering",
        };

        println!("Hello, I'm {}. I'm a {} student studying {} at the {} level.", 
                 self.name, grade_str, major_str, grade_str);
    }
}

fn main() {
    let s1 = Student::new("John".to_string(),
        GradeLevel::Bachelor,
        Major::ComputerScience);
    s1.introduce_yourself();
}