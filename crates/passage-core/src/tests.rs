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
    assert_eq!(Piece::DECK.iter().map(|&(_, n)| n).sum::<usize>(), DECK_SIZE);
    for (piece, n) in Piece::DECK {
        assert_eq!(cards.iter().filter(|&&p| p == piece).count(), n, "{piece:?}");
    }
    assert!(!cards.contains(&Piece::GareDepart) && !cards.contains(&Piece::Parking));
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
    let ordre: Vec<usize> = deck(7).iter().take(12).map(|&p| Piece::DECK.iter().position(|&(q, _)| q == p).unwrap()).collect();
    assert_eq!(ordre, PAQUET_7_DEBUT);
    assert_eq!(day_number(2026, 10, 24), 20261024);
}

const PAQUET_7_DEBUT: [usize; 12] = [11, 13, 9, 0, 0, 1, 14, 6, 12, 0, 8, 5];

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
    let t = Card::new(Piece::RailT);
    assert_eq!(t.sides(Network::Rail), NORTH | EAST | WEST);
    assert_eq!(t.rotated().sides(Network::Rail), EAST | SOUTH | NORTH);
    assert_eq!(Card::new(Piece::RouteCroix).sides(Network::Route), 15);
}

#[test]
fn les_pieces_ont_le_bon_nombre_d_entrees() {
    let entrees = |p: Piece| {
        let c = Card::new(p);
        c.sides(Network::Rail).count_ones() + c.sides(Network::Route).count_ones()
    };
    for p in [Piece::RailDroit, Piece::RailCourbe, Piece::RouteDroite, Piece::RouteCourbe, Piece::Gare2Droite, Piece::Gare2Courbe] {
        assert_eq!(entrees(p), 2, "{p:?}");
    }
    for p in [Piece::RailT, Piece::RouteT, Piece::Gare3Rail, Piece::Gare3Route] {
        assert_eq!(entrees(p), 3, "{p:?}");
    }
    for p in [Piece::RailCroix, Piece::RouteCroix, Piece::Croisement, Piece::Gare4] {
        assert_eq!(entrees(p), 4, "{p:?}");
    }
    for p in [Piece::Impasse, Piece::RondPoint, Piece::GareDepart, Piece::Parking] {
        assert_eq!(entrees(p), 1, "{p:?}");
    }
    // Les gares routières sont les seules à relier les deux voies.
    for (p, _) in Piece::DECK {
        let station = matches!(p, Piece::Gare2Droite | Piece::Gare2Courbe | Piece::Gare3Rail | Piece::Gare3Route | Piece::Gare4);
        assert_eq!(Card::new(p).joined(), station, "{p:?}");
    }
}

#[test]
fn les_gares_routieres_ont_du_rail_et_de_la_route() {
    for p in [Piece::Gare2Droite, Piece::Gare2Courbe, Piece::Gare3Rail, Piece::Gare3Route, Piece::Gare4] {
        let c = Card::new(p);
        assert!(c.sides(Network::Rail) != 0 && c.sides(Network::Route) != 0, "{p:?}");
        assert_eq!(c.sides(Network::Rail) & c.sides(Network::Route), 0, "{p:?}");
    }
}

#[test]
fn la_grille_de_depart_a_ses_quatre_pieces_imposees() {
    let board = Board::default();
    assert_eq!(board.cells.iter().flatten().count(), 4);
    for (cell, piece, rot) in STARTS {
        assert_eq!(board.cells[cell], Some(Card { piece, rot }));
        assert!(Board::is_start(cell));
        // L'entrée est sur un bord de la grille et regarde vers l'intérieur.
        let c = board.cells[cell].unwrap();
        let cote = (0..4).find(|&s| (c.sides(Network::Rail) | c.sides(Network::Route)) & (1 << s) != 0).unwrap();
        assert!(neighbor(cell, cote).is_some(), "l'entrée de la case {cell} regarde hors de la grille");
        assert!((0..4).any(|s| neighbor(cell, s).is_none()), "la case {cell} n'est pas sur un bord");
    }
    assert_eq!(STARTS.iter().filter(|s| s.1 == Piece::GareDepart).count(), 2);
    assert_eq!(STARTS.iter().filter(|s| s.1 == Piece::Parking).count(), 2);
}

