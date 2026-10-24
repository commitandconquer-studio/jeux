// sfx-gen — Commit & Conquer. Licence GPL-3.0-or-later.
//
// Fabrique les sons des jeux par synthèse : pas de fichier téléchargé, donc
// aucun souci de licence. Les sons produits sont sous la même licence que le
// code (GPL). Usage :
//
//     cargo run -p sfx-gen   # depuis la racine du projet : écrit la
//                            # bibliothèque commune dans assets/sounds/
use std::f32::consts::TAU;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const SR: f32 = 44_100.0;

/// Tampon audio mono, échantillons entre -1 et 1.
struct Buf(Vec<f32>);

impl Buf {
    fn new(seconds: f32) -> Self {
        Buf(vec![0.0; (seconds * SR) as usize])
    }

    fn len_s(&self) -> f32 {
        self.0.len() as f32 / SR
    }

    /// Ajoute `f(t)` entre `start` et `start + dur` secondes.
    fn add(&mut self, start: f32, dur: f32, mut f: impl FnMut(f32) -> f32) {
        let i0 = (start * SR) as usize;
        let n = (dur * SR) as usize;
        for i in 0..n {
            if let Some(s) = self.0.get_mut(i0 + i) {
                *s += f(i as f32 / SR);
            }
        }
    }

    /// Comme `add`, mais déborde au début du tampon (boucles sans couture).
    fn add_wrapped(&mut self, start: f32, dur: f32, mut f: impl FnMut(f32) -> f32) {
        let len = self.0.len();
        let i0 = (start * SR) as usize;
        for i in 0..(dur * SR) as usize {
            self.0[(i0 + i) % len] += f(i as f32 / SR);
        }
    }

    fn normalize(&mut self, peak: f32) {
        let max = self.0.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        if max > 0.0 {
            for s in &mut self.0 {
                *s *= peak / max;
            }
        }
    }

    fn write_wav(&self, path: &Path) -> std::io::Result<()> {
        let mut w = BufWriter::new(File::create(path)?);
        let data_len = (self.0.len() * 2) as u32;
        w.write_all(b"RIFF")?;
        w.write_all(&(36 + data_len).to_le_bytes())?;
        w.write_all(b"WAVEfmt ")?;
        w.write_all(&16u32.to_le_bytes())?; // taille du bloc fmt
        w.write_all(&1u16.to_le_bytes())?; // PCM
        w.write_all(&1u16.to_le_bytes())?; // mono
        w.write_all(&(SR as u32).to_le_bytes())?;
        w.write_all(&(SR as u32 * 2).to_le_bytes())?; // octets par seconde
        w.write_all(&2u16.to_le_bytes())?; // octets par échantillon
        w.write_all(&16u16.to_le_bytes())?; // bits
        w.write_all(b"data")?;
        w.write_all(&data_len.to_le_bytes())?;
        for s in &self.0 {
            w.write_all(&((s.clamp(-1.0, 1.0) * 32_767.0) as i16).to_le_bytes())?;
        }
        Ok(())
    }
}

/// Bruit blanc déterministe.
struct Noise(u32);

impl Noise {
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0 as f32 / u32::MAX as f32 * 2.0 - 1.0
    }
}

/// Filtre passe-bas à un pôle (fréquence de coupure modifiable à chaque échantillon).
#[derive(Default)]
struct LowPass(f32);

impl LowPass {
    fn run(&mut self, x: f32, cutoff: f32) -> f32 {
        let a = 1.0 - (-TAU * cutoff / SR).exp();
        self.0 += a * (x - self.0);
        self.0
    }
}

/// Attaque linéaire puis décroissance exponentielle.
fn env(t: f32, attack: f32, decay: f32) -> f32 {
    if t < attack { t / attack } else { (-(t - attack) / decay).exp() }
}

/// Phase accumulée d'un glissando exponentiel de `f0` à `f1` sur `dur`.
fn sweep_phase(t: f32, f0: f32, f1: f32, dur: f32) -> f32 {
    let k = (f1 / f0).ln() / dur;
    if k.abs() < 1e-6 { TAU * f0 * t } else { TAU * f0 * ((k * t).exp() - 1.0) / k }
}

fn note(midi: f32) -> f32 {
    440.0 * 2f32.powf((midi - 69.0) / 12.0)
}

// ---------------------------------------------------------------- sons ----

/// Petit « bloup » montant : le saut du crapaud.
fn hop() -> Buf {
    let mut b = Buf::new(0.12);
    b.add(0.0, 0.12, |t| {
        let p = sweep_phase(t, 260.0, 640.0, 0.12);
        (p.sin() + 0.25 * (2.0 * p).sin()) * env(t, 0.004, 0.035)
    });
    b.normalize(0.7);
    b
}

/// Coassement en deux temps : « crô-âk ».
fn croak() -> Buf {
    let mut b = Buf::new(0.42);
    let mut lp = LowPass::default();
    let mut phase = 0.0;
    b.add(0.0, 0.42, |t| {
        let f = if t < 0.17 { 92.0 } else { 78.0 };
        phase += f / SR;
        let saw = 2.0 * (phase % 1.0) - 1.0;
        // « r » roulé : modulation rapide de l'amplitude
        let rattle = 0.55 + 0.45 * (TAU * 32.0 * t).sin();
        let bump = |a: f32, b: f32| if t > a && t < b { (((t - a) / (b - a)) * std::f32::consts::PI).sin() } else { 0.0 };
        let shape = bump(0.0, 0.16) + 0.9 * bump(0.19, 0.42);
        lp.run(saw * rattle, 900.0) * shape
    });
    b.normalize(0.75);
    b
}

/// Plouf : bulle grave puis gerbe d'eau (bruit filtré).
fn splash() -> Buf {
    let mut b = Buf::new(0.8);
    let mut n = Noise(12_345);
    let mut lp = LowPass::default();
    b.add(0.0, 0.8, |t| {
        let cutoff = 600.0 + 3_400.0 * (-t / 0.12).exp();
        lp.run(n.next(), cutoff) * env(t, 0.01, 0.2)
    });
    b.add(0.0, 0.18, |t| sweep_phase(t, 220.0, 90.0, 0.18).sin() * env(t, 0.003, 0.06) * 0.8);
    b.normalize(0.8);
    b
}

