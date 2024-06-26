use arraystring::{typenum::U30, ArrayString};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// Types
pub type Name = ArrayString<U30>;
pub type Age = u8;
pub type Grade = f32;

pub enum SearchOption {
    Name,
    Age,
    Grade
}

impl FromStr for SearchOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '1' => Ok(SearchOption::Name),
            '2' => Ok(SearchOption::Age),
            '3' => Ok(SearchOption::Grade),
            _ => Err(())
        }
    }
}

/// Generic structure chosen for this TAD
pub struct Student {
    pub name: Name, // Size for each name is fixed
    pub age: Age,
    pub grade: Grade,
}

/// Needed for use Student in print!()
impl fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Student {}, Age: {}, Grade: {}",
            self.name, self.age, self.grade
        )
    }
}

impl Display for StudentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res: String = String::new();
        for i in self.0.iter() {
            res.push_str(format!("{i}").as_str());
        }
        write!(f, "{res}")
    }
}

/// List/Vec of Student structure
pub struct StudentList(Vec<Student>);

impl StudentList {
    /// Create a new empty StudentList.
    ///
    /// For this TAD, this returns a useless vector. \
    /// You cannot add a student without create with StudentList::with_capacity()
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::new();
    /// assert!(l.is_empty());
    /// ```
    pub fn new() -> StudentList {
        StudentList(vec![])
    }

    /// Create a new StudentList with capacity number_of_items.
    ///
    /// For this TAD, you cannot add a student if l.len() >= l.with_capacity
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// ```
    pub fn with_capacity(number_of_items: u8) -> StudentList {
        StudentList(Vec::<Student>::with_capacity(usize::from(number_of_items)))
    }

    /// Add a new Student to StudentList only if StudentList.len() < StudentList.capacity().
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert_eq!(l.len(), 1);
    /// ```
    ///
    pub fn add(&mut self, name: Name, age: Age, grade: Grade) -> Result<&Student, &str> {
        if self.0.len() >= self.0.capacity() {
            return Err("Limite de usuários excedido");
        }
        match self.get_by_name(name) {
            Some(_) => Err("Nome já existe na coleção"),
            None => {
                self.0.push(Student { name, age, grade });
                Ok(self.0.last().unwrap())
            }
        }
    }

    /// Returns an Option<Student, None> searching for name.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_name(Name::try_from_str("Student Name"));
    /// ```
    pub fn get_by_name(&self, name: Name) -> Option<&Student> {
        self.0.iter().find(|&i| i.name == name)
    }

    /// Returns a Option<Student, None> searching for age.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_age(25);
    /// ```
    pub fn get_by_age(&self, age: Age) -> Option<&Student> {
        self.0.iter().find(|&i| i.age == age)
    }

    /// Returns an Option<Student, None> searching for grade.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_age(10);
    /// ```
    pub fn get_by_grade(&self, grade: Grade) -> Option<&Student> {
        self.0.iter().find(|&i| i.grade == grade)
    }

    /// Removes a Student from StudentList by name and returns Ok() if found.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.remove(Name::try_from_str("Student Name"));
    /// assert_eq!(l.len(), 0);
    /// ```
    pub fn remove_by_name(&mut self, student: Name) -> Result<&str, &str> {
        if let Some(index) = self.0.iter().position(|n| n.name == student) {
            self.0.swap_remove(index);
            return Ok("");
        }
        Err("Aluno não encontrado")
    }

    /// Removes a Student from StudentList by age and returns Ok() if found.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.remove(25);
    /// assert_eq!(l.len(), 0);
    /// ```
    pub fn remove_by_age(&mut self, student: Age) -> Result<&str, &str> {
        if let Some(index) = self.0.iter().position(|n| n.age == student) {
            self.0.swap_remove(index);
            return Ok("");
        }
        Err("Aluno não encontrado")
    }

    /// Removes a Student from StudentList by grade and returns Ok() if found.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.remove(10);
    /// assert_eq!(l.len(), 0);
    /// ```
    pub fn remove_by_grade(&mut self, student: Grade) -> Result<&str, &str> {
        if let Some(index) = self.0.iter().position(|n| n.grade == student) {
            self.0.swap_remove(index);
            return Ok("");
        }
        Err("Aluno não encontrado")
    }


    /// Returns the total number of Student the StudentList can hold.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// assert_eq!(l.capacity, 8);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of Students currently in StudentList, also referred to as its 'length'.
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert_eq!(l.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Clears the StudentList.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.clear();
    /// assert!(l.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns true if the StudentList contains no Students.
    /// Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// assert!(l.is_empty());
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert!(!l.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

