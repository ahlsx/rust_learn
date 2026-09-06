#[derive(Debug)]
struct Student {
    name: String,
    GPA: f32,
}

fn main() {
    let students = vec!["A 2.1", "B 3.1", "C 4.1", "D 1.1"];

    let good_students: Vec<Student> = students
        .iter()
        .map(|s: &&str| {
            let mut s = s.split(" ");
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student {
                name: name,
                GPA: gpa,
            })
        })
        .flatten()
        .filter(|s| s.GPA > 3.0)
        .collect();

    for s in good_students {
        println!("{:?}", s);
    }
}
