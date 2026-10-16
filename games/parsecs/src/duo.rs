//! Le duo de commentateurs : le pilote et Boulon (le petit drone de NOVA) se
//! répondent dans une bulle pendant la traversée. Plusieurs répliques par
//! situation, tirées au hasard ; ton drôle et familial, adapté dans chaque
//! langue.
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::ui::ThemedPanel;
use game_common::{Rng, tx};

use crate::GameState;
use crate::partie::Evenement;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Qui {
    Pilote,
    Boulon,
}

impl Qui {
    fn nom(self) -> &'static str {
        match self {
            Qui::Pilote => tx!("Pilote", "Pilot", "Pilot", "Pilota", "Piloto").t(),
            Qui::Boulon => tx!("Boulon", "Bolt", "Bolzen", "Bullone", "Tornillo").t(),
        }
    }
    fn couleur(self) -> Color {
        match self {
            Qui::Pilote => Color::srgb(0.45, 0.85, 1.0),
            Qui::Boulon => Color::srgb(1.0, 0.65, 0.25),
        }
    }
}

type Echange = &'static [(Qui, Texte)];

use Qui::{Boulon, Pilote};

const DEBUT: &[Echange] = &[
    &[(Pilote, tx!("Ceinture en vue. Douze parsecs à traverser.", "Belt in sight. Twelve parsecs to cross.", "Gürtel in Sicht. Zwölf Parsec zu durchqueren.", "Cintura in vista. Dodici parsec da attraversare.", "Cinturón a la vista. Doce pársecs por cruzar.")),
      (Boulon, tx!("Douze ?! J'ai oublié mon casque…", "Twelve?! I forgot my helmet…", "Zwölf?! Ich hab meinen Helm vergessen…", "Dodici?! Ho dimenticato il casco…", "¡¿Doce?! Me he olvidado el casco…"))],
    &[(Boulon, tx!("Réacteurs chauds, boucliers prêts !", "Engines hot, shields ready!", "Triebwerke heiß, Schilde bereit!", "Motori caldi, scudi pronti!", "¡Motores calientes, escudos listos!")),
      (Pilote, tx!("Alors on y va. Accroche-toi, Boulon.", "Then let's go. Hold on tight, Bolt.", "Dann los. Halt dich fest, Bolzen.", "Allora andiamo. Tieniti forte, Bullone.", "Pues vamos. Agárrate, Tornillo."))],
    &[(Pilote, tx!("Personne n'a jamais traversé en entier.", "Nobody has ever made it all the way.", "Niemand hat es je ganz hindurch geschafft.", "Nessuno l'ha mai attraversata tutta.", "Nadie la ha cruzado entera jamás.")),
      (Boulon, tx!("Il y a un début à tout… non ?", "There's a first time for everything… right?", "Es gibt für alles ein erstes Mal… oder?", "C'è una prima volta per tutto… no?", "Siempre hay una primera vez… ¿no?"))],
];

const PALIER: &[Echange] = &[
    &[(Boulon, tx!("Ouah… on s'en sort bien !", "Wow… we're doing great!", "Wow… wir schlagen uns gut!", "Wow… ce la caviamo bene!", "¡Guau… lo estamos haciendo genial!"))],
    &[(Pilote, tx!("Pas mal, hein ?", "Not bad, eh?", "Nicht schlecht, was?", "Niente male, eh?", "No está mal, ¿eh?")),
      (Boulon, tx!("Je n'ai même pas eu le temps d'avoir peur !", "I didn't even have time to get scared!", "Ich hatte nicht mal Zeit, Angst zu haben!", "Non ho nemmeno avuto il tempo di avere paura!", "¡Ni me ha dado tiempo a asustarme!"))],
    &[(Boulon, tx!("Continue comme ça, pilote !", "Keep it up, pilot!", "Weiter so, Pilot!", "Continua così, pilota!", "¡Sigue así, piloto!"))],
    &[(Boulon, tx!("Je note tout pour mes mémoires.", "I'm writing it all down for my memoirs.", "Ich schreibe alles für meine Memoiren auf.", "Annoto tutto per le mie memorie.", "Lo apunto todo para mis memorias.")),
      (Pilote, tx!("Mets-moi en couverture.", "Put me on the cover.", "Bring mich aufs Titelbild.", "Mettimi in copertina.", "Ponme en la portada."))],
];

