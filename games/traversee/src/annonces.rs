//! Les petits messages de Cross Express : bandeau en haut de l'écran (début, premier train, aigles…)
//! et phrases de fin de partie, différentes selon la cause de la mort.
use bevy::prelude::*;
use game_common::tx;
use game_common::ui::ThemedText;
use game_common::Rng;

use crate::GameState;
use crate::eagle::Eagle;
use crate::frog::{Cause, Frog};
use crate::lanes::{Lane, LaneKind};

/// Un message à afficher en bandeau (le précédent s'efface).
#[derive(Message, Clone, Copy)]
pub struct Annonce(pub &'static str, pub Color);

impl Annonce {
    pub fn blanc(texte: &'static str) -> Self {
        Annonce(texte, Color::WHITE)
    }
}

#[derive(Component)]
struct Bandeau {
    age: f32,
}

pub struct AnnoncesPlugin;

impl Plugin for AnnoncesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Annonce>()
            .add_systems(OnEnter(GameState::Playing), bienvenue)
            .add_systems(OnEnter(GameState::Dying), |mut commands: Commands, q: Query<Entity, With<Bandeau>>| q.iter().for_each(|e| commands.entity(e).despawn()))
            .add_systems(Update, (afficher, vieillir))
            .add_systems(Update, (premier_train, premier_aigle).run_if(in_state(GameState::Playing)));
    }
}

fn bienvenue(frog: Res<Frog>, mut annonces: MessageWriter<Annonce>) {
    if !frog.started {
        annonces.write(Annonce::blanc(
            tx!(
                "Bienvenue chez Cross Express. L'assurance vie est en option.",
                "Welcome to Cross Express. Life insurance is optional.",
                "Willkommen bei Cross Express. Die Lebensversicherung ist optional.",
                "Benvenuti in Cross Express. L'assicurazione sulla vita è facoltativa.",
                "Bienvenido a Cross Express. El seguro de vida es opcional."
            )
            .t(),
        ));
    }
}

/// Un train arrive dans le champ de vision pour la première fois de la partie.
fn premier_train(frog: Res<Frog>, lanes: Query<&Lane>, mut vu: Local<bool>, mut annonces: MessageWriter<Annonce>) {
    if !frog.started {
        *vu = false;
        return;
    }
    if *vu {
        return;
    }
    if lanes.iter().any(|l| l.kind == LaneKind::Rail && l.row >= frog.row && l.row <= frog.row + 4) {
        *vu = true;
        annonces.write(Annonce::blanc(
            tx!(
                "Attention, ce train ne fait pas de livraison.",
                "Careful, this train doesn't deliver.",
                "Achtung, dieser Zug liefert nichts aus.",
                "Attenzione, questo treno non fa consegne.",
                "Cuidado, este tren no hace entregas."
            )
            .t(),
        ));
    }
}

/// Le premier avertissement d'aigle de la partie.
fn premier_aigle(frog: Res<Frog>, eagle: Res<Eagle>, mut vu: Local<bool>, mut annonces: MessageWriter<Annonce>) {
    if !frog.started {
        *vu = false;
        return;
    }
    if !*vu && eagle.warning() {
        *vu = true;
        annonces.write(Annonce::blanc(
            tx!(
                "Nouveau problème : le ciel aussi vous déteste.",
                "New problem: the sky hates you too.",
                "Neues Problem: Der Himmel hasst Sie auch.",
                "Nuovo problema: anche il cielo ti odia.",
                "Nuevo problema: el cielo también le odia."
            )
            .t(),
        ));
    }
}

fn afficher(mut commands: Commands, mut lues: MessageReader<Annonce>, anciens: Query<Entity, With<Bandeau>>) {
    let Some(a) = lues.read().last().copied() else { return };
    for e in &anciens {
        commands.entity(e).despawn();
    }
    commands.spawn((
        Text::new(a.0),
        TextFont::from_font_size(30.0),
        TextColor(a.1),
        TextShadow { offset: Vec2::splat(3.0), color: Color::srgba(0.0, 0.0, 0.0, 0.75) },
        TextLayout::justify(Justify::Center),
        Node { position_type: PositionType::Absolute, width: percent(100), top: percent(22), padding: UiRect::horizontal(px(40)), ..default() },
        GlobalZIndex(36),
        Pickable::IGNORE,
        ThemedText,
        Bandeau { age: 0.0 },
    ));
}

