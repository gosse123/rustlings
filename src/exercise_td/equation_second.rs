use crate::utils::lecture_clavier::lire_nombre;

pub fn secod() {
    let a = lire_nombre();
    let b = lire_nombre();
    let c = lire_nombre();

    if a == 0 {
        if b == 0 {
            println!("se n'est pas une equation du second degre");
        } else {
            println!("la solution est {}", -(c / b));
        }
    } else {
        let delta = (b * b) - (4 * a * c);
        if delta < 0 {
            println!("pas de solution");
        } else if delta > 0 {
            println!(
                "x1 = {} , x2 = {}",
                ((b - delta) / 2 * a),
                ((b - delta) / 2 * a)
            );
        } else {
            println!("x = {}", (b / 2 * a));
        }
    }
}