/// Choc : bruit sec, coup sourd et tintement de tôle.
fn crash() -> Buf {
    let mut b = Buf::new(0.6);
    let mut n = Noise(777);
    let mut lp = LowPass::default();
    b.add(0.0, 0.25, |t| {
        let x = n.next();
        (x - lp.run(x, 1_500.0)) * env(t, 0.001, 0.05)
    });
    b.add(0.0, 0.3, |t| sweep_phase(t, 110.0, 40.0, 0.3).sin() * env(t, 0.002, 0.08) * 1.2);
    b.add(0.01, 0.55, |t| {
        let metal = [523.0, 1187.0, 1663.0, 2311.0].iter().map(|f| (TAU * f * t).sin()).sum::<f32>();
        metal * env(t, 0.001, 0.12) * 0.18
    });
    b.normalize(0.85);
    b
}

/// Klaxon de voiture : deux notes saturées.
fn honk(f1: f32, f2: f32, dur: f32, beeps: usize) -> Buf {
    let mut b = Buf::new(dur * beeps as f32 + 0.05 * beeps as f32);
    for i in 0..beeps {
        let mut lp = LowPass::default();
        b.add(i as f32 * (dur + 0.05), dur, |t| {
            let x = ((TAU * f1 * t).sin() + (TAU * f2 * t).sin()) * 2.5;
            let gate = (t / 0.01).min(1.0) * ((dur - t) / 0.03).clamp(0.0, 1.0);
            lp.run(x.tanh(), 2_200.0) * gate
        });
    }
    b.normalize(0.6);
    b
}

/// Sonnerie du passage à niveau : deux « ding », jouable en boucle.
fn bell() -> Buf {
    let mut b = Buf::new(0.7);
    for start in [0.0, 0.35] {
        b.add_wrapped(start, 0.6, |t| {
            let f = 1_250.0;
            ((TAU * f * t).sin() + 0.4 * (TAU * f * 2.76 * t).sin()) * env(t, 0.002, 0.12)
        });
    }
    b.normalize(0.5);
    b
}

/// Train : klaxon en accord puis grondement rythmé des roues.
fn train() -> Buf {
    let mut b = Buf::new(2.4);
    let chord = [311.0, 370.0, 466.0];
    let mut lp = LowPass::default();
    b.add(0.0, 0.9, |t| {
        let x: f32 = chord.iter().map(|f| 2.0 * ((f * t) % 1.0) - 1.0).sum();
        let gate = (t / 0.03).min(1.0) * ((0.9 - t) / 0.15).clamp(0.0, 1.0);
        lp.run(x, 1_400.0) * gate * 0.5
    });
    let mut n = Noise(4242);
    let mut lp2 = LowPass::default();
    b.add(0.1, 2.3, |t| {
        let fade = (t / 0.3).min(1.0) * ((2.3 - t) / 0.8).clamp(0.0, 1.0);
        let clack = 0.6 + 0.4 * (-((t * 7.0) % 1.0) / 0.12).exp();
        lp2.run(n.next(), 350.0) * fade * clack * 3.0
    });
    b.normalize(0.85);
    b
}

/// Carillon montant (toutes les 10 rangées).
fn chime() -> Buf {
    let mut b = Buf::new(0.9);
    for (i, m) in [84.0, 88.0, 91.0].iter().enumerate() {
        let f = note(*m);
        b.add(i as f32 * 0.08, 0.7, |t| ((TAU * f * t).sin() + 0.2 * (TAU * f * 3.0 * t).sin()) * env(t, 0.003, 0.18));
    }
    b.normalize(0.55);
    b
}

/// Clic des boutons.
fn click() -> Buf {
    let mut b = Buf::new(0.05);
    b.add(0.0, 0.05, |t| sweep_phase(t, 1_200.0, 700.0, 0.05).sin() * env(t, 0.001, 0.012));
    b.normalize(0.5);
    b
}

/// Musique de fond : 8 mesures à 112 BPM, en boucle sans couture.
/// Grille Do – La m – Fa – Sol, basse douce, accords, mélodie pentatonique.
fn music() -> Buf {
    let beat = 60.0 / 112.0;
    let bars = 8;
    let mut b = Buf::new(beat * 4.0 * bars as f32);
    let chords: [[f32; 3]; 4] = [[60.0, 64.0, 67.0], [57.0, 60.0, 64.0], [53.0, 57.0, 60.0], [55.0, 59.0, 62.0]];
    for bar in 0..bars {
        let chord = chords[bar / 2 % 4];
        let t0 = bar as f32 * 4.0 * beat;
        // Basse (triangle) sur les temps 1 et 3
        for k in [0.0, 2.0] {
            let f = note(chord[0] - 12.0);
            b.add_wrapped(t0 + k * beat, beat * 1.8, |t| {
                let tri = 2.0 * (2.0 * ((f * t) % 1.0) - 1.0).abs() - 1.0;
                tri * env(t, 0.01, 0.5) * 0.45
            });
        }
        // Accords en notes répétées douces (croches)
        for e in 0..8 {
            for m in chord {
                let f = note(m);
                b.add_wrapped(t0 + e as f32 * beat / 2.0, beat * 0.5, |t| (TAU * f * t).sin() * env(t, 0.005, 0.09) * 0.08);
            }
        }
        // Charleston sur les contretemps
        let mut n = Noise(99 + bar as u32);
        let mut lp = LowPass::default();
        for e in 0..4 {
            b.add_wrapped(t0 + (e as f32 + 0.5) * beat, 0.06, |t| {
                let x = n.next();
                (x - lp.run(x, 6_000.0)) * env(t, 0.001, 0.015) * 0.18
            });
        }
    }
    // Mélodie (do majeur pentatonique), 2 phrases de 4 mesures ; 0 = silence.
    let melody: [f32; 32] = [
        72.0, 0.0, 76.0, 79.0, 81.0, 79.0, 76.0, 0.0, 72.0, 74.0, 76.0, 0.0, 74.0, 72.0, 69.0, 0.0, //
        72.0, 0.0, 76.0, 79.0, 84.0, 81.0, 79.0, 0.0, 76.0, 79.0, 74.0, 0.0, 72.0, 0.0, 0.0, 0.0,
    ];
    for (i, &m) in melody.iter().enumerate() {
        if m == 0.0 {
            continue;
        }
        let f = note(m);
        b.add_wrapped(i as f32 * beat, beat * 1.5, |t| {
            let pluck = (TAU * f * t).sin() + 0.3 * (TAU * 2.0 * f * t).sin() + 0.1 * (TAU * 3.0 * f * t).sin();
            pluck * env(t, 0.005, 0.25) * 0.35
        });
    }
    b.normalize(0.55);
    b
}

