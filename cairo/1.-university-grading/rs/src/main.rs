fn main() {
    let student_grades: [usize; 50] = [
        72, 53, 15, 54, 21, 58, 35, 52, 70, 66, 2, 13, 70, 83, 60, 5, 87, 53, 9, 32, 23, 79, 78,
        37, 68, 15, 97, 1, 21, 19, 49, 15, 8, 11, 59, 66, 67, 50, 83, 98, 72, 80, 53, 54, 21, 40,
        93, 16, 75, 67,
    ];
    let mut avg: usize = 0;
    for elem in &student_grades {
        avg += elem;
    }
    avg = avg / student_grades.len();

    let mut above_avg_students = 0;
    for elem in &student_grades {
        if elem > &avg {
            above_avg_students += 1;
        }
    }

    println!("Grades avg: {}", avg);
    println!("Total of above avg students: {}", above_avg_students);
}
