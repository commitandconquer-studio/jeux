//! L'histoire : les 5 chapitres, leurs réglages et les dialogues.
//!
//! Univers original Commit & Conquer : la colonie libre de Nova résiste à
//! l'Hégémonie. Isa Valen, ingénieure en chef, bâtit le Bouclier de Nova ;
//! Boulon, petit drone de maintenance, l'accompagne (et commente).
use bevy::prelude::Color;
use game_common::langue::Texte;
use game_common::tx;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Speaker {
    Isa,
    Boulon,
}

/// Le nom du drone, dans chaque langue.
pub const BOULON_NOM: Texte = tx!("Boulon", "Bolt", "Bolzen", "Bullone", "Tornillo");

impl Speaker {
    pub fn name(self) -> &'static str {
        match self {
            Speaker::Isa => "Isa Valen",
            Speaker::Boulon => BOULON_NOM.t(),
        }
    }

    pub fn color(self) -> Color {
        match self {
            Speaker::Isa => Color::srgb(0.45, 0.78, 1.0),
            Speaker::Boulon => Color::srgb(1.0, 0.70, 0.30),
        }
    }
}

pub type Line = (Speaker, Texte);

pub struct Chapter {
    pub name: Texte,
    pub planet: Color,
    /// La planète brille (lave).
    pub planet_glow: bool,
    /// Points de coque du croiseur ennemi.
    pub hull: f32,
    /// Secondes entre deux descentes d'un cran au début.
    pub gravity: f32,
    /// Secondes entre deux tirs ennemis.
    pub fire_every: f32,
    pub intro: &'static [Line],
    pub outro: &'static [Line],
}

use Speaker::{Boulon, Isa};