const BOSS: &[Echange] = &[
    &[(Pilote, tx!("Ouch, un big boss arrive…", "Ouch, a big boss is coming…", "Autsch, ein dicker Brocken kommt…", "Ahia, arriva un pezzo grosso…", "Uy, se acerca un jefazo…")),
      (Boulon, tx!("Il est moins large que mes fesses… mais il ne va pas être facile à détruire !", "It's not as wide as my backside… but it won't be easy to blow up!", "Er ist nicht so breit wie mein Hintern… aber leicht zu zerstören wird er nicht!", "Non è largo quanto il mio sedere… ma non sarà facile da distruggere!", "No es tan ancho como mi trasero… ¡pero no será fácil de destruir!"))],
    &[(Boulon, tx!("Alerte ! Rocher géant droit devant !", "Alert! Giant rock dead ahead!", "Alarm! Riesenfels direkt voraus!", "Allarme! Roccia gigante dritto davanti!", "¡Alerta! ¡Roca gigante justo delante!")),
      (Pilote, tx!("Vise les fissures. Et prie.", "Aim for the cracks. And pray.", "Ziel auf die Risse. Und bete.", "Mira alle crepe. E prega.", "Apunta a las grietas. Y reza."))],
    &[(Pilote, tx!("C'est une lune, ça ?", "Is that a moon?", "Ist das ein Mond?", "Quella è una luna?", "¿Eso es una luna?")),
      (Boulon, tx!("Non. Mais ça en a la taille !", "No. But it's the size of one!", "Nein. Aber so groß wie einer!", "No. Ma ne ha le dimensioni!", "No. ¡Pero tiene su tamaño!"))],
];

const BOSS_DETRUIT: &[Echange] = &[
    &[(Boulon, tx!("Il est tombé ! On l'a eu !", "It's down! We got it!", "Er ist hinüber! Wir haben ihn!", "È caduto! L'abbiamo preso!", "¡Ha caído! ¡Lo tenemos!"))],
    &[(Pilote, tx!("Et un caillou de moins.", "One less pebble.", "Ein Kieselstein weniger.", "Un sassolino in meno.", "Un pedrusco menos.")),
      (Boulon, tx!("Un caillou ? C'était une montagne !", "A pebble? That was a mountain!", "Ein Kiesel? Das war ein Berg!", "Un sassolino? Era una montagna!", "¿Un pedrusco? ¡Era una montaña!"))],
];

const FROLE: &[Echange] = &[
    &[(Boulon, tx!("C'était moins une !", "That was close!", "Das war knapp!", "C'è mancato poco!", "¡Por los pelos!"))],
    &[(Boulon, tx!("J'ai senti le vent du rocher !", "I felt the rock go by!", "Ich hab den Fels vorbeisausen gespürt!", "Ho sentito il vento della roccia!", "¡He sentido pasar la roca!")),
      (Pilote, tx!("Il ne m'a même pas décoiffé.", "Didn't even mess up my hair.", "Hat mir nicht mal die Frisur zerzaust.", "Non mi ha nemmeno spettinato.", "Ni me ha despeinado."))],
    &[(Pilote, tx!("Ouf. Un poil trop près.", "Phew. A hair too close.", "Puh. Ein Haar zu nah.", "Uff. Un pelo troppo vicino.", "Uf. Un pelín demasiado cerca."))],
];

const FAIBLE: &[Echange] = &[
    &[(Boulon, tx!("On prend l'eau, là !", "We're taking on water here!", "Wir saufen hier ab!", "Stiamo imbarcando acqua!", "¡Que nos hundimos!")),
      (Pilote, tx!("Dans l'espace, Boulon ?", "In space, Bolt?", "Im Weltraum, Bolzen?", "Nello spazio, Bullone?", "¿En el espacio, Tornillo?"))],
    &[(Boulon, tx!("Bouclier presque vide ! Je ne regarde plus !", "Shield almost empty! I can't look!", "Schild fast leer! Ich schau nicht mehr hin!", "Scudo quasi vuoto! Non guardo più!", "¡Escudo casi vacío! ¡No puedo mirar!"))],
];

