use music_generator::*;
use rand::prelude::*;
use std::path::Path;

use NoteType::*;

fn generate(chords: Vec<TimedChord>, notes: Vec<NoteType>, bpm: f32) -> Song {   
    let mut rng = rand::thread_rng();
    let d = rand::distributions::Bernoulli::new(0.8).unwrap();
    // let scale = scale.notes(key).take(12).collect::<Vec<_>>();
    let scale = notes;
    let mut melody = vec![];
    let mut harmony = vec![];

    let close_range = 2;
    let large_range = 5;
    let mut prev_note = 0;
    let mut octave = 4;
    let mut hold_for = 1;
    for beat in 0..(chords.duration() as u32 * 8) {
        if hold_for > 0 {
            hold_for -= 1;
            melody.push(Note::new(scale[prev_note], octave).with_duration(0.125));
            continue;
        } else {
            hold_for = rng.gen_range(0..=2);
        }

        // Generate notes close to each other
        let note = if d.sample(&mut rng) {
            let i: i32 = rng.gen_range(-close_range..close_range);
            let new_note = (prev_note as i32 + i).max(0).min(scale.len() as i32-1) as usize;
            prev_note = new_note;
            scale[new_note]
        } else {
            let i: i32 = rng.gen_range(-large_range..large_range);
            let new_note = (prev_note as i32 + i).max(0).min(scale.len() as i32-1) as usize;
            prev_note = new_note;
            scale[new_note]
        };
    
        if beat % 8 == 0 && d.sample(&mut rng) {
            octave = (octave + 1).min(5);
        } else if beat % 8 == 0 {
            octave = (octave - 1).max(4);
        }

        melody.push(Note::new(note, octave).with_duration(0.125));
    }


    let close_range = 3;
    let large_range = 5;
    let mut prev_note: usize = 0;
    let mut octave = 3;
    let mut hold_for = 1;
    let d = rand::distributions::Bernoulli::new(0.3).unwrap();
    for beat in 0..(chords.duration() as u32 * 8) {
        if hold_for > 0 {
            hold_for -= 1;
            harmony.push(Note::new(scale[prev_note], octave).with_duration(0.125));
            continue;
        } else {
            hold_for = rng.gen_range(0..=2);
        }

        // Generate notes close to each other
        let note = if d.sample(&mut rng) {
            let i: i32 = rng.gen_range(-close_range..close_range);
            let new_note = (prev_note as i32 + i).max(0).min(scale.len() as i32 - 1) as usize;
            prev_note = new_note;
            scale[new_note]
        } else {
            let i: i32 = rng.gen_range(-large_range..large_range);
            let new_note = (prev_note as i32 + i).max(0).min(scale.len() as i32 - 1) as usize;
            prev_note = new_note;
            scale[new_note]
        };
    
        if beat % 8 == 0 && d.sample(&mut rng) {
            octave = (octave + 1).min(4);
        } else if beat % 8 == 0 {
            octave = (octave - 1).max(3);
        }

        harmony.push(Note::new(note, octave).with_duration(0.125));
    }

    Song::new(
        melody,
        chords,
        bpm,
        false
    ).with_harmony(harmony)
}

fn generate_major_blues(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let keys = vec![
        E, A, E, E,
        A, A, E, E,
        B, A, E, B,

        E, A, E, E,
        A, A, E, E,
        B, A, E, B,
    ];

    // Generate 4 bar blues progression
    let chords: Vec<TimedChord> = keys.iter().enumerate().map(|(i, key)| {
        let key = *key;
        match i % 12 {
            3 | 5 | 7 | 11 => Chord::major7(key, 4),
            _ => Chord::major(key, 4),
        }.with_duration(2.0) + (G.to_offset() - E.to_offset())
    }).collect();
            

    generate(chords, Scale::MajorBlues.notes(G).take(12).collect(), bpm)
}

fn generate_minor_blues(bpm: f32) -> Song {
    /*
    // Randomly sample the G pentatonic iterator
    let mut rng = rand::thread_rng();
    let keys = vec![
        E, A, E, E,
        A, A, E, E,
        B, A, E, B,

        E, A, E, E,
        A, A, E, E,
        B, A, E, B,
    ];
    let mut melody = vec![];

    // Generate 4 bar blues progression
    let chords: Vec<TimedChord> = keys.iter().enumerate().map(|(i, key)| {
        // Chord::minor(*key, 4).with_duration(2.0)
        let key = *key;
        match i % 12 {
            3 | 5 | 7 | 11 => Chord::minor7(key, 4),
            _ => Chord::minor(key, 4),
        }.with_duration(2.0)
    }).collect();
    

    let d = rand::distributions::Bernoulli::new(0.05).unwrap();
    for beat in 0..(chords.duration() as u32 * 8) {
        let key = keys[(beat as usize / 8) % keys.len()];
        // Randomly get a note from the scale
        let scale = Scale::MinorBlues.notes(E).take(12);

        let note = scale.choose(&mut rng).unwrap();

        melody.push(Note::new(note, 4).with_duration(0.125));
    }

    Song::new(
        melody,
        chords,
        bpm,
        false
    )
     */

    // Randomly sample the G pentatonic iterator
    let keys = vec![
        E, A, E, E,
        A, A, E, E,
        B, A, E, B,
    
        E, A, E, E,
        A, A, E, E,
        B, A, E, B,
    ];
    
    // Generate 4 bar blues progression
    let chords: Vec<TimedChord> = keys.iter().enumerate().map(|(i, key)| {
        let key = *key;
        match i % 12 {
            3 | 5 | 7 | 11 => Chord::minor7(key, 4),
            _ => Chord::minor(key, 4),
        }.with_duration(2.0) + (G.to_offset() - E.to_offset())
    }).collect();
            
    generate(chords, Scale::MinorBlues.notes(G).take(12).collect(), bpm)
}