fn vieillir(mut commands: Commands, time: Res<Time<Real>>, mut q: Query<(Entity, &mut Bandeau, &mut TextColor)>) {
    for (e, mut b, mut c) in &mut q {
        b.age += time.delta_secs();
        if b.age > 3.4 {
            commands.entity(e).despawn();
        } else {
            c.0 = c.0.with_alpha((3.4 - b.age).clamp(0.0, 1.0));
        }
    }
}

/// La phrase de fin de partie : une au hasard parmi celles de la cause.
pub fn mort(cause: Cause, train: bool, rng: &mut Rng) -> &'static str {
    let choix: Vec<&'static str> = match (cause, train) {
        (Cause::Squashed, true) => vec![
            tx!("Votre carrière a déraillé.", "Your career has derailed.", "Ihre Karriere ist entgleist.", "La tua carriera è deragliata.", "Su carrera descarriló.").t(),
            tx!("Ce train ne faisait pas de livraison.", "That train wasn't doing deliveries.", "Dieser Zug liefert nichts aus.", "Quel treno non faceva consegne.", "Ese tren no hacía entregas.").t(),
            tx!("Retard ? Non : le train, lui, était à l'heure.", "Late? No: the train was on time.", "Verspätung? Nein: Der Zug war pünktlich.", "In ritardo? No: il treno era puntuale.", "¿Tarde? No: el tren llegó puntual.").t(),
            tx!("Terminus. Tout le monde descend (surtout vous).", "Last stop. Everybody out (especially you).", "Endstation. Alles aussteigen (besonders Sie).", "Capolinea. Tutti giù (soprattutto tu).", "Fin de trayecto. Todos abajo (sobre todo usted).").t(),
        ],
        (Cause::Squashed, false) => vec![
            tx!("Livraison express. Un peu trop express.", "Express delivery. A little too express.", "Expresslieferung. Ein bisschen zu express.", "Consegna express. Un po' troppo express.", "Entrega exprés. Un poco demasiado exprés.").t(),
            tx!("Vous avez livré votre dernier colis.", "You delivered your last parcel.", "Das war Ihr letztes Paket.", "Era il tuo ultimo pacco.", "Fue su último paquete.").t(),
            tx!("Le code de la route vous avait prévenu.", "The highway code did warn you.", "Die Straßenverkehrsordnung hat Sie gewarnt.", "Il codice della strada ti aveva avvertito.", "El código de circulación se lo advirtió.").t(),
            tx!("Livraison annulée. Pare-chocs : 1, livreur : 0.", "Delivery cancelled. Bumper: 1, courier: 0.", "Lieferung storniert. Stoßstange: 1, Kurier: 0.", "Consegna annullata. Paraurti: 1, corriere: 0.", "Entrega cancelada. Parachoques: 1, repartidor: 0.").t(),
        ],
        (Cause::Drowned, _) => vec![
            tx!("La livraison est tombée à l'eau.", "The delivery fell in the water.", "Die Lieferung ist ins Wasser gefallen.", "La consegna è caduta in acqua.", "La entrega cayó al agua.").t(),
            tx!("Colis étanche : non. Livreur étanche : non plus.", "Waterproof parcel: no. Waterproof courier: no either.", "Wasserdichtes Paket: nein. Wasserdichter Kurier: auch nicht.", "Pacco impermeabile: no. Corriere impermeabile: neanche.", "Paquete impermeable: no. Repartidor impermeable: tampoco.").t(),
            tx!("Plouf ! Livraison à la nage non incluse.", "Splash! Swimming delivery not included.", "Platsch! Schwimmlieferung nicht inbegriffen.", "Splash! Consegna a nuoto non inclusa.", "¡Plof! Entrega a nado no incluida.").t(),
            tx!("Vous ne savez pas nager, mais vous savez couler.", "You can't swim, but you can sink.", "Schwimmen können Sie nicht, sinken schon.", "Non sai nuotare, ma sai affondare.", "No sabe nadar, pero sí hundirse.").t(),
        ],
        (Cause::CarriedAway, _) => vec![
            tx!("Emporté par le courant.", "Swept away by the current.", "Von der Strömung fortgerissen.", "Trascinato dalla corrente.", "Arrastrado por la corriente.").t(),
            tx!("La rivière a pris livraison, à votre place.", "The river took delivery in your place.", "Der Fluss hat die Lieferung für Sie übernommen.", "Il fiume ha ritirato la consegna al posto tuo.", "El río recibió la entrega en su lugar.").t(),
            tx!("Livraison à la dérive.", "Delivery adrift.", "Lieferung treibt davon.", "Consegna alla deriva.", "Entrega a la deriva.").t(),
        ],
        (Cause::Eagle, _) => vec![
            tx!("Votre colis a pris son envol. Vous aussi.", "Your parcel took off. So did you.", "Ihr Paket ist abgehoben. Sie auch.", "Il tuo pacco ha preso il volo. Anche tu.", "Su paquete echó a volar. Usted también.").t(),
            tx!("Livraison par les airs. Sans parachute.", "Air delivery. No parachute.", "Luftlieferung. Ohne Fallschirm.", "Consegna aerea. Senza paracadute.", "Entrega aérea. Sin paracaídas.").t(),
            tx!("Le ciel aussi vous déteste.", "The sky hates you too.", "Der Himmel hasst Sie auch.", "Anche il cielo ti odia.", "El cielo también le odia.").t(),
            tx!("Enlevé par l'aigle ! Aigle Prime : livraison en 1 seconde.", "Carried off by the eagle! Eagle Prime: delivery in 1 second.", "Vom Adler geholt! Adler Prime: Lieferung in 1 Sekunde.", "Rapito dall'aquila! Aquila Prime: consegna in 1 secondo.", "¡Raptado por el águila! Águila Prime: entrega en 1 segundo.").t(),
        ],
        (Cause::TooSlow, _) => vec![
            tx!("Livraison hors délai. Réclamation impossible.", "Delivery overdue. No complaints accepted.", "Lieferung verspätet. Keine Reklamation möglich.", "Consegna in ritardo. Nessun reclamo possibile.", "Entrega fuera de plazo. Reclamación imposible.").t(),
            tx!("Trop lent ! Même les colis vous dépassent.", "Too slow! Even the parcels overtake you.", "Zu langsam! Selbst die Pakete überholen Sie.", "Troppo lento! Persino i pacchi ti superano.", "¡Demasiado lento! Hasta los paquetes le adelantan.").t(),
            tx!("Le client attendait depuis ce matin.", "The customer has been waiting since this morning.", "Der Kunde wartet seit heute Morgen.", "Il cliente aspetta da stamattina.", "El cliente esperaba desde esta mañana.").t(),
        ],
        (Cause::Void, _) => vec![
            tx!("Votre livraison est partie dans une autre galaxie.", "Your delivery went to another galaxy.", "Ihre Lieferung ging in eine andere Galaxie.", "La tua consegna è finita in un'altra galassia.", "Su entrega se fue a otra galaxia.").t(),
            tx!("Adresse introuvable : le vide.", "Address not found: the void.", "Adresse nicht gefunden: das Nichts.", "Indirizzo non trovato: il vuoto.", "Dirección no encontrada: el vacío.").t(),
            tx!("Perdu dans le vide !", "Lost in the void!", "Im Nichts verloren!", "Perso nel vuoto!", "¡Perdido en el vacío!").t(),
        ],
    };
    choix[rng.below(choix.len() as u32) as usize]
}