const BONUS: &[Echange] = &[
    &[(Boulon, tx!("Ça, c'est un cadeau !", "Now that's a gift!", "Na, das ist ein Geschenk!", "Questo sì che è un regalo!", "¡Eso sí que es un regalo!"))],
    &[(Pilote, tx!("Joli bonus.", "Nice bonus.", "Schöner Bonus.", "Bel bonus.", "Buen bonus.")),
      (Boulon, tx!("Je peux le garder pour ma collection ?", "Can I keep it for my collection?", "Darf ich ihn für meine Sammlung behalten?", "Posso tenerlo per la mia collezione?", "¿Me lo puedo quedar para mi colección?"))],
    &[(Boulon, tx!("Ramassé ! Ça brille, ça sent le neuf !", "Got it! Shiny, and it smells brand new!", "Hab ihn! Glänzt und riecht ganz neu!", "Preso! Brilla e profuma di nuovo!", "¡Cogido! ¡Brilla y huele a nuevo!"))],
];

const PARSEC: &[Echange] = &[
    &[(Boulon, tx!("Un parsec de plus ! On avance !", "Another parsec! We're getting there!", "Noch ein Parsec! Wir kommen voran!", "Un altro parsec! Stiamo avanzando!", "¡Otro pársec! ¡Avanzamos!"))],
    &[(Pilote, tx!("Parsec franchi. Ça se resserre devant.", "Parsec cleared. It's getting thicker ahead.", "Parsec geschafft. Vorne wird es dichter.", "Parsec superato. Davanti si infittisce.", "Pársec superado. Delante se pone más denso.")),
      (Boulon, tx!("Plus de rochers ? Génial… enfin, je crois.", "More rocks? Great… I think.", "Mehr Felsen? Super… glaube ich.", "Più rocce? Fantastico… credo.", "¿Más rocas? Genial… creo."))],
];

const TRAVERSEE: &[Echange] = &[
    &[(Boulon, tx!("Douze parsecs ! ON L'A FAIT !", "Twelve parsecs! WE DID IT!", "Zwölf Parsec! WIR HABEN ES GESCHAFFT!", "Dodici parsec! CE L'ABBIAMO FATTA!", "¡Doce pársecs! ¡LO HEMOS LOGRADO!")),
      (Pilote, tx!("Et si on continuait, pour voir ?", "How about we keep going, just to see?", "Und wenn wir einfach weiterfliegen?", "E se continuassimo, tanto per vedere?", "¿Y si seguimos, a ver qué pasa?"))],
];

const RECORD: &[Echange] = &[
    &[(Boulon, tx!("Record battu ! Je le savais !", "New record! I knew it!", "Neuer Rekord! Ich wusste es!", "Record battuto! Lo sapevo!", "¡Récord batido! ¡Lo sabía!"))],
    &[(Pilote, tx!("Nouveau record.", "New record.", "Neuer Rekord.", "Nuovo record.", "Nuevo récord.")),
      (Boulon, tx!("Je fais encadrer l'écran !", "I'm framing the screen!", "Ich lasse den Bildschirm einrahmen!", "Faccio incorniciare lo schermo!", "¡Voy a enmarcar la pantalla!"))],
];

const DEBLOQUE: &[Echange] = &[
    &[(Boulon, tx!("Regarde ce qui nous attend au hangar…", "Look what's waiting for us in the hangar…", "Schau, was im Hangar auf uns wartet…", "Guarda cosa ci aspetta nell'hangar…", "Mira lo que nos espera en el hangar…"))],
];

const TOUCHE: &[Echange] = &[
    &[(Boulon, tx!("Aïe ! Mon vernis !", "Ouch! My paint job!", "Au! Mein Lack!", "Ahi! La mia vernice!", "¡Ay! ¡Mi pintura!"))],
    &[(Pilote, tx!("Bouclier touché, ça tient.", "Shield hit, it's holding.", "Schild getroffen, er hält.", "Scudo colpito, regge.", "Escudo dañado, aguanta."))],
];