#[test]
fn on_ne_peut_pas_reprendre_ni_couvrir_une_piece_de_depart() {
    let mut board = Board::default();
    assert!(!board.place(STARTS[0].0, Card::new(Piece::RailDroit)));
    assert_eq!(board.remove(STARTS[0].0), None);
    assert!(board.cells[STARTS[0].0].is_some());
}

#[test]
fn deux_rails_droits_superposes() {
    let mut board = Board::empty();
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.longest_route, s.open_ends), (1, 2, 0, 2));
    assert_eq!(s.total(), 1);
}

#[test]
fn une_boucle_de_quatre_courbes_ne_perd_rien() {
    let mut board = Board::empty();
    board.place(at(0, 0), card(Piece::RailCourbe, 1));
    board.place(at(0, 1), card(Piece::RailCourbe, 2));
    board.place(at(1, 1), card(Piece::RailCourbe, 3));
    board.place(at(1, 0), card(Piece::RailCourbe, 0));
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.open_ends), (4, 4, 0));
    assert_eq!(s.total(), 8);
}

#[test]
fn un_cul_de_sac_termine_proprement_une_voie() {
    // Un rail droit fermé à chaque bout par une impasse : plus aucun bout dans le vide.
    let mut board = Board::empty();
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    board.place(at(2, 3), card(Piece::Impasse, 2)); // entrée au sud, vers le rail
    board.place(at(4, 3), card(Piece::Impasse, 0)); // entrée au nord, vers le rail
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.open_ends), (2, 3, 0));
    assert_eq!(s.total(), 5);
}

#[test]
fn un_rond_point_termine_une_route() {
    let mut board = Board::empty();
    board.place(at(3, 3), card(Piece::RouteDroite, 1));
    board.place(at(3, 2), card(Piece::RondPoint, 1)); // entrée à l'est
    board.place(at(3, 4), card(Piece::RondPoint, 3)); // entrée à l'ouest
    let s = board.score();
    assert_eq!((s.links, s.longest_route, s.open_ends), (2, 3, 0));
}

#[test]
fn un_cul_de_sac_seul_perd_un_point() {
    let mut board = Board::empty();
    board.place(at(3, 3), Card::new(Piece::Impasse));
    assert_eq!(board.score().open_ends, 1);
}

#[test]
fn un_rail_en_t_a_trois_branches() {
    let mut board = Board::empty();
    board.place(at(3, 3), Card::new(Piece::RailT)); // nord, est, ouest
    board.place(at(2, 3), card(Piece::Impasse, 2));
    board.place(at(3, 4), card(Piece::Impasse, 3));
    board.place(at(3, 2), card(Piece::Impasse, 1));
    let s = board.score();
    assert_eq!((s.links, s.longest_rail, s.open_ends), (3, 4, 0));
}

#[test]
fn un_passage_a_niveau_entierement_raccorde() {
    let mut board = Board::empty();
    board.place(at(3, 3), Card::new(Piece::Croisement));
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(4, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 2), card(Piece::RouteDroite, 1));
    board.place(at(3, 4), card(Piece::RouteDroite, 1));
    let s = board.score();
    assert_eq!(s.crossings, 1);
    assert_eq!((s.links, s.longest_rail, s.longest_route), (4, 3, 3));
    assert_eq!(s.open_ends, 4);
    assert_eq!(s.total(), 4 + 3 + 3 + CROSSING_BONUS - 4);
}

#[test]
fn le_passage_a_niveau_ne_relie_pas_le_rail_et_la_route() {
    let mut board = Board::empty();
    board.place(at(3, 3), Card::new(Piece::Croisement));
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 2), card(Piece::RouteDroite, 1));
    let s = board.score();
    // Le rail du haut est raccordé au rail du croisement, pas à la route.
    assert_eq!(s.links, 2);
    assert_eq!((s.longest_rail, s.longest_route), (2, 2));
}

#[test]
fn une_gare_routiere_relie_le_rail_et_la_route() {
    let mut board = Board::empty();
    // Gare à 2 entrées : rail au nord, route au sud.
    board.place(at(3, 3), Card::new(Piece::Gare2Droite));
    board.place(at(2, 3), card(Piece::Impasse, 2));
    board.place(at(4, 3), card(Piece::RondPoint, 0));
    let s = board.score();
    assert_eq!(s.stations, 1);
    assert_eq!((s.links, s.longest_rail, s.longest_route, s.open_ends), (2, 2, 2, 0));
    assert_eq!(s.total(), 2 + 2 + 2 + STATION_BONUS);
}

