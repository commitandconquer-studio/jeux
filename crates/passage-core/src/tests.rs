use super::*;

/// Grille 6 × 6 faite à la main : un rail horizontal (rangée 2) et une route
/// verticale (colonne 2) qui se croisent au passage à niveau (2, 2).
const MAIN: &str = "6 203.451 020.522 \
    ..o...\
    ..o...\
    rrxrr.\
    ..o.r.\
    ..o.rr\
    ..o...";

fn hand_made() -> Puzzle {
    Puzzle::from_line(&format!("{MAIN} {} 0", "0".repeat(36))).expect("la grille faite main est valide")
}

fn cells_of(p: &Puzzle) -> Vec<Kind> {
    p.solution.clone()
}

#[test]
fn les_chiffres_des_bords_sont_deduits() {
    let p = hand_made();
    assert_eq!(p.row_counts[Network::Rail as usize], vec![0, 0, 5, 1, 2, 0]);
    assert_eq!(p.col_counts[Network::Rail as usize], vec![1, 1, 1, 1, 3, 1]);
    assert_eq!(p.row_counts[Network::Route as usize], vec![1; 6]);
    assert_eq!(p.col_counts[Network::Route as usize], vec![0, 0, 6, 0, 0, 0]);
}

#[test]
fn la_ligne_de_texte_fait_l_aller_retour() {
    let p = hand_made();
    assert_eq!(Puzzle::from_line(&p.to_line()), Some(p));
}

#[test]
fn la_solution_passe_la_verification() {
    let p = hand_made();
    assert_eq!(p.check(&cells_of(&p)), Ok(()));
    assert!(p.is_solved_by(&cells_of(&p)));
}

#[test]
fn une_voie_qui_se_touche_est_refusee() {
    let p = hand_made();
    let mut cells = cells_of(&p);
    cells[3 * 6 + 3] = Kind::Rail; // touche (2,3), (3,4) et (4,3)... sans être sur la voie
    assert!(matches!(p.check(&cells), Err(Fault::Degree { .. })));
}

#[test]
fn une_voie_coupee_est_refusee() {
    let p = hand_made();
    let mut cells = cells_of(&p);
    cells[3 * 6 + 4] = Kind::Vide; // le rail est interrompu
    assert!(p.check(&cells).is_err());
}

#[test]
fn un_croisement_mal_oriente_est_refuse() {
    // Croisement sur la route, mais le rail y arrive aussi par le nord.
    let mut cells = hand_made().solution;
    cells[6 + 2] = Kind::Croisement; // (1, 2) devient croisement sans raison
    assert!(check_structure(6, &hand_made().gates, &cells).is_err());
}

#[test]
fn une_gare_hors_du_bord_est_refusee() {
    let mut gates = hand_made().gates;
    gates[0][0] = Gate { row: 2, col: 1, side: 3 };
    assert_eq!(check_structure(6, &gates, &hand_made().solution), Err(Fault::Gate(Network::Rail)));
}

#[test]
fn des_chiffres_faux_sont_refuses() {
    let mut p = hand_made();
    p.row_counts[0][0] = 3;
    assert_eq!(p.check(&p.solution.clone()), Err(Fault::Count));
}

#[test]
fn la_recherche_trouve_toujours_la_solution() {
    let p = hand_made();
    let all = solutions(&p, 50);
    assert!(!all.is_empty());
    assert!(all.contains(&p.solution));
    for cells in &all {
        assert_eq!(p.check(cells), Ok(()));
    }
}

#[test]
fn une_grille_entierement_devoilee_est_unique_et_logique() {
    let mut p = hand_made();
    p.givens = vec![true; 36];
    assert_eq!(solutions(&p, 5).len(), 1);
    assert!(logic_solve(&p).is_some());
}

#[test]
fn la_grille_faite_main_a_une_seule_solution_trouvable_par_la_logique() {
    let p = hand_made();
    assert_eq!(solutions(&p, 5), vec![p.solution.clone()]);
    assert!(logic_solve(&p).is_some());
}

#[test]
fn un_indice_vient_d_une_case_deductible() {
    let mut p = hand_made();
    p.givens = vec![true; 36];
    p.givens[0] = false;
    let known = vec![None; 36];
    let hint = next_deduction(&p, &known).expect("une case est déductible");
    assert!(hint.removed != 0);
}