// ------------------------------------------------------------ Snake ----

/// Croque : petit claquement puis note montante.
fn eat() -> Buf {
    let mut b = Buf::new(0.14);
    let mut n = Noise(31);
    b.add(0.0, 0.02, |t| n.next() * env(t, 0.0005, 0.006) * 0.6);
    b.add(0.005, 0.13, |t| {
        let p = sweep_phase(t, 520.0, 1_040.0, 0.06);
        (p.sin() + 0.3 * (2.0 * p).sin()) * env(t, 0.002, 0.05)
    });
    b.normalize(0.7);
    b
}

/// Petit tic discret à chaque virage.
fn turn() -> Buf {
    let mut b = Buf::new(0.03);
    b.add(0.0, 0.03, |t| (TAU * 1_800.0 * t).sin() * env(t, 0.0005, 0.006));
    b.normalize(0.4);
    b
}

/// Arpège de départ.
fn start() -> Buf {
    let mut b = Buf::new(0.7);
    for (i, m) in [72.0, 76.0, 79.0, 84.0].iter().enumerate() {
        let f = note(*m);
        b.add(i as f32 * 0.07, 0.4, |t| ((TAU * f * t).sin() + 0.3 * (TAU * 2.0 * f * t).sin()) * env(t, 0.003, 0.1));
    }
    b.normalize(0.6);
    b
}

/// « Wah wah waaah » de la défaite.
fn game_over() -> Buf {
    let mut b = Buf::new(1.6);
    let notes = [(0.0, 67.0, 0.3), (0.32, 66.0, 0.3), (0.64, 65.0, 0.9)];
    for (start, m, dur) in notes {
        let f = note(m);
        let mut lp = LowPass::default();
        let mut phase = 0.0;
        b.add(start, dur, |t| {
            let vib = if dur > 0.5 { 1.0 + 0.012 * (TAU * 6.0 * t).sin() } else { 1.0 };
            phase += f * vib / SR;
            let wa = 500.0 + 1_400.0 * (t / dur * std::f32::consts::PI).sin();
            let x = 2.0 * (phase % 1.0) - 1.0;
            let gate = (t / 0.02).min(1.0) * ((dur - t) / 0.08).clamp(0.0, 1.0);
            lp.run(x, wa) * gate
        });
    }
    b.normalize(0.7);
    b
}

/// « Toc » de bloc de bois posé.
fn tock() -> Buf {
    let mut b = Buf::new(0.2);
    let mut n = Noise(808);
    b.add(0.0, 0.01, |t| n.next() * env(t, 0.0003, 0.002) * 0.5);
    b.add(0.0, 0.2, |t| {
        let body = (TAU * 880.0 * t).sin() + 0.5 * (TAU * 1_870.0 * t).sin() * (-t / 0.015).exp();
        let thump = sweep_phase(t, 240.0, 120.0, 0.1).sin() * 0.6;
        (body + thump) * env(t, 0.001, 0.04)
    });
    b.normalize(0.75);
    b
}

/// Raclement d'une caisse qu'on pousse.
fn push() -> Buf {
    let mut b = Buf::new(0.22);
    let mut n = Noise(4_711);
    let mut lp = LowPass::default();
    b.add(0.0, 0.22, |t| {
        // Bruit grave haché : le bois qui accroche le sol.
        let grain = 0.6 + 0.4 * (TAU * 38.0 * t).sin().signum();
        let gate = (t / 0.02).min(1.0) * ((0.22 - t) / 0.06).clamp(0.0, 1.0);
        lp.run(n.next(), 700.0) * grain * gate * 3.0
    });
    b.normalize(0.6);
    b
}

/// Petite fanfare de victoire : arpège puis accord tenu.
fn win() -> Buf {
    let mut b = Buf::new(1.6);
    let notes = [72.0, 76.0, 79.0];
    for (i, m) in notes.iter().enumerate() {
        let f = note(*m);
        b.add(i as f32 * 0.11, 0.3, |t| ((3.0 * (TAU * f * t).sin()).tanh() * 0.6) * env(t, 0.004, 0.12));
    }
    for m in [72.0, 76.0, 79.0, 84.0] {
        let f = note(m);
        b.add(0.36, 1.2, |t| {
            let tone = (TAU * f * t).sin() + 0.4 * (TAU * 2.0 * f * t).sin() + 0.15 * (TAU * 3.0 * f * t).sin();
            tone * (t / 0.02).min(1.0) * (-t / 0.45).exp()
        });
    }
    b.normalize(0.65);
    b
}

// ---------------------------------------------------------- Espace ----

/// Tir de canon à ions : « piou » descendant, avec souffle.
fn laser() -> Buf {
    let mut b = Buf::new(0.45);
    let mut n = Noise(1_977);
    let mut lp = LowPass::default();
    b.add(0.0, 0.45, |t| {
        let p = sweep_phase(t, 1_800.0, 180.0, 0.45);
        let saw = 2.0 * ((p / TAU) % 1.0) - 1.0;
        let hiss = lp.run(n.next(), 3_000.0) * 0.4;
        (saw * 0.7 + p.sin() * 0.5 + hiss) * env(t, 0.003, 0.12)
    });
    b.normalize(0.65);
    b
}

