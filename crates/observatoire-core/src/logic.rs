//! Le solveur logique : il raisonne comme un joueur, sans deviner. Sert aux
//! indices du jeu et à mesurer la difficulté des grilles.
use crate::{LUNE, SOLEIL, Signe, Signes, VIDE, erreurs, lignes, oppose, possible};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Technique {
    /// Un « = » ou un « × » relie la case à une case déjà remplie.
    Signe,
    /// Deux identiques côte à côte : leurs voisins sont l'autre symbole.
    Paire,
    /// Deux identiques séparés d'une case : celle du milieu est l'autre.
    Trou,
    /// La ligne a déjà sa moitié d'un symbole : le reste est l'autre.
    Compte,
    /// L'autre symbole mènerait à une impasse un peu plus loin.
    Absurde,
}

impl Technique {
    pub fn cout(self) -> u32 {
        match self {
            Technique::Signe => 1,
            Technique::Paire => 2,
            Technique::Trou => 2,
            Technique::Compte => 3,
            Technique::Absurde => 8,
        }
    }

    /// L'explication de la technique, dans la langue `langue` (0 français,
    /// 1 anglais, 2 allemand, 3 italien, 4 espagnol).
    pub fn explain(self, langue: usize) -> &'static str {
        let t: [&str; 5] = match self {
            Technique::Signe => [
                "Un signe relie cette case à une case déjà remplie : « = » veut dire pareille, « × » différente.",
                "A sign links this cell to one already filled: “=” means the same, “×” means different.",
                "Ein Zeichen verbindet dieses Feld mit einem gefüllten: „=“ heißt gleich, „ד heißt verschieden.",
                "Un segno collega questa casella a una già piena: «=» vuol dire uguale, «×» diversa.",
                "Un signo une esta casilla a otra ya llena: «=» significa igual, «×» distinta.",
            ],
            Technique::Paire => [
                "Deux symboles identiques côte à côte : leurs voisins sont forcément l'autre symbole.",
                "Two identical symbols side by side: their neighbours must be the other symbol.",
                "Zwei gleiche Symbole nebeneinander: Ihre Nachbarn sind zwingend das andere Symbol.",
                "Due simboli uguali vicini: i loro vicini sono per forza l'altro simbolo.",
                "Dos símbolos iguales seguidos: sus vecinos son por fuerza el otro símbolo.",
            ],
            Technique::Trou => [
                "Deux symboles identiques séparés par une case : celle du milieu est l'autre symbole.",
                "Two identical symbols with one cell between them: the middle one is the other symbol.",
                "Zwei gleiche Symbole mit einem Feld dazwischen: Das mittlere ist das andere Symbol.",
                "Due simboli uguali separati da una casella: quella in mezzo è l'altro simbolo.",
                "Dos símbolos iguales separados por una casilla: la del medio es el otro símbolo.",
            ],
            Technique::Compte => [
                "Cette ligne a déjà sa moitié de ce symbole : les cases restantes ont l'autre.",
                "This line already has its half of that symbol: the remaining cells get the other.",
                "Diese Reihe hat schon ihre Hälfte dieses Symbols: Die übrigen Felder bekommen das andere.",
                "Questa linea ha già la sua metà di quel simbolo: le caselle rimaste hanno l'altro.",
                "Esta línea ya tiene su mitad de ese símbolo: las casillas restantes llevan el otro.",
            ],
            Technique::Absurde => [
                "Si l'on mettait l'autre symbole ici, la grille deviendrait impossible un peu plus loin.",
                "If you put the other symbol here, the grid would become impossible a little further on.",
                "Mit dem anderen Symbol hier würde das Gitter ein Stück weiter unmöglich.",
                "Se qui mettessi l'altro simbolo, la griglia diventerebbe impossibile poco più in là.",
                "Si pusieras aquí el otro símbolo, la cuadrícula sería imposible un poco más adelante.",
            ],
        };
        t[langue.min(4)]
    }
}

/// Une déduction : la case, sa valeur, et les cases qui l'expliquent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deduction {
    pub technique: Technique,
    pub case: usize,
    pub valeur: u8,
    pub raison: Vec<usize>,
}

