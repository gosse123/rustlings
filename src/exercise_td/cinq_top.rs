use crate::utils::lecture_clavier::lire_nombre;

pub fn cinq_stop() {
    let mut six_stop = 0;

    loop {
        let nombre = lire_nombre();

        if six_stop == 5 {
            break;
        } else {
            if nombre == 6 {
                six_stop += 1;
                continue;
            }
        }
    }
}