/// « Bip » de rotation.
fn blip() -> Buf {
    let mut b = Buf::new(0.06);
    b.add(0.0, 0.06, |t| ((3.0 * (TAU * 1_320.0 * t).sin()).tanh()) * env(t, 0.002, 0.018));
    b.normalize(0.4);
    b
}

/// Verrouillage d'un bloc : « clac » métallique.
fn clunk() -> Buf {
    let mut b = Buf::new(0.25);
    let mut n = Noise(66);
    let mut lp = LowPass::default();
    b.add(0.0, 0.25, |t| {
        let x = n.next();
        let click = (x - lp.run(x, 2_500.0)) * env(t, 0.0005, 0.01);
        let ring = ((TAU * 640.0 * t).sin() + 0.6 * (TAU * 1_510.0 * t).sin()) * env(t, 0.001, 0.05) * 0.4;
        let thud = sweep_phase(t, 160.0, 70.0, 0.15).sin() * env(t, 0.001, 0.04) * 0.8;
        click + ring + thud
    });
    b.normalize(0.6);
    b
}

/// Ligne d'énergie complète : scintillement montant.
fn charge() -> Buf {
    let mut b = Buf::new(0.7);
    b.add(0.0, 0.7, |t| {
        let p = sweep_phase(t, 400.0, 1_600.0, 0.5);
        let shimmer = 0.6 + 0.4 * (TAU * 24.0 * t).sin();
        (p.sin() + 0.4 * (2.0 * p).sin()) * shimmer * (t / 0.05).min(1.0) * (-t / 0.25).exp()
    });
    b.normalize(0.6);
    b
}

/// Alarme : sirène à deux tons, deux fois.
fn alarm() -> Buf {
    let mut b = Buf::new(1.0);
    for k in 0..4 {
        let f = if k % 2 == 0 { 880.0 } else { 660.0 };
        b.add(k as f32 * 0.25, 0.24, |t| {
            let gate = (t / 0.01).min(1.0) * ((0.24 - t) / 0.02).clamp(0.0, 1.0);
            (2.5 * (TAU * f * t).sin()).tanh() * gate * 0.5
        });
    }
    b.normalize(0.55);
    b
}

/// Grosse explosion lointaine.
fn explosion() -> Buf {
    let mut b = Buf::new(2.2);
    let mut n = Noise(31_337);
    let mut lp = LowPass::default();
    b.add(0.0, 2.2, |t| {
        let cutoff = 150.0 + 2_500.0 * (-t / 0.25).exp();
        lp.run(n.next(), cutoff) * env(t, 0.005, 0.6) * 3.0
    });
    b.add(0.0, 0.8, |t| sweep_phase(t, 90.0, 30.0, 0.8).sin() * env(t, 0.003, 0.3));
    b.normalize(0.9);
    b
}

// ---------------------------------------------------------- 12 Parsecs ----

/// Rocher qui se brise : craquement grave et éclats.
fn rock_break() -> Buf {
    let mut b = Buf::new(0.5);
    let mut n = Noise(4_407);
    let mut lp = LowPass::default();
    b.add(0.0, 0.5, |t| {
        let cutoff = 300.0 + 1_800.0 * (-t / 0.06).exp();
        let crack = lp.run(n.next(), cutoff) * env(t, 0.001, 0.12) * 2.5;
        let thud = sweep_phase(t, 120.0, 50.0, 0.3).sin() * env(t, 0.002, 0.08);
        crack + thud
    });
    b.normalize(0.7);
    b
}

/// Bonus ramassé : petit arpège montant et brillant.
fn power_up() -> Buf {
    let mut b = Buf::new(0.5);
    for (k, m) in [72.0, 76.0, 79.0, 84.0].iter().enumerate() {
        let f = note(*m);
        b.add(k as f32 * 0.07, 0.25, |t| ((TAU * f * t).sin() + 0.3 * (TAU * 2.0 * f * t).sin()) * env(t, 0.003, 0.09));
    }
    b.normalize(0.55);
    b
}

/// Impact sur le bouclier : « vloum » électrique.
fn shield_hit() -> Buf {
    let mut b = Buf::new(0.35);
    let mut n = Noise(812);
    b.add(0.0, 0.35, |t| {
        let p = sweep_phase(t, 520.0, 140.0, 0.35);
        let buzz = (4.0 * p.sin()).tanh() * (0.6 + 0.4 * (TAU * 55.0 * t).sin());
        (buzz * 0.8 + n.next() * 0.25) * env(t, 0.002, 0.1)
    });
    b.normalize(0.6);
    b
}

/// « Blabla » électronique de Boulon : quelques bips qui changent de ton.
fn chatter() -> Buf {
    let mut b = Buf::new(0.42);
    for (k, f) in [880.0, 1_175.0, 990.0, 1_320.0, 1_050.0].iter().enumerate() {
        let f = *f;
        b.add(k as f32 * 0.075, 0.06, |t| (2.0 * (TAU * f * t).sin()).tanh() * env(t, 0.002, 0.03));
    }
    b.normalize(0.35);
    b
}

/// Musique de la traversée : synthé en dents de scie, mi mineur, 132 BPM.
fn parsecs_music() -> Buf {
    theme(&Theme {
        bpm: 132.0,
        chords: [[52.0, 55.0, 59.0], [48.0, 52.0, 55.0], [55.0, 59.0, 62.0], [50.0, 54.0, 57.0]],
        bass: &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5],
        pad: true,
        lead: Lead::Saw,
        melody: [
            76.0, 0.0, 79.0, 0.0, 83.0, 0.0, 81.0, 79.0, 76.0, 0.0, 0.0, 74.0, 72.0, 0.0, 74.0, 0.0, //
            79.0, 0.0, 81.0, 83.0, 86.0, 0.0, 83.0, 0.0, 81.0, 79.0, 78.0, 0.0, 76.0, 0.0, 0.0, 0.0,
        ],
        drums: Drums::HiHat,
        waves: false,
    })
}

