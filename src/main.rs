use std::collections::HashMap;
//use std::fmt::Debug;
struct School<T>{
    students: HashMap<String, T>
}
impl<T: Copy + Ord + PartialEq + std::fmt::Debug> School<T>{
    fn new() -> School<T>{
        School{
            students: HashMap::new()
        }
    }
    fn add(&mut self, name: &str, grade: T){
        self.students.insert(name.to_string(), grade);
    }
    fn get_student_grade(&self){
        for (key, value) in &self.students {
            println!("{:?} {:?}", key, value);
        }
    }
    fn grades(&self) -> Vec<T>{
        let mut grades: Vec<T> = self.students.values().cloned().collect();
        grades.sort();
        grades.dedup();
        grades
    }
    fn grade(&self, grade: T) -> Vec<String>{
        
        let sts = &self.students;
        let mut result = Vec::<String>::new();

        for (st, g) in sts {
            if *g == grade {
                result.push(st.to_string())
            }
        }
        result.sort();
        result
        
    }
}
fn main() {
    let mut school = School::new();
    school.add("Alice", 10);
    school.add("Bob", 8);
    school.add("Charlie", 9);
    school.add("David", 9);
    school.add("Eve", 7);
    let mut school2 = School::new();
    school2.add("Teo", "A+");
    school2.add("Ti", "B+");
    school2.add("Tun", "C+");
    school2.add("An", "A+");
    println!("\n");
    println!("****************List of students****************\n");
    school.get_student_grade();
    school2.get_student_grade();
    println!("\n");
    println!("****************List of grade****************\n\n");
    println!("List of grade {:?}", school.grades());
    println!("List of grade {:?}", school2.grades());
    println!("\n");
    println!("****************List of similar grade ***************\n\n");
    let a = school.grade(9);
    let b = school2.grade("A+");
    println!("Student similar grade {:?}", a);
    println!("Student similar grade {:?}", b);
}
#[test]
fn school_with_noone() {
    let new_school = School::<u32>::new();
    assert_eq!(new_school.grades(), vec![]);
}
#[test]
fn student_grade_u32() {
    let mut new_school = School::<u32>::new();
    new_school.add("phongnguyen", 10);
    assert_eq!(new_school.grades(), vec![10]);
}
#[test]
fn student_grade_str(){
    let mut new_school = School::<&str>::new();
    new_school.add("phongnguyen", "A+");
    assert_eq!(new_school.grades(), vec!["A+"]);
}
#[test]
fn student_add_grade_str(){
    let mut new_school = School::<&str>::new();
    assert_eq!(new_school.add("phong", "A+"), new_school.get_student_grade());
}
#[test]
fn student_add_grade_u32(){
    let mut new_school = School::<u32>::new();
    assert_eq!(new_school.add("phong", 10), new_school.get_student_grade());
}
#[test] 
fn student_grade_str_2(){
    let mut new_school = School::<&str>::new();
    new_school.add("phongnguyen", "A+");
    assert_eq!(new_school.grades(), vec!["A+"]);
}
#[test]
fn student_grade_u32_2(){
    let mut new_school = School::<u32>::new();
    new_school.add("phongnguyen", 10);
    assert_eq!(new_school.grades(), vec![10]);
}
#[test]
fn student_dulicate_name_u32(){
    let mut new_school = School::<u32>::new();
    new_school.add("phongnguyen", 10);
    new_school.add("phucnguyen", 10);
    assert_eq!(new_school.grade(10), vec!["phongnguyen", "phucnguyen"]);
}
#[test]
fn student_dulicate_name_str(){
    let mut new_school = School::<&str>::new();
    new_school.add("nguyen van teo", "A+");
    new_school.add("tran van teo", "A+");
    assert_eq!(new_school.grade("A+"), vec!["nguyen van teo", "tran van teo"]);
}