#[test]
fn une_gare_routiere_inachevee_ne_rapporte_pas_son_bonus() {
    let mut board = Board::empty();
    board.place(at(3, 3), Card::new(Piece::Gare2Droite));
    board.place(at(2, 3), card(Piece::Impasse, 2));
    let s = board.score();
    assert_eq!(s.stations, 0);
    assert_eq!(s.open_ends, 1);
}

#[test]
fn les_gares_de_depart_reliees_par_le_rail_rapportent() {
    let mut board = Board::default();
    assert_eq!(board.score().starts, 0);
    // Gare ouest en (1, 0) vers l'est ; gare est en (5, 6) vers l'ouest.
    // Voie : (1,1)..(1,3) droite, (1,3) courbe vers le sud, descendre jusqu'à la rangée 5, puis à l'est.
    board.place(at(1, 1), card(Piece::RailDroit, 1));
    board.place(at(1, 2), card(Piece::RailDroit, 1));
    board.place(at(1, 3), card(Piece::RailCourbe, 2)); // est, sud
    board.place(at(2, 3), card(Piece::RailDroit, 0));
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    board.place(at(4, 3), card(Piece::RailDroit, 0));
    board.place(at(5, 3), card(Piece::RailCourbe, 0)); // nord, est
    board.place(at(5, 4), card(Piece::RailDroit, 1));
    board.place(at(5, 5), card(Piece::RailDroit, 1));
    assert_eq!(board.score().starts, 1);
}

#[test]
fn les_quatre_departs_dans_un_meme_reseau_rapportent() {
    // Le rail relie les deux gares, la route relie les deux parkings, et les
    // deux voies se croisent dans une gare routière à 4 entrées en (3, 3).
    let mut board = Board::default();
    for (r, c, p, rot) in [
        // Rail : gare ouest -> (1,3) -> nord-sud par la gare routière -> (5,3) -> gare est.
        (1, 1, Piece::RailDroit, 1),
        (1, 2, Piece::RailDroit, 1),
        (1, 3, Piece::RailCourbe, 2), // ouest, sud
        (2, 3, Piece::RailDroit, 0),
        (4, 3, Piece::RailDroit, 0),
        (5, 3, Piece::RailCourbe, 0), // nord, est
        (5, 4, Piece::RailDroit, 1),
        (5, 5, Piece::RailDroit, 1),
        // Route : parking nord -> (3,5) -> est-ouest par la gare routière -> (3,1) -> parking sud.
        (1, 5, Piece::RouteDroite, 0),
        (2, 5, Piece::RouteDroite, 0),
        (3, 5, Piece::RouteCourbe, 3), // ouest, nord
        (3, 4, Piece::RouteDroite, 1),
        (3, 2, Piece::RouteDroite, 1),
        (3, 1, Piece::RouteCourbe, 1), // est, sud
        (4, 1, Piece::RouteDroite, 0),
        (5, 1, Piece::RouteDroite, 0),
    ] {
        assert!(board.place(at(r, c), card(p, rot)), "case ({r}, {c}) prise");
    }
    // Avec un passage à niveau au centre, les deux voies sont complètes mais séparées.
    let mut croisement = board.clone();
    croisement.place(at(3, 3), Card::new(Piece::Croisement));
    let s = croisement.score();
    assert_eq!((s.starts, s.stations, s.crossings, s.open_ends), (2, 0, 1, 0));
    // Avec une gare routière à 4 entrées, elles ne forment plus qu'un réseau.
    board.place(at(3, 3), Card::new(Piece::Gare4)); // rail nord-sud, route est-ouest, raccordés
    let s = board.score();
    assert_eq!((s.starts, s.stations, s.crossings, s.open_ends), (3, 1, 0, 0));
}

#[test]
fn deux_voies_differentes_ne_se_raccordent_pas() {
    let mut board = Board::empty();
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
        (at(3, 3), card(Piece::Gare3Rail, 1)),
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
    let mut board = Board::empty();
    assert!(board.place(0, Card::new(Piece::RailDroit)));
    assert!(!board.place(0, Card::new(Piece::RouteDroite)));
}

