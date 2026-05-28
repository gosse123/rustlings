use std::io;

pub fn lire_nombre() -> i32 {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Erreur de lecture");

    let a: i32 = a.trim().parse().expect("se n'est pas un nombre");
    a
}