fn repliques(e: Evenement) -> &'static [Echange] {
    match e {
        Evenement::Debut => DEBUT,
        Evenement::Palier => PALIER,
        Evenement::Boss => BOSS,
        Evenement::BossDetruit => BOSS_DETRUIT,
        Evenement::Frole => FROLE,
        Evenement::BouclierFaible => FAIBLE,
        Evenement::Bonus => BONUS,
        Evenement::Parsec(_) => PARSEC,
        Evenement::Traversee => TRAVERSEE,
        Evenement::Record => RECORD,
        Evenement::Debloque => DEBLOQUE,
        Evenement::Touche => TOUCHE,
    }
}

/// Importance : une réplique importante passe devant les autres.
fn important(e: Evenement) -> bool {
    matches!(e, Evenement::Boss | Evenement::Traversee | Evenement::Record | Evenement::Debloque | Evenement::BouclierFaible)
}

/// Quelqu'un parle (le son joue un petit « blabla » pour Boulon).
#[derive(Message, Clone, Copy)]
pub struct Parle(pub Qui);

pub struct DuoPlugin;

impl Plugin for DuoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Duo { file: Vec::new(), temps: 0.0, rng: Rng::from_time(), calme: 0.0 })
            .add_message::<Parle>()
            .add_systems(OnEnter(GameState::Playing), bulle)
            .add_systems(OnExit(GameState::Playing), |mut d: ResMut<Duo>| d.file.clear())
            .add_systems(Update, (ecouter, parler).chain().run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
struct Duo {
    /// Répliques en attente.
    file: Vec<(Qui, Texte)>,
    /// Temps restant pour la réplique affichée.
    temps: f32,
    rng: Rng,
    /// Délai minimal avant un commentaire peu important.
    calme: f32,
}

#[derive(Component)]
struct Bulle;

#[derive(Component)]
struct Nom;

#[derive(Component)]
struct Phrase;

fn bulle(mut commands: Commands) {
    let racine = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                bottom: Val::Px(16.0),
                max_width: Val::Px(460.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            ThemedPanel,
            Visibility::Hidden,
            Pickable::IGNORE,
            Bulle,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    commands.spawn((Text::new(""), TextFont::from_font_size(15.0), TextColor(Color::WHITE), Nom, Pickable::IGNORE, ChildOf(racine)));
    commands.spawn((Text::new(""), TextFont::from_font_size(19.0), TextColor(Color::WHITE), Phrase, Pickable::IGNORE, ChildOf(racine)));
}

fn ecouter(mut duo: ResMut<Duo>, mut evenements: MessageReader<Evenement>, time: Res<Time>) {
    duo.calme = (duo.calme - time.delta_secs()).max(0.0);
    for e in evenements.read() {
        let imp = important(*e);
        // On ne coupe pas la parole pour un détail ; l'important passe devant.
        if !imp && (duo.calme > 0.0 || !duo.file.is_empty()) {
            continue;
        }
        if imp {
            duo.file.clear();
            duo.temps = 0.0;
        }
        let choix = repliques(*e);
        let i = duo.rng.below(choix.len() as u32) as usize;
        duo.file.extend(choix[i].iter().copied());
        duo.calme = 6.0;
    }
}

fn parler(
    time: Res<Time>,
    mut duo: ResMut<Duo>,
    mut bulles: Query<&mut Visibility, With<Bulle>>,
    mut noms: Query<(&mut Text, &mut TextColor), (With<Nom>, Without<Phrase>)>,
    mut phrases: Query<&mut Text, (With<Phrase>, Without<Nom>)>,
    mut parle: MessageWriter<Parle>,
) {
    duo.temps -= time.delta_secs();
    if duo.temps > 0.0 {
        return;
    }
    let Ok(mut vis) = bulles.single_mut() else { return };
    if duo.file.is_empty() {
        *vis = Visibility::Hidden;
        return;
    }
    let (qui, texte) = duo.file.remove(0);
    let phrase = texte.t();
    // Le temps de lire : environ une seconde pour 14 lettres, au moins 2,2 s.
    duo.temps = (phrase.chars().count() as f32 / 14.0).max(2.2);
    *vis = Visibility::Inherited;
    if let Ok((mut t, mut c)) = noms.single_mut() {
        t.0 = qui.nom().to_uppercase();
        c.0 = qui.couleur();
    }
    if let Ok(mut t) = phrases.single_mut() {
        t.0 = phrase.to_string();
    }
    parle.write(Parle(qui));
}
