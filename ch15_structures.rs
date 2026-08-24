struct Student {
    name:String,
    major:String,
    batch:u32
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn get_instance(x: i32, y: i32) -> Point {
        Point {x: x, y: y}
    }

    fn display(&self) {
        println!("x = {}, y = {}", self.x, self.y);
    }
}

fn main() {
    let student1 = Student {
        name:String::from("Jonathan"),
        major:String::from("Computer Science"),
        batch:2016
    };

    let mut student2 = Student {
        name:String::from("Rizky"),
        major:String::from("Computer Science"),
        batch:2014
    };

    println!("Name: {}, Major: {}, Batch: {}", student1.name, student1.major, student1.batch);

    student2.major = String::from("Information system");
    println!("Name: {}, Major: {}, Batch: {}", student2.name, student2.major, student2.batch);

    display(student1);
    display(student2);

    let student1 = Student {
        name:String::from("Jonathan"),
        major:String::from("Computer Science"),
        batch:2016
    };

    let student2 = Student {
        name:String::from("Rizky"),
        major:String::from("Computer Science"),
        batch:2014
    };

    let senior = who_is_senior(student1, student2);
    println!("Senior!");
    display(senior);

    let small = Rectangle {
        width: 10,
        height: 20
    };

    println!("width is {}, height is {}, area of rectangle is {}", small.width, small.height, small.area());

    let p1 = Point::get_instance(10, 20);
    p1.display();
}

fn display(student: Student) {
    println!("Name: {}, Major: {}, Batch: {}", student.name, student.major, student.batch);
}

fn who_is_senior(student1: Student, student2: Student) -> Student {
    if student1.batch < student2.batch {
        return student1
    }
    else {
        return student2
    }
}