#[derive(Clone, Copy)]
enum Lead {
    Pluck,
    Bell,
    Oud,
    Marimba,
    Square,
    /// Synthé en dents de scie (ambiance spatiale).
    Saw,
    /// Orgue : jeux tenus, harmoniques nombreuses, attaque lente.
    Organ,
}

impl Lead {
    fn sample(self, f: f32, t: f32) -> f32 {
        let s = |k: f32| (TAU * k * f * t).sin();
        match self {
            Lead::Pluck => (s(1.0) + 0.3 * s(2.0) + 0.1 * s(3.0)) * env(t, 0.005, 0.25),
            Lead::Bell => (s(1.0) + 0.5 * s(2.76) + 0.25 * s(5.4)) * env(t, 0.002, 0.6),
            Lead::Oud => (1..=6).map(|k| s(k as f32) / k as f32 * (-t * (3.0 + 2.0 * k as f32)).exp()).sum::<f32>() * env(t, 0.003, 10.0),
            Lead::Marimba => s(1.0) * env(t, 0.002, 0.25) + 0.3 * s(4.0) * env(t, 0.001, 0.04),
            Lead::Square => (3.0 * s(1.0)).tanh() * env(t, 0.01, 0.3) * 0.6,
            Lead::Saw => (1..=8).map(|k| s(k as f32) / k as f32).sum::<f32>() * env(t, 0.015, 0.45) * 0.5,
            Lead::Organ => [(1.0, 1.0), (2.0, 0.7), (3.0, 0.4), (4.0, 0.35), (6.0, 0.2), (8.0, 0.15)]
                .iter()
                .map(|&(k, a)| s(k) * a)
                .sum::<f32>()
                * (t / 0.08).min(1.0)
                * (-t / 2.5).exp()
                * 0.35,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Drums {
    None,
    HiHat,
    Darbuka,
    Toms,
}

/// Une musique en boucle de 8 mesures : 4 accords de 2 mesures, une mélodie
/// d'une note par temps (0 = silence), basse, percussions, ambiance.
struct Theme {
    bpm: f32,
    chords: [[f32; 3]; 4],
    /// Temps de la mesure où joue la basse.
    bass: &'static [f32],
    /// Nappes tenues (sinon accords en croches).
    pad: bool,
    lead: Lead,
    melody: [f32; 32],
    drums: Drums,
    /// Bruit de vagues.
    waves: bool,
}

fn theme(th: &Theme) -> Buf {
    let beat = 60.0 / th.bpm;
    let bar = beat * 4.0;
    let mut b = Buf::new(bar * 8.0);
    for bar_i in 0..8 {
        let chord = th.chords[bar_i / 2];
        let t0 = bar_i as f32 * bar;
        for &k in th.bass {
            let f = note(chord[0] - 12.0);
            b.add_wrapped(t0 + k * beat, beat * 1.5, |t| {
                let tri = 2.0 * (2.0 * ((f * t) % 1.0) - 1.0).abs() - 1.0;
                tri * env(t, 0.01, 0.4) * 0.45
            });
        }
        if th.pad {
            if bar_i % 2 == 0 {
                let dur = bar * 2.0;
                for m in chord {
                    let f = note(m);
                    b.add_wrapped(t0, dur, |t| {
                        let gate = (t / 0.4).min(1.0) * ((dur - t) / 0.4).clamp(0.0, 1.0);
                        ((TAU * f * t).sin() + 0.3 * (TAU * 2.0 * f * t).sin()) * gate * 0.07
                    });
                }
            }
        } else {
            for e in 0..8 {
                for m in chord {
                    let f = note(m);
                    b.add_wrapped(t0 + e as f32 * beat / 2.0, beat * 0.5, |t| (TAU * f * t).sin() * env(t, 0.005, 0.09) * 0.08);
                }
            }
        }
        let mut n = Noise(500 + bar_i as u32);
        let mut lp = LowPass::default();
        let mut hit = |b: &mut Buf, at: f32, kind: u8| {
            let start = t0 + at * beat;
            match kind {
                // charleston / « tek » : bruit aigu très court
                0 => b.add_wrapped(start, 0.06, |t| {
                    let x = n.next();
                    (x - lp.run(x, 6_000.0)) * env(t, 0.001, 0.015) * 0.2
                }),
                // « doum » / grosse caisse : sinus qui descend
                1 => b.add_wrapped(start, 0.25, |t| sweep_phase(t, 130.0, 50.0, 0.25).sin() * env(t, 0.002, 0.09) * 0.7),
                // caisse claire : bruit moyen
                _ => b.add_wrapped(start, 0.15, |t| {
                    let x = n.next();
                    (x - lp.run(x, 1_500.0)) * env(t, 0.001, 0.05) * 0.35
                }),
            }
        };
        match th.drums {
            Drums::None => {}
            Drums::HiHat => (0..4).for_each(|e| hit(&mut b, e as f32 + 0.5, 0)),
            Drums::Darbuka => {
                hit(&mut b, 0.0, 1);
                hit(&mut b, 2.5, 1);
                for at in [1.0, 1.5, 3.0, 3.5] {
                    hit(&mut b, at, 0);
                }
            }
            Drums::Toms => {
                for e in 0..4 {
                    hit(&mut b, e as f32, 1);
                    hit(&mut b, e as f32 + 0.5, 0);
                }
                hit(&mut b, 1.0, 2);
                hit(&mut b, 3.0, 2);
            }
        }
    }
    for (i, &m) in th.melody.iter().enumerate() {
        if m > 0.0 {
            let f = note(m);
            b.add_wrapped(i as f32 * beat, beat * 2.5, |t| th.lead.sample(f, t) * 0.35);
        }
    }
    if th.waves {
        // Houle : bruit filtré qui monte et descend, 4 vagues par boucle.
        let len = b.len_s();
        let period = len / 4.0;
        let mut n = Noise(2024);
        let mut lp = LowPass::default();
        b.add(0.0, len, |t| {
            let swell = (std::f32::consts::PI * (t % period) / period).sin().powi(2);
            lp.run(n.next(), 300.0 + 900.0 * swell) * swell * 0.9
        });
    }
    b.normalize(0.55);
    b
}

fn snake_musics() -> [(&'static str, Buf); 5] {
    [
        (
            "music/prairie",
            theme(&Theme {
                bpm: 120.0,
                chords: [[55.0, 59.0, 62.0], [52.0, 55.0, 59.0], [48.0, 52.0, 55.0], [50.0, 54.0, 57.0]],
                bass: &[0.0, 2.0],
                pad: false,
                lead: Lead::Pluck,
                melody: [
                    79.0, 0.0, 81.0, 83.0, 86.0, 83.0, 81.0, 0.0, 79.0, 81.0, 83.0, 0.0, 81.0, 79.0, 76.0, 0.0, //
                    79.0, 0.0, 83.0, 86.0, 88.0, 86.0, 83.0, 0.0, 81.0, 83.0, 79.0, 0.0, 79.0, 0.0, 0.0, 0.0,
                ],
                drums: Drums::HiHat,
                waves: false,
            }),
        ),
        (
            "music/desert",
            theme(&Theme {
                bpm: 96.0,
                chords: [[50.0, 57.0, 62.0], [46.0, 53.0, 58.0], [48.0, 55.0, 60.0], [50.0, 57.0, 62.0]],
                bass: &[0.0, 2.5],
                pad: true,
                lead: Lead::Oud,
                melody: [
                    74.0, 72.0, 70.0, 69.0, 70.0, 69.0, 67.0, 66.0, 67.0, 66.0, 63.0, 62.0, 0.0, 63.0, 66.0, 0.0, //
                    62.0, 66.0, 67.0, 69.0, 70.0, 0.0, 69.0, 67.0, 66.0, 67.0, 63.0, 0.0, 62.0, 0.0, 0.0, 0.0,
                ],
                drums: Drums::Darbuka,
                waves: false,
            }),
        ),
        (
            "music/banquise",
            theme(&Theme {
                bpm: 72.0,
                chords: [[53.0, 57.0, 64.0], [52.0, 55.0, 62.0], [50.0, 53.0, 60.0], [48.0, 52.0, 59.0]],
                bass: &[0.0],
                pad: true,
                lead: Lead::Bell,
                melody: [
                    84.0, 0.0, 0.0, 88.0, 0.0, 0.0, 86.0, 0.0, 83.0, 0.0, 0.0, 0.0, 81.0, 0.0, 79.0, 0.0, //
                    84.0, 0.0, 0.0, 91.0, 0.0, 88.0, 0.0, 0.0, 86.0, 0.0, 84.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ],
                drums: Drums::None,
                waves: false,
            }),
        ),
        (
            "music/ocean",
            theme(&Theme {
                bpm: 88.0,
                chords: [[48.0, 52.0, 55.0], [47.0, 50.0, 55.0], [45.0, 48.0, 52.0], [41.0, 45.0, 48.0]],
                bass: &[0.0, 2.0],
                pad: true,
                lead: Lead::Marimba,
                melody: [
                    72.0, 76.0, 79.0, 76.0, 74.0, 0.0, 71.0, 0.0, 72.0, 76.0, 81.0, 79.0, 77.0, 0.0, 0.0, 0.0, //
                    72.0, 76.0, 79.0, 84.0, 83.0, 0.0, 79.0, 0.0, 77.0, 76.0, 74.0, 72.0, 0.0, 0.0, 0.0, 0.0,
                ],
                drums: Drums::None,
                waves: true,
            }),
        ),
        (
            "music/volcan",
            theme(&Theme {
                bpm: 132.0,
                chords: [[57.0, 60.0, 64.0], [53.0, 57.0, 60.0], [55.0, 59.0, 62.0], [52.0, 56.0, 59.0]],
                bass: &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5],
                pad: false,
                lead: Lead::Square,
                melody: [
                    69.0, 72.0, 76.0, 72.0, 77.0, 76.0, 72.0, 0.0, 67.0, 71.0, 74.0, 71.0, 76.0, 0.0, 68.0, 0.0, //
                    69.0, 72.0, 76.0, 81.0, 79.0, 77.0, 76.0, 72.0, 74.0, 72.0, 71.0, 68.0, 69.0, 0.0, 0.0, 0.0,
                ],
                drums: Drums::Toms,
                waves: false,
            }),
        ),
    ]
}

/// Musique calme pour les jeux de concentration (Tour de blocs).
fn zen_music() -> Buf {
    theme(&Theme {
        bpm: 84.0,
        chords: [[50.0, 54.0, 57.0], [47.0, 50.0, 54.0], [43.0, 47.0, 50.0], [45.0, 49.0, 52.0]],
        bass: &[0.0, 2.0],
        pad: true,
        lead: Lead::Marimba,
        melody: [
            74.0, 0.0, 78.0, 81.0, 0.0, 78.0, 76.0, 0.0, 74.0, 0.0, 71.0, 0.0, 69.0, 0.0, 0.0, 0.0, //
            74.0, 0.0, 78.0, 83.0, 0.0, 81.0, 78.0, 0.0, 76.0, 0.0, 78.0, 76.0, 74.0, 0.0, 0.0, 0.0,
        ],
        drums: Drums::None,
        waves: false,
    })
}

/// Musique épique de space opera (NOVA) : la mineur, basse en croches, toms.
fn nova_music() -> Buf {
    theme(&Theme {
        bpm: 124.0,
        chords: [[57.0, 60.0, 64.0], [53.0, 57.0, 60.0], [48.0, 52.0, 55.0], [55.0, 59.0, 62.0]],
        bass: &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5],
        pad: true,
        lead: Lead::Saw,
        melody: [
            69.0, 0.0, 0.0, 72.0, 76.0, 0.0, 74.0, 72.0, 77.0, 0.0, 76.0, 0.0, 72.0, 0.0, 0.0, 0.0, //
            72.0, 0.0, 74.0, 76.0, 79.0, 0.0, 77.0, 76.0, 74.0, 0.0, 71.0, 0.0, 69.0, 0.0, 0.0, 0.0,
        ],
        drums: Drums::Toms,
        waves: false,
    })
}

/// Nocturne pour L'Observatoire : boîte à musique sur nappes, fa majeur, 72 BPM.
fn nocturne_music() -> Buf {
    theme(&Theme {
        bpm: 72.0,
        chords: [[53.0, 57.0, 60.0], [50.0, 53.0, 57.0], [46.0, 50.0, 53.0], [48.0, 52.0, 55.0]],
        bass: &[0.0, 2.0],
        pad: true,
        lead: Lead::Bell,
        melody: [
            77.0, 0.0, 81.0, 0.0, 84.0, 0.0, 81.0, 79.0, 77.0, 0.0, 74.0, 0.0, 72.0, 0.0, 0.0, 0.0, //
            74.0, 0.0, 77.0, 0.0, 82.0, 0.0, 81.0, 0.0, 79.0, 0.0, 76.0, 79.0, 77.0, 0.0, 0.0, 0.0,
        ],
        drums: Drums::None,
        waves: false,
    })
}

// ---------------------------------------------------------- Église ----

/// Sceau de cire posé : « tchac » sourd et un peu claquant.
fn stamp() -> Buf {
    let mut b = Buf::new(0.3);
    let mut n = Noise(2_718);
    let mut lp = LowPass::default();
    b.add(0.0, 0.08, |t| lp.run(n.next(), 1_800.0) * env(t, 0.0005, 0.02) * 2.0);
    b.add(0.0, 0.25, |t| sweep_phase(t, 180.0, 60.0, 0.2).sin() * env(t, 0.001, 0.05));
    b.normalize(0.7);
    b
}

/// Plume qui gratte le parchemin : deux traits rapides (une croix).
fn quill() -> Buf {
    let mut b = Buf::new(0.32);
    let mut n = Noise(1_234);
    let mut lp = LowPass::default();
    for start in [0.0, 0.15] {
        b.add(start, 0.13, |t| {
            let x = n.next();
            let hiss = x - lp.run(x, 2_500.0);
            let scritch = 0.5 + 0.5 * (TAU * 90.0 * t).sin().abs();
            hiss * scritch * (t / 0.01).min(1.0) * ((0.13 - t) / 0.03).clamp(0.0, 1.0)
        });
    }
    b.normalize(0.45);
    b
}

/// Carillon de cloches d'église : partiels inharmoniques, longue résonance.
fn church_bells() -> Buf {
    let mut b = Buf::new(5.0);
    // Harmoniques d'une cloche : bourdon, fondamentale, tierce mineure, quinte, nominal…
    let partials = [(0.5, 0.5, 2.2), (1.0, 1.0, 1.6), (1.19, 0.6, 1.2), (1.5, 0.4, 1.0), (2.0, 0.5, 0.8), (2.67, 0.25, 0.5), (3.0, 0.2, 0.4)];
    for (i, m) in [67.0, 64.0, 60.0, 67.0, 55.0].iter().enumerate() {
        let f = note(*m);
        b.add(i as f32 * 0.55, 4.4, |t| {
            partials.iter().map(|&(k, a, d)| (TAU * f * k * t).sin() * a * (-t / d).exp()).sum::<f32>() * (t / 0.003).min(1.0)
        });
    }
    b.normalize(0.7);
    b
}

/// Musique d'orgue, lente et solennelle (ré mineur modal), en boucle.
fn organ_music() -> Buf {
    theme(&Theme {
        bpm: 66.0,
        chords: [[50.0, 53.0, 57.0], [48.0, 52.0, 55.0], [46.0, 50.0, 53.0], [45.0, 49.0, 52.0]],
        bass: &[0.0],
        pad: true,
        lead: Lead::Organ,
        melody: [
            74.0, 0.0, 72.0, 0.0, 69.0, 0.0, 0.0, 0.0, 72.0, 0.0, 70.0, 0.0, 67.0, 0.0, 0.0, 0.0, //
            69.0, 0.0, 70.0, 72.0, 74.0, 0.0, 0.0, 0.0, 73.0, 0.0, 0.0, 0.0, 74.0, 0.0, 0.0, 0.0,
        ],
        drums: Drums::None,
        waves: false,
    })
}

// ------------------------------------------------------ Le berger ----

/// Aboiement : deux « ouaf » secs et graves.
fn bark() -> Buf {
    let mut b = Buf::new(0.5);
    let mut n = Noise(4_242);
    let mut lp = LowPass::default();
    for (i, start) in [0.0f32, 0.2].into_iter().enumerate() {
        let f0 = if i == 0 { 420.0 } else { 380.0 };
        b.add(start, 0.17, |t| {
            let p = sweep_phase(t, f0, f0 * 0.55, 0.17);
            let voix = p.sin() + 0.6 * (2.0 * p).sin() + 0.35 * (3.0 * p).sin();
            let souffle = lp.run(n.next(), 2_400.0) * 0.5;
            (voix * 0.6 + souffle) * env(t, 0.006, 0.05)
        });
    }
    b.normalize(0.75);
    b
}

/// Bêlement : « mêêê » nasillard, un peu tremblé.
fn bleat() -> Buf {
    let mut b = Buf::new(0.8);
    let mut lp = LowPass::default();
    let mut phase = 0.0;
    b.add(0.0, 0.8, |t| {
        let f = 330.0 - 40.0 * t + 14.0 * (TAU * 28.0 * t).sin();
        phase += f / SR;
        let dent = 2.0 * (phase % 1.0) - 1.0;
        let forme = (t / 0.06).min(1.0) * ((0.8 - t) / 0.25).clamp(0.0, 1.0);
        lp.run(dent, 1_300.0 + 500.0 * (TAU * 3.0 * t).sin()) * forme * (0.75 + 0.25 * (TAU * 28.0 * t).sin())
    });
    b.normalize(0.55);
    b
}

/// Hurlement de loup : un long « ouuuh » qui monte puis retombe.
fn howl() -> Buf {
    let mut b = Buf::new(2.2);
    let mut lp = LowPass::default();
    let mut phase = 0.0;
    b.add(0.0, 2.2, |t| {
        let x = t / 2.2;
        let hauteur = 300.0 + 280.0 * (x * 3.0).min(1.0) - 200.0 * ((x - 0.55) / 0.45).clamp(0.0, 1.0);
        let f = hauteur + 9.0 * (TAU * 5.5 * t).sin() * (x * 4.0).min(1.0);
        phase += f / SR;
        let voix = (TAU * phase).sin() + 0.45 * (TAU * 2.0 * phase).sin() + 0.2 * (TAU * 3.0 * phase).sin();
        let forme = (t / 0.25).min(1.0) * ((2.2 - t) / 0.7).clamp(0.0, 1.0);
        lp.run(voix, 2_200.0) * forme
    });
    b.normalize(0.6);
    b
}

fn main() -> std::io::Result<()> {
    // Racine du projet (par défaut : le dossier courant).
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let dir = Path::new(&root).join("assets/sounds");

    // (catégorie/nom, description, son) — les noms doivent correspondre à
    // `game_common::sounds`.
    let mut library: Vec<(&str, &str, Buf)> = vec![
        ("ui/click", "Clic de bouton", click()),
        ("ui/start", "Arpège montant de début de partie", start()),
        ("ui/chime", "Carillon de récompense (palier atteint)", chime()),
        ("ui/gameover", "« Wah wah waaah » de défaite", game_over()),
        ("ui/win", "Petite fanfare de victoire (niveau réussi)", win()),
        ("sfx/hop", "Petit « bloup » de saut", hop()),
        ("sfx/croak", "Coassement de crapaud", croak()),
        ("sfx/splash", "Plouf dans l'eau", splash()),
        ("sfx/crash", "Choc / écrasement", crash()),
        ("sfx/honk", "Klaxon de voiture (long)", honk(415.0, 523.0, 0.32, 1)),
        ("sfx/honk2", "Double coup de klaxon (court, aigu)", honk(554.0, 698.0, 0.12, 2)),
        ("sfx/bell", "Sonnerie de passage à niveau (en boucle)", bell()),
        ("sfx/train", "Klaxon et grondement de train", train()),
        ("sfx/eat", "Croque / ramasser un objet", eat()),
        ("sfx/turn", "Petit tic discret (virage, déplacement)", turn()),
        ("sfx/tock", "« Toc » de bloc de bois posé", tock()),
        ("sfx/push", "Raclement d'une caisse poussée", push()),
        ("sfx/laser", "Tir de canon à ions (« piou » descendant)", laser()),
        ("sfx/blip", "« Bip » électronique court (rotation)", blip()),
        ("sfx/clunk", "« Clac » métallique (bloc verrouillé)", clunk()),
        ("sfx/charge", "Scintillement montant (énergie chargée)", charge()),
        ("sfx/alarm", "Alarme à deux tons", alarm()),
        ("sfx/explosion", "Grosse explosion lointaine", explosion()),
        ("music/nova", "Musique épique de space opera, la mineur, 124 BPM", nova_music()),
        ("sfx/rock", "Rocher qui se brise (craquement et éclats)", rock_break()),
        ("sfx/powerup", "Bonus ramassé : arpège montant", power_up()),
        ("sfx/shield", "Impact sur le bouclier", shield_hit()),
        ("sfx/chatter", "« Blabla » électronique d'un petit robot", chatter()),
        ("sfx/bark", "Aboiement : deux « ouaf » secs", bark()),
        ("sfx/bleat", "Bêlement d'un mouton", bleat()),
        ("sfx/howl", "Hurlement de loup", howl()),
        ("music/parsecs", "Musique de traversée spatiale, mi mineur, 132 BPM", parsecs_music()),
        ("sfx/stamp", "Sceau de cire posé (« tchac »)", stamp()),
        ("sfx/quill", "Plume qui gratte le parchemin (deux traits)", quill()),
        ("sfx/churchbells", "Carillon de cloches d'église", church_bells()),
        ("music/orgue", "Musique d'orgue lente et solennelle, ré mineur, 66 BPM", organ_music()),
        ("music/zen", "Musique zen (marimba, nappes), ré majeur, 84 BPM", zen_music()),
        ("music/nocturne", "Nocturne en boîte à musique, fa majeur, 72 BPM", nocturne_music()),
        ("music/campagne", "Musique guillerette, do majeur, 112 BPM", music()),
    ];
    let descriptions = [
        "Musique joyeuse, sol majeur, 120 BPM",
        "Musique orientale (oud, darbouka), 96 BPM",
        "Musique lente et cristalline (cloches), 72 BPM",
        "Musique calme (marimba) avec vagues, 88 BPM",
        "Musique rapide et tendue (la mineur), 132 BPM",
    ];
    for ((name, buf), desc) in snake_musics().into_iter().zip(descriptions) {
        library.push((name, desc, buf));
    }

    let mut catalogue = String::from(
        "# Bibliothèque de sons Commit & Conquer\n\n\
         Fichier généré par `cargo run -p sfx-gen` : ne pas modifier à la main.\n\
         Tous les sons sont fabriqués par synthèse (licence GPL-3.0-or-later, comme le code).\n\
         Dans le code, utilisez les constantes de `game_common::sounds`.\n\n\
         | Fichier | Durée | Description |\n|---|---|---|\n",
    );
    for (name, desc, buf) in &library {
        let path = dir.join(format!("{name}.wav"));
        std::fs::create_dir_all(path.parent().unwrap())?;
        buf.write_wav(&path)?;
        println!("{:<16} {:>5.2} s  → {}", name, buf.len_s(), path.display());
        catalogue += &format!("| `{name}.wav` | {:.2} s | {desc} |\n", buf.len_s());
    }
    std::fs::write(dir.join("CATALOGUE.md"), catalogue)
}