/// Les déductions simples (sans raisonnement par l'absurde), la moins chère d'abord.
fn simple(n: usize, signes: &Signes, cases: &[u8]) -> Option<Deduction> {
    // Signe : une case vide reliée à une case remplie.
    for (a, b, s) in signes.tous() {
        for (x, y) in [(a, b), (b, a)] {
            if cases[x] != VIDE && cases[y] == VIDE {
                let valeur = if s == Signe::Egal { cases[x] } else { oppose(cases[x]) };
                return Some(Deduction { technique: Technique::Signe, case: y, valeur, raison: vec![x] });
            }
        }
    }
    let lignes = lignes(n);
    // Paire : deux identiques côte à côte.
    for l in &lignes {
        for k in 0..n - 1 {
            let (a, b) = (l[k], l[k + 1]);
            if cases[a] != VIDE && cases[a] == cases[b] {
                for voisin in [k.checked_sub(1), (k + 2 < n).then_some(k + 2)].into_iter().flatten() {
                    if cases[l[voisin]] == VIDE {
                        return Some(Deduction { technique: Technique::Paire, case: l[voisin], valeur: oppose(cases[a]), raison: vec![a, b] });
                    }
                }
            }
        }
    }
    // Trou : deux identiques séparés d'une case vide.
    for l in &lignes {
        for k in 0..n - 2 {
            let (a, m, b) = (l[k], l[k + 1], l[k + 2]);
            if cases[a] != VIDE && cases[a] == cases[b] && cases[m] == VIDE {
                return Some(Deduction { technique: Technique::Trou, case: m, valeur: oppose(cases[a]), raison: vec![a, b] });
            }
        }
    }
    // Compte : la ligne a déjà sa moitié d'un symbole.
    for l in &lignes {
        for v in [SOLEIL, LUNE] {
            if l.iter().filter(|&&i| cases[i] == v).count() == n / 2
                && let Some(&vide) = l.iter().find(|&&i| cases[i] == VIDE)
            {
                let raison = l.iter().copied().filter(|&i| cases[i] == v).collect();
                return Some(Deduction { technique: Technique::Compte, case: vide, valeur: oppose(v), raison });
            }
        }
    }
    None
}

/// La grille devient-elle impossible ? (règle enfreinte, ou case sans valeur possible)
fn impasse(n: usize, signes: &Signes, cases: &[u8]) -> bool {
    !erreurs(n, signes, cases).is_empty()
        || (0..n * n).any(|i| cases[i] == VIDE && !possible(n, signes, cases, i, SOLEIL) && !possible(n, signes, cases, i, LUNE))
}

/// La prochaine déduction logique, s'il y en a une.
pub fn next_deduction(n: usize, signes: &Signes, cases: &[u8]) -> Option<Deduction> {
    if let Some(d) = simple(n, signes, cases) {
        return Some(d);
    }
    // Par l'absurde : essayer un symbole, dérouler les déductions simples ;
    // si l'on tombe sur une impasse, c'est l'autre symbole.
    for i in 0..n * n {
        if cases[i] != VIDE {
            continue;
        }
        for v in [SOLEIL, LUNE] {
            let mut essai = cases.to_vec();
            essai[i] = v;
            loop {
                if impasse(n, signes, &essai) {
                    return Some(Deduction { technique: Technique::Absurde, case: i, valeur: oppose(v), raison: vec![] });
                }
                match simple(n, signes, &essai) {
                    Some(d) => essai[d.case] = d.valeur,
                    None => break,
                }
            }
        }
    }
    None
}

/// Résout par la logique seule ; renvoie la difficulté (somme des coûts des
/// techniques employées), ou `None` si la logique ne suffit pas.
pub fn logic_solve(n: usize, signes: &Signes, donnees: &[u8]) -> Option<u32> {
    let mut cases = donnees.to_vec();
    let mut cout = 0;
    while cases.contains(&VIDE) {
        let d = next_deduction(n, signes, &cases)?;
        cases[d.case] = d.valeur;
        cout += d.technique.cout();
    }
    erreurs(n, signes, &cases).is_empty().then_some(cout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_techniques_simples() {
        let n = 4;
        let s = Signes::vides(n);
        // S S . .  → la troisième case est une lune (paire).
        let mut c = vec![VIDE; 16];
        c[0] = SOLEIL;
        c[1] = SOLEIL;
        let d = next_deduction(n, &s, &c).unwrap();
        assert_eq!((d.technique, d.case, d.valeur), (Technique::Paire, 2, LUNE));
        // S . S .  → la case du milieu est une lune (trou).
        let mut c = vec![VIDE; 16];
        c[0] = SOLEIL;
        c[2] = SOLEIL;
        let d = next_deduction(n, &s, &c).unwrap();
        assert_eq!((d.technique, d.case, d.valeur), (Technique::Trou, 1, LUNE));
    }

    #[test]
    fn chaque_technique_parle_cinq_langues() {
        for t in [Technique::Signe, Technique::Paire, Technique::Trou, Technique::Compte, Technique::Absurde] {
            for l in 0..5 {
                assert!(!t.explain(l).is_empty());
            }
        }
    }
}
