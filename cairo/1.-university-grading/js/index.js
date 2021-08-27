const studentGrades = [
  72, 53, 15, 54, 21, 58, 35, 52, 70, 66, 2, 13, 70, 83, 60, 5, 87, 53, 9, 32, 23, 79, 78,
  37, 68, 15, 97, 1, 21, 19, 49, 15, 8, 11, 59, 66, 67, 50, 83, 98, 72, 80, 53, 54, 21, 40,
  93, 16, 75, 67,
]

let avg = studentGrades.reduce((prev, current) => prev + current) / studentGrades.length;

let aboveAvgStudents = studentGrades.filter(grade => grade > avg).length;


console.log("Grades avg: ", parseInt(avg, 10));
console.log("Total of above avg students: ", aboveAvgStudents);