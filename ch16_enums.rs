#[derive(Debug)] // biar gak error saat print enum
enum GenderCategory {
    Male,
    Female
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: GenderCategory
}

// Option is a predefined enum in the Rust standard library. This enum has two values − Some(data) and None.
fn is_even(no:i32)->Option<bool> {
    if no % 2 == 0 {
        Some(true)
    }
    else {
        None
    }
}

enum CarType {
    Hatchback,
    Sedan,
    SUV
}

fn print_size(car: CarType) {
    match car {
        CarType::Hatchback => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("Medium sized car");
        },
        CarType::SUV => {
            println!("Large sized car");
        }
    }
}

#[derive(Debug)]
enum AnotherGenderCategory {
    Name(String),
    Usr_ID(i32)
}

fn main() {
   let male = GenderCategory::Male;
   let female = GenderCategory::Female; 

   println!("{:?}", male);
   println!("{:?}", female);

   let p1 = Person{
    name:String::from("Jonathan"),
    gender:GenderCategory::Male
   };

   let p2 = Person{
    name:String::from("Wati"),
    gender:GenderCategory::Female
   };

   println!("{:?}", p1);
   println!("{:?}", p2);

   let result = is_even(3);
   println!("{:?}", result);
   println!("{:?}", is_even(30));

   print_size(CarType::Hatchback);
   print_size(CarType::Sedan);
   print_size(CarType::SUV);

   match is_even(5) {
    Some(data) => {
        if data == true {
            println!("Even no");
        }
    },
    None => {
        println!("Not even")
    }
   }

   let p1 = AnotherGenderCategory::Name(String::from("Jonathan"));
   let p2 = AnotherGenderCategory::Usr_ID(100);

   println!("{:?}", p1);
   println!("{:?}", p2);

   match p1 {
    AnotherGenderCategory::Name(val) => {
        println!("{}", val);
    }
    AnotherGenderCategory::Usr_ID(val) => {
        println!("{}", val);
    }
   }
}