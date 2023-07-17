impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut strength = vec![];

        // Genero un vector con la cant de soldados de cada fila.
        for row in mat {
            strength.push(row.iter().fold(0, |acc, e| acc + e));
        }

        // Convierto ese vector en un vector de (fuerza, indice).
        let mut strength = strength.iter().enumerate().collect::<Vec<_>>();

        // Ordeno según fuerza.
        strength.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Descarto la fuerza, mantengo los índices.
        let strength: Vec<i32> = strength.iter().map(|e| e.0 as i32).collect();

        // Entrego un vector con los k primeros índices.
        strength[0..k as usize].to_vec()
    }
}

fn main() {
    todo!();
}
