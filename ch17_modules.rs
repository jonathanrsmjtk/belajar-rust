/*  
Sebuah logical group disebut module, dan banyak module dikompilasi menjadi sebuah unit bernama crate. 
Binary crate: executable project yang punya main()
Library crate: komponen yang bisa digunakan oleh crate lain, tidak punya main()
3rd party crates bisa diambil dari https://crates.io/ dan ditambahkan ke project dengan menambahkan dependencies di Cargo.toml
*/

pub mod movies {
    pub fn play(name: String) {
        println!("Playing movie {}", name);
    }
}

pub mod eng_movies {
    pub mod english {
        pub mod comedy {
            pub fn play(name: String) {
                println!("Playing comedy movie {}", name);
            }
        }
        pub mod thriller {
            pub fn play(name: String) {
                println!("Playing thriller movie {}", name);
            }
        }
    }
}

use movies::play;
use eng_movies::english::comedy;

fn main() {
    movies::play("Cinta fitri".to_string());
    play("Ganteng Ganteng Serigala".to_string());
    comedy::play("Rush Hour 3".to_string());
}