pub const CHAPTERS: [Chapter; 5] = [
    Chapter {
        name: tx!("Nova Prime", "Nova Prime", "Nova Prime", "Nova Prime", "Nova Prime"),
        planet: Color::srgb(0.30, 0.62, 0.42),
        planet_glow: false,
        hull: 100.0,
        gravity: 0.9,
        fire_every: 40.0,
        intro: &[
            (Isa, tx!(
                "Boulon, réveille-toi. Un croiseur de l'Hégémonie vient de sortir de l'hyperespace.",
                "Bolt, wake up. A Hegemony cruiser just dropped out of hyperspace.",
                "Bolzen, aufwachen. Ein Kreuzer der Hegemonie ist gerade aus dem Hyperraum gesprungen.",
                "Bullone, svegliati. Un incrociatore dell'Egemonia è appena uscito dall'iperspazio.",
                "Tornillo, despierta. Un crucero de la Hegemonía acaba de salir del hiperespacio.",
            )),
            (Boulon, tx!(
                "Un croiseur ? Un gros ? Avec des canons ? Je peux aller me cacher dans la soute ?",
                "A cruiser? A big one? With guns? Can I go hide in the cargo hold?",
                "Ein Kreuzer? Ein großer? Mit Kanonen? Darf ich mich im Frachtraum verstecken?",
                "Un incrociatore? Grosso? Con i cannoni? Posso andare a nascondermi nella stiva?",
                "¿Un crucero? ¿Grande? ¿Con cañones? ¿Puedo ir a esconderme en la bodega?",
            )),
            (Isa, tx!(
                "Le Bouclier de Nova est notre seule défense. Chaque rangée d'énergie complète charge le canon à ions.",
                "The Nova Shield is our only defence. Every full row of energy charges the ion cannon.",
                "Der Nova-Schild ist unsere einzige Verteidigung. Jede volle Energiereihe lädt die Ionenkanone.",
                "Lo Scudo di Nova è la nostra unica difesa. Ogni fila di energia completa carica il cannone a ioni.",
                "El Escudo de Nova es nuestra única defensa. Cada fila de energía completa carga el cañón de iones.",
            )),
            (Isa, tx!(
                "Plus on complète de rangées d'un coup, plus le tir est puissant. On tient bon.",
                "The more rows we clear at once, the harder the shot. We hold the line.",
                "Je mehr Reihen auf einmal, desto stärker der Schuss. Wir halten durch.",
                "Più file completiamo in un colpo, più il tiro è potente. Teniamo duro.",
                "Cuantas más filas completemos de golpe, más potente es el disparo. Aguantamos.",
            )),
        ],
        outro: &[
            (Boulon, tx!(
                "Il… il recule ! Il fume ! On l'a eu !",
                "It's… it's pulling back! It's smoking! We got it!",
                "Er… er zieht sich zurück! Er raucht! Wir haben ihn erwischt!",
                "Si… si ritira! Fuma! L'abbiamo preso!",
                "¡Se… se retira! ¡Echa humo! ¡Lo tenemos!",
            )),
            (Isa, tx!(
                "Ce n'était qu'un éclaireur. L'Hégémonie en enverra d'autres. Cap sur Kessa.",
                "That was only a scout. The Hegemony will send more. Set course for Kessa.",
                "Das war nur ein Späher. Die Hegemonie wird weitere schicken. Kurs auf Kessa.",
                "Era solo un esploratore. L'Egemonia ne manderà altri. Rotta su Kessa.",
                "Solo era un explorador. La Hegemonía enviará más. Rumbo a Kessa.",
            )),
        ],
    },
    Chapter {
        name: tx!("Kessa", "Kessa", "Kessa", "Kessa", "Kessa"),
        planet: Color::srgb(0.85, 0.64, 0.38),
        planet_glow: false,
        hull: 150.0,
        gravity: 0.75,
        fire_every: 32.0,
        intro: &[
            (Isa, tx!(
                "Les raffineries de Kessa alimentent toute la colonie. Si elles tombent, Nova gèle.",
                "Kessa's refineries power the whole colony. If they fall, Nova freezes.",
                "Die Raffinerien von Kessa versorgen die ganze Kolonie. Fallen sie, erfriert Nova.",
                "Le raffinerie di Kessa alimentano tutta la colonia. Se cadono, Nova gela.",
                "Las refinerías de Kessa abastecen a toda la colonia. Si caen, Nova se congela.",
            )),
            (Boulon, tx!(
                "J'ai du sable dans les engrenages. Rien de grave. Enfin, j'espère.",
                "I've got sand in my gears. Nothing serious. I hope.",
                "Ich habe Sand im Getriebe. Nichts Schlimmes. Hoffe ich.",
                "Ho della sabbia negli ingranaggi. Niente di grave. Spero.",
                "Tengo arena en los engranajes. Nada grave. Eso espero.",
            )),
            (Isa, tx!(
                "Attention : leurs tirs abîment le bas du bouclier. Garde-le propre.",
                "Careful: their shots damage the bottom of the shield. Keep it clean.",
                "Vorsicht: Ihre Schüsse beschädigen den unteren Teil des Schilds. Halte ihn sauber.",
                "Attento: i loro colpi danneggiano la base dello scudo. Tienila pulita.",
                "Cuidado: sus disparos dañan la base del escudo. Mantenla limpia.",
            )),
        ],
        outro: &[
            (Isa, tx!(
                "Les raffineries sont sauves. Mais leurs croiseurs deviennent plus rapides.",
                "The refineries are safe. But their cruisers are getting faster.",
                "Die Raffinerien sind gerettet. Aber ihre Kreuzer werden schneller.",
                "Le raffinerie sono salve. Ma i loro incrociatori diventano più veloci.",
                "Las refinerías están a salvo. Pero sus cruceros son cada vez más rápidos.",
            )),
            (Boulon, tx!(
                "Plus rapides ? Moi aussi je peux être rapide. Surtout pour fuir.",
                "Faster? I can be fast too. Especially when running away.",
                "Schneller? Ich kann auch schnell sein. Vor allem beim Weglaufen.",
                "Più veloci? Anch'io so essere veloce. Soprattutto quando scappo.",
                "¿Más rápidos? Yo también sé ser rápido. Sobre todo huyendo.",
            )),
        ],
    },
    Chapter {
        name: tx!("Hélion", "Helion", "Helion", "Helion", "Helión"),
        planet: Color::srgb(0.75, 0.88, 0.98),
        planet_glow: false,
        hull: 200.0,
        gravity: 0.62,
        fire_every: 26.0,
        intro: &[
            (Boulon, tx!(
                "Il fait moins 140 degrés. Mes capteurs ont froid. Est-ce que les capteurs peuvent avoir froid ?",
                "It's minus 140 degrees. My sensors are cold. Can sensors even get cold?",
                "Es hat minus 140 Grad. Meine Sensoren frieren. Können Sensoren überhaupt frieren?",
                "Ci sono meno 140 gradi. I miei sensori hanno freddo. Ma i sensori possono avere freddo?",
                "Estamos a menos 140 grados. Mis sensores tienen frío. ¿Pueden los sensores tener frío?",
            )),
            (Isa, tx!(
                "Concentre-toi. Ils tirent plus souvent ici : garde le bouclier bas.",
                "Focus. They fire more often here: keep the shield low.",
                "Konzentrier dich. Hier schießen sie öfter: Halte den Schild niedrig.",
                "Concentrati. Qui sparano più spesso: tieni basso lo scudo.",
                "Concéntrate. Aquí disparan más a menudo: mantén el escudo bajo.",
            )),
        ],
        outro: &[
            (Boulon, tx!(
                "La lune de glace tient ! Et moi aussi, à peu près.",
                "The ice moon is holding! And so am I, more or less.",
                "Der Eismond hält! Und ich auch, so ungefähr.",
                "La luna di ghiaccio regge! E anch'io, più o meno.",
                "¡La luna de hielo resiste! Y yo también, más o menos.",
            )),
            (Isa, tx!(
                "Prochaine étape : Thalassa. Les familles évacuées comptent sur nous.",
                "Next stop: Thalassa. The evacuated families are counting on us.",
                "Nächstes Ziel: Thalassa. Die evakuierten Familien zählen auf uns.",
                "Prossima tappa: Thalassa. Le famiglie evacuate contano su di noi.",
                "Próxima parada: Thalassa. Las familias evacuadas cuentan con nosotros.",
            )),
        ],
    },
    Chapter {
        name: tx!("Thalassa", "Thalassa", "Thalassa", "Thalassa", "Thalassa"),
        planet: Color::srgb(0.20, 0.50, 0.85),
        planet_glow: false,
        hull: 260.0,
        gravity: 0.52,
        fire_every: 22.0,
        intro: &[
            (Isa, tx!(
                "Thalassa abrite les familles évacuées. Pas question de reculer.",
                "Thalassa shelters the evacuated families. Retreat is not an option.",
                "Auf Thalassa leben die evakuierten Familien. Rückzug kommt nicht in Frage.",
                "Thalassa ospita le famiglie evacuate. Ritirarsi è fuori discussione.",
                "Thalassa acoge a las familias evacuadas. Retroceder no es una opción.",
            )),
            (Boulon, tx!(
                "Pour une fois, je suis d'accord. On tient. Et on gagne. Dans cet ordre.",
                "For once, I agree. We hold. And we win. In that order.",
                "Ausnahmsweise bin ich einverstanden. Wir halten. Und wir gewinnen. In dieser Reihenfolge.",
                "Per una volta sono d'accordo. Resistiamo. E vinciamo. In quest'ordine.",
                "Por una vez, estoy de acuerdo. Resistimos. Y ganamos. En ese orden.",
            )),
        ],
        outro: &[
            (Isa, tx!(
                "Thalassa est sauvée. Il est temps de porter le combat chez eux.",
                "Thalassa is saved. Time to take the fight to them.",
                "Thalassa ist gerettet. Zeit, den Kampf zu ihnen zu tragen.",
                "Thalassa è salva. È ora di portare la battaglia da loro.",
                "Thalassa está a salvo. Es hora de llevar la lucha a su terreno.",
            )),
            (Boulon, tx!(
                "Chez eux ? Vous voulez dire… à la Forge ? Oh non. Oh non non non.",
                "To them? You mean… to the Forge? Oh no. Oh no no no.",
                "Zu ihnen? Du meinst… zur Schmiede? Oh nein. Oh nein, nein, nein.",
                "Da loro? Vuoi dire… alla Forgia? Oh no. Oh no no no.",
                "¿A su terreno? ¿Quieres decir… a la Forja? Oh, no. Oh, no, no, no.",
            )),
        ],
    },
    Chapter {
        name: tx!("La Forge", "The Forge", "Die Schmiede", "La Forgia", "La Forja"),
        planet: Color::srgb(0.95, 0.38, 0.10),
        planet_glow: true,
        hull: 340.0,
        gravity: 0.42,
        fire_every: 18.0,
        intro: &[
            (Isa, tx!(
                "Voilà la Forge, le chantier de l'Hégémonie. Et leur vaisseau amiral, le Souverain.",
                "There's the Forge, the Hegemony's shipyard. And their flagship, the Sovereign.",
                "Da ist die Schmiede, die Werft der Hegemonie. Und ihr Flaggschiff, die Souverän.",
                "Ecco la Forgia, il cantiere dell'Egemonia. E la loro ammiraglia, il Sovrano.",
                "Ahí está la Forja, el astillero de la Hegemonía. Y su buque insignia, el Soberano.",
            )),
            (Boulon, tx!(
                "Il est… très gros. Très, très gros.",
                "It's… very big. Very, very big.",
                "Sie ist… sehr groß. Sehr, sehr groß.",
                "È… molto grosso. Molto, molto grosso.",
                "Es… muy grande. Muy, muy grande.",
            )),
            (Isa, tx!(
                "Plus il est gros, plus il est facile à toucher. Chargez le canon.",
                "The bigger it is, the easier it is to hit. Charge the cannon.",
                "Je größer, desto leichter zu treffen. Kanone laden.",
                "Più è grosso, più è facile da colpire. Caricate il cannone.",
                "Cuanto más grande, más fácil de acertar. Cargad el cañón.",
            )),
        ],
        outro: &[
            (Boulon, tx!(
                "Le Souverain se replie ! Toute la flotte se replie !",
                "The Sovereign is retreating! The whole fleet is retreating!",
                "Die Souverän zieht sich zurück! Die ganze Flotte zieht sich zurück!",
                "Il Sovrano si ritira! Tutta la flotta si ritira!",
                "¡El Soberano se retira! ¡Toda la flota se retira!",
            )),
            (Isa, tx!(
                "Nova est libre. Beau travail, Boulon.",
                "Nova is free. Nice work, Bolt.",
                "Nova ist frei. Gute Arbeit, Bolzen.",
                "Nova è libera. Bel lavoro, Bullone.",
                "Nova es libre. Buen trabajo, Tornillo.",
            )),
            (Boulon, tx!(
                "C'est surtout vous. Moi, j'ai juste un peu tremblé.",
                "That was mostly you. I just shook a little.",
                "Das warst vor allem du. Ich habe nur ein bisschen gezittert.",
                "È merito tuo, soprattutto. Io ho solo tremato un po'.",
                "Fuiste sobre todo tú. Yo solo temblé un poco.",
            )),
        ],
    },
];

/// Petites remarques de Boulon pendant la partie.
pub const BOULON_FOUR: Texte = tx!(
    "Quatre rangées d'un coup ! Ils ont dû le sentir passer !",
    "Four rows at once! They definitely felt that one!",
    "Vier Reihen auf einmal! Das haben sie gespürt!",
    "Quattro file in un colpo! Questa l'hanno sentita!",
    "¡Cuatro filas de golpe! ¡Eso les ha dolido!",
);
pub const BOULON_WARNING: Texte = tx!(
    "Ils chargent leurs canons ! Attention en bas !",
    "They're charging their guns! Watch the bottom!",
    "Sie laden ihre Kanonen! Achtung, unten!",
    "Stanno caricando i cannoni! Attenti in basso!",
    "¡Están cargando sus cañones! ¡Cuidado abajo!",
);
pub const BOULON_HALF: Texte = tx!(
    "Leur coque est à moitié fondue. Continuez !",
    "Their hull is half melted. Keep going!",
    "Ihre Hülle ist halb geschmolzen. Weiter so!",
    "Il loro scafo è mezzo fuso. Continuate!",
    "Su casco está medio fundido. ¡Seguid así!",
);
