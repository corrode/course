#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: String) -> Self {
        todo!()
    }
    
    fn add_grade(&mut self, grade: f64) {
        todo!()
    }
    
    fn average(&self) -> Option<f64> {
        todo!()
    }
    
    fn letter_grade(&self) -> Option<char> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_student() {
        let mut student = Student::new("Alice".to_string());
        student.add_grade(85.0);
        student.add_grade(90.0);
        student.add_grade(78.0);
        
        assert_eq!(student.average(), Some(84.33333333333333));
        assert_eq!(student.letter_grade(), Some('B'));
    }
}