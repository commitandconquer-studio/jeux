use super::*;

fn card(piece: Piece, rot: u8) -> Card {
    Card { piece, rot }
}

fn at(row: usize, col: usize) -> usize {
    row * N + col
}

#[test]
fn le_paquet_a_les_bonnes_quantites() {
    let cards = deck(1);
    assert_eq!(cards.len(), DECK_SIZE);
    for piece in Piece::ALL {
        assert_eq!(cards.iter().filter(|&&p| p == piece).count(), PER_PIECE);
    }
}

#[test]
fn le_meme_numero_donne_le_meme_paquet() {
    assert_eq!(deck(42), deck(42));
    assert_ne!(deck(1), deck(2));
    assert_ne!(deck(20261024), deck(20261025));
}

#[test]
fn le_paquet_ne_change_jamais_d_une_version_a_l_autre() {
    // Si ce test casse, les numéros de partie des joueurs ne donneraient
    // plus les mêmes cartes : il ne faut pas changer le tirage.
    let order: Vec<usize> = deck(7).iter().map(|&p| Piece::ALL.iter().position(|&q| q == p).unwrap()).collect();
    assert_eq!(
        order,
        [3, 4, 3, 0, 0, 0, 4, 2, 4, 0, 3, 2, 0, 1, 1, 0, 1, 3, 1, 4, 1, 4, 2, 4, 2, 1, 1, 0, 3, 2, 3, 3, 0, 1, 2, 4, 2, 4, 2, 3]
    );
    assert_eq!(day_number(2026, 10, 24), 20261024);
}

#[test]
fn deux_numeros_voisins_donnent_deux_paquets_differents() {
    for number in 0..200 {
        assert_ne!(deck(number), deck(number + 1), "numéros {number} et {}", number + 1);
    }
}

#[test]
fn les_cotes_tournent_dans_le_sens_des_aiguilles_d_une_montre() {
    let courbe = Card::new(Piece::RailCourbe);
    assert_eq!(courbe.sides(Network::Rail), NORTH | EAST);
    assert_eq!(courbe.rotated().sides(Network::Rail), EAST | SOUTH);
    assert_eq!(courbe.rotated().rotated().sides(Network::Rail), SOUTH | WEST);
    assert_eq!(courbe.rotated().rotated().rotated().sides(Network::Rail), WEST | NORTH);
    assert_eq!(courbe.sides(Network::Route), 0);
    let croix = Card::new(Piece::Croisement);
    assert_eq!(croix.sides(Network::Rail), NORTH | SOUTH);
    assert_eq!(croix.sides(Network::Route), EAST | WEST);
    assert_eq!(croix.rotated().sides(Network::Rail), EAST | WEST);
    assert_eq!(croix.rotated().sides(Network::Route), NORTH | SOUTH);
}

#[test]
fn deux_rails_droits_superposes() {
    let mut board = Board::default();
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.longest_route, s.open_ends), (1, 2, 0, 2));
    assert_eq!(s.total(), 1);
}

#[test]
fn une_boucle_de_quatre_courbes_ne_perd_rien() {
    let mut board = Board::default();
    board.place(at(0, 0), card(Piece::RailCourbe, 1)); // est, sud
    board.place(at(0, 1), card(Piece::RailCourbe, 2)); // sud, ouest
    board.place(at(1, 1), card(Piece::RailCourbe, 3)); // ouest, nord
    board.place(at(1, 0), card(Piece::RailCourbe, 0)); // nord, est
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.open_ends), (4, 4, 0));
    assert_eq!(s.total(), 8);
}

#[test]
fn un_passage_a_niveau_entierement_raccorde() {
    let mut board = Board::default();
    board.place(at(3, 3), Card::new(Piece::Croisement)); // rail nord-sud, route est-ouest
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(4, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 2), card(Piece::RouteDroite, 1));
    board.place(at(3, 4), card(Piece::RouteDroite, 1));
    let s = board.score();
    assert_eq!(s.crossings, 1);
    assert_eq!((s.links, s.longest_rail, s.longest_route), (4, 3, 3));
    // Les quatre bras finissent à l'air libre : un côté ouvert chacun.
    assert_eq!(s.open_ends, 4);
    assert_eq!(s.total(), 4 + 3 + 3 + CROSSING_BONUS - 4);
}