/// Les cases libres au départ d'une partie.
fn libres() -> Vec<usize> {
    (0..N * N).filter(|&c| !Board::is_start(c)).collect()
}

#[test]
fn une_partie_se_joue_jusqu_a_la_quarantieme_carte() {
    let mut game = Game::new(5);
    assert_eq!(game.upcoming().len(), 2);
    assert_eq!(game.current(), Some(game.deck[0]));
    for (k, cell) in libres().into_iter().take(DECK_SIZE).enumerate() {
        assert!(!game.is_finished());
        assert!(game.play(cell, (k % 4) as u8));
    }
    assert!(game.is_finished());
    assert_eq!(game.current(), None);
    assert!(game.upcoming().is_empty());
    assert!(!game.play(libres()[DECK_SIZE], 0), "plus de carte à poser");
    assert_eq!(libres().len() - DECK_SIZE, 5);
}

#[test]
fn on_peut_annuler_la_derniere_carte() {
    let mut game = Game::new(5);
    let before = game.score();
    game.play(libres()[3], 0);
    assert_eq!(game.played(), 1);
    assert!(game.undo());
    assert_eq!(game.played(), 0);
    assert_eq!(game.score(), before);
    assert_eq!(game.board, Board::default());
    assert!(!game.undo());
}

#[test]
fn on_ne_peut_pas_jouer_sur_une_case_prise_ni_sur_un_depart() {
    let mut game = Game::new(5);
    assert!(!game.play(STARTS[0].0, 0));
    assert!(game.play(libres()[0], 0));
    assert!(!game.play(libres()[0], 1));
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


#[test]
fn un_cote_ouvert_vers_une_case_vide_est_ouvert_pas_bloque() {
    let mut board = Board::empty();
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    assert_eq!(board.side_status(at(3, 3), 0, Network::Rail), SideStatus::Open);
    assert_eq!(board.side_status(at(3, 3), 1, Network::Rail), SideStatus::Closed);
    let s = board.score();
    assert_eq!((s.open_ends, s.blocked), (2, 0));
    // Au bord de la grille aussi : ouvert, mais jamais « bloqué ».
    board.place(at(0, 0), card(Piece::RailDroit, 0));
    assert_eq!(board.side_status(at(0, 0), 0, Network::Rail), SideStatus::Open);
}

#[test]
fn un_rail_qui_arrive_sur_une_route_est_bloque() {
    let mut board = Board::empty();
    board.place(at(3, 3), card(Piece::RailDroit, 0));
    board.place(at(2, 3), card(Piece::RouteDroite, 0)); // une route au nord du rail
    assert_eq!(board.side_status(at(3, 3), 0, Network::Rail), SideStatus::Blocked);
    // Le côté sud de la route regarde le rail, mais la route n'a pas de rail : bloqué aussi.
    assert_eq!(board.side_status(at(2, 3), 2, Network::Route), SideStatus::Blocked);
    let s = board.score();
    assert_eq!((s.blocked, s.open_ends), (2, 4));
}

#[test]
fn un_cote_ouvert_contre_un_cote_ferme_est_bloque() {
    let mut board = Board::empty();
    board.place(at(3, 3), card(Piece::RailDroit, 0)); // nord-sud
    board.place(at(3, 4), card(Piece::RailDroit, 0)); // à l'est : son côté ouest est fermé
    // Le rail n'ouvre pas son côté est : rien n'arrive sur la carte voisine.
    assert_eq!(board.side_status(at(3, 3), 1, Network::Rail), SideStatus::Closed);
    // Mais un rail tourné vers elle serait bloqué.
    board.place(at(5, 5), card(Piece::RailDroit, 1)); // est-ouest
    board.place(at(5, 6), card(Piece::RailDroit, 0)); // nord-sud : son côté ouest est fermé
    assert_eq!(board.side_status(at(5, 5), 1, Network::Rail), SideStatus::Blocked);
}

#[test]
fn on_peut_arreter_la_partie_avant_la_fin_du_paquet() {
    let mut game = Game::new(5);
    game.play(libres()[0], 0);
    assert!(!game.is_finished());
    game.finish();
    assert!(game.is_finished());
    assert_eq!(game.played(), 1);
    assert_eq!(game.current(), None);
    assert!(game.upcoming().is_empty());
    assert!(!game.play(libres()[1], 0), "partie terminée");
}