fn generate_major_pentatonic(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let chords = vec![
        Chord::major(G, 4).with_duration(4.0),
        Chord::major(D, 4).with_duration(4.0),
        Chord::major(C, 4).with_duration(4.0),
        Chord::major(G, 4).with_duration(4.0),
        Chord::major(C, 4).with_duration(4.0),
        Chord::major(G, 4).with_duration(4.0),
        Chord::major(D, 4).with_duration(4.0),
        Chord::major(C, 4).with_duration(4.0),
    ];

    // let mut melody = vec![];
    
    // let scale = Scale::MajorPentatonic.notes(G).take(12).collect::<Vec<_>>();
    // for beat in 0..(chords.duration() as u32 * 8) {
    //     // Randomly get a note from the scale
    //     let note = scale.choose(&mut rng).unwrap();

    //     // melody.push(note.with_duration(0.125));
    //     melody.push(Note::new(*note, 4).with_duration(0.125));
    // }

    // Song::new(
    //     melody,
    //     chords,
    //     bpm,
    //     false
    // )

    generate(chords, Scale::MajorPentatonic.notes(G).take(12).collect(), bpm)

}

fn generate_minor_pentatonic(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let chords = vec![
        Chord::minor(G, 4).with_duration(4.0),
        Chord::minor(D, 4).with_duration(4.0),
        Chord::minor(C, 4).with_duration(4.0),
        Chord::minor(G, 4).with_duration(4.0),
        Chord::minor(C, 4).with_duration(4.0),
        Chord::minor(G, 4).with_duration(4.0),
        Chord::minor(D, 4).with_duration(4.0),
        Chord::minor(C, 4).with_duration(4.0),
    ];

    // let mut melody = vec![];
    
    // let scale = Scale::MinorPentatonic.notes(G).take(12).collect::<Vec<_>>();
    // for beat in 0..(chords.duration() as u32 * 8) {
    //     // Randomly get a note from the scale
    //     let note = scale.choose(&mut rng).unwrap();

    //     // melody.push(note.with_duration(0.125));
    //     melody.push(Note::new(*note, 4).with_duration(0.125));
    // }

    generate(chords, Scale::MinorPentatonic.notes(G).take(12).collect(), bpm)

    // Song::new(
    //     melody,
    //     chords,
    //     bpm,
    //     false
    // )
}

fn main() {
    println!("Hello, world!");

    // let bpm = 120.0;
    let bpm = 240.0;


    // The chords
    // let chords = vec![
    //     Chord::minor(G, 4).with_duration(8.0),
    //     Chord::minor(D, 4).with_duration(8.0),
    //     Chord::minor(C, 4).with_duration(8.0),
    //     Chord::minor(G, 4).with_duration(8.0),
    //     Chord::minor(C, 4).with_duration(8.0),
    //     Chord::minor(G, 4).with_duration(8.0),
    //     Chord::minor(D, 4).with_duration(8.0),
    //     Chord::minor(C, 4).with_duration(8.0),
    // ];


    // for note in Scale::MajorBlues.notes_in_octave(G, 4).take(12) {
    //     println!("{:?} = {}", note, note.frequency());
    //     melody.push(note.with_duration(1.0));
    // }
    // let sample_generator = Song::generate_frequency_samples_sawtooth_wave;


    // let sample_generator = Song::generate_frequency_samples_square_wave;
    let cross_fade = 0.013;

    let sample_generators: Vec<(Box<dyn Fn(&Song, f32, f32, u16) -> Vec<f32>>, &str)> = vec![
        (Box::new(Song::generate_frequency_samples_sine_wave), "sine"),
        (Box::new(Song::generate_frequency_samples_sawtooth_wave), "sawtooth"),
        (Box::new(Song::generate_frequency_samples_square_wave), "square"),
    ];

    let mb = generate_major_blues(bpm);
    let mp = generate_major_pentatonic(bpm);
    let mm = generate_minor_pentatonic(bpm);
    let mmb = generate_minor_blues(bpm);

    for (sample_generator, name) in sample_generators {
        mb.save(&Path::new(&format!("{}_major_blues.wav", name)), cross_fade, &*sample_generator).unwrap();
        mmb.save(&Path::new(&format!("{}_minor_blues.wav", name)), cross_fade, &*sample_generator).unwrap();
        mm.save(&Path::new(&format!("{}_major_pentatonic.wav", name)), cross_fade, &*sample_generator).unwrap();
        mp.save(&Path::new(&format!("{}_minor_pentatonic.wav", name)), cross_fade, &*sample_generator).unwrap();
    }

    // #[cfg(feature = "playback")]
    // generate_major_blues(bpm).save(&Path::new("major_blues.wav"), sample_generator).unwrap();
    // #[cfg(feature = "playback")]
    // generate_minor_blues(bpm).save(&Path::new("minor_blues.wav"), sample_generator).unwrap();
    // #[cfg(feature = "playback")]
    // generate_major_pentatonic(bpm).save(&Path::new("major.wav"), sample_generator).unwrap();
    // #[cfg(feature = "playback")]
    // generate_minor_pentatonic(bpm).save(&Path::new("minor.wav"), sample_generator).unwrap();

    // #[cfg(feature = "playback")]
    // loop {
    //     (generate_major_blues(bpm) + generate_major_pentatonic(bpm) + generate_minor_pentatonic(bpm) + generate_minor_blues(bpm)).play();
    // }
}