#[test]
fn deux_voies_differentes_ne_se_raccordent_pas() {
    let mut board = Board::default();
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    board.place(at(2, 3), card(Piece::RouteDroite, 0));
    let s = board.score();
    assert_eq!(s.links, 0);
    assert_eq!(s.open_ends, 4);
}

#[test]
fn le_score_ne_depend_pas_de_l_ordre_de_pose() {
    let pieces = [
        (at(1, 1), card(Piece::RailCourbe, 1)),
        (at(1, 2), card(Piece::RailDroit, 1)),
        (at(2, 1), card(Piece::RouteCourbe, 0)),
        (at(3, 3), card(Piece::Croisement, 1)),
    ];
    let mut a = Board::default();
    let mut b = Board::default();
    for &(cell, c) in &pieces {
        a.place(cell, c);
    }
    for &(cell, c) in pieces.iter().rev() {
        b.place(cell, c);
    }
    assert_eq!(a.score(), b.score());
}

#[test]
fn une_case_prise_refuse_une_seconde_carte() {
    let mut board = Board::default();
    assert!(board.place(0, Card::new(Piece::RailDroit)));
    assert!(!board.place(0, Card::new(Piece::RouteDroite)));
}

#[test]
fn une_partie_se_joue_jusqu_a_la_quarantieme_carte() {
    let mut game = Game::new(5);
    assert_eq!(game.upcoming().len(), 2);
    assert_eq!(game.current(), Some(game.deck[0]));
    for cell in 0..DECK_SIZE {
        assert!(!game.is_finished());
        assert!(game.play(cell, (cell % 4) as u8));
    }
    assert!(game.is_finished());
    assert_eq!(game.current(), None);
    assert!(game.upcoming().is_empty());
    assert!(!game.play(45, 0), "plus de carte à poser");
}

#[test]
fn on_peut_annuler_la_derniere_carte() {
    let mut game = Game::new(5);
    let before = game.score();
    game.play(10, 0);
    assert_eq!(game.played(), 1);
    assert!(game.undo());
    assert_eq!(game.played(), 0);
    assert_eq!(game.score(), before);
    assert_eq!(game.board, Board::default());
    assert!(!game.undo());
}

#[test]
fn on_ne_peut_pas_jouer_sur_une_case_prise() {
    let mut game = Game::new(5);
    assert!(game.play(0, 0));
    assert!(!game.play(0, 1));
    assert_eq!(game.played(), 1);
}

/// Joue chaque carte au meilleur endroit du moment (glouton), pour avoir un
/// repère du score atteignable. Sert à régler le barème et les étoiles.
fn greedy(number: u64) -> i32 {
    let mut game = Game::new(number);
    while !game.is_finished() {
        let mut best = (i32::MIN, 0, 0);
        let free: Vec<usize> = (0..N * N).filter(|&c| game.board.cells[c].is_none()).collect();
        for cell in free {
            for rot in 0..4 {
                game.play(cell, rot);
                let total = game.score().total();
                game.undo();
                if total > best.0 {
                    best = (total, cell, rot);
                }
            }
        }
        game.play(best.1, best.2);
    }
    game.score().total()
}

#[test]
#[ignore = "mesure d'équilibrage : cargo test -p passage-core --release -- --ignored --nocapture"]
fn equilibrage() {
    let mut totals: Vec<i32> = (1..=200).map(greedy).collect();
    totals.sort();
    let mean = totals.iter().sum::<i32>() as f64 / totals.len() as f64;
    println!(
        "glouton sur 200 numéros : min {} · 25 % {} · médiane {} · 75 % {} · max {} · moyenne {mean:.1}",
        totals[0], totals[50], totals[100], totals[150], totals[199]
    );
}

