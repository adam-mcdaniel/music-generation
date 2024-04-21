use music_generator::*;
use rand::prelude::*;
use std::path::{Path, PathBuf};

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

/// Generate a song using the major pentatonic scale in the key of G!
fn generate_major_pentatonic_song(bpm: f32) -> Song {
    // For randomly sampling the G pentatonic iterator
    let mut rng = rand::thread_rng();

    // A chord progression in the key of G on the 4th octave
    let chords = vec![
        Chord::major(G, 4),
        Chord::major(D, 4),
        Chord::major(C, 4),
        Chord::major(G, 4),
        Chord::major(C, 4),
        Chord::major(G, 4),
        Chord::major(D, 4),
        Chord::major(C, 4),
    ];
    // Add a duration of 4 beats to each chord
    let chords: Vec<_> = chords.iter().map(|chord| chord.with_duration(4.0)).collect();

    // Our melody and harmony notes
    let mut melody = vec![];
    let mut harmony = vec![];

    // The scale of notes to choose from
    let scale = Scale::MajorPentatonic.notes(G).take(12).collect::<Vec<_>>();

    // Each melody note is 1/8th of a beat
    let note_duration = 0.125;
    // Generate a melody for each beat in the song
    for beat in 0..(chords.duration() as u32 * 8) {
        // Create a random melody note with 1/8th of a beat duration
        let melody_note_name = scale.choose(&mut rng).unwrap();
        // Add the note to the melody on the 4th octave and with a duration
        melody.push(Note::new(*melody_note_name, 4)
                        .with_duration(note_duration));

        // Create a random harmony note with 1/8th of a beat duration
        let harmony_note_name = scale.choose(&mut rng).unwrap();
        // Add the note to the harmony on the 3rd octave and with a duration
        harmony.push(Note::new(*harmony_note_name, 3)
                        .with_duration(note_duration));
    }

    // Create a song with the melody, harmony, chords, and bpm (and opt out of looping the song)
    let song = Song::new(melody, chords, bpm, false).with_harmony(harmony);

    // The cross fade duration (in seconds) between notes
    // This will fade in/fade out the notes to prevent popping
    let note_cross_fade = 0.01;
    // Save the song to a file
    song.save(
        &Path::new("major_pentatonic.wav"),
        note_cross_fade,
        // Generate the file using square wave samples (sounds like 8-bit chiptune)
        &Song::generate_frequency_samples_square_wave
    ).unwrap();

    // Play the song
    #[cfg(feature = "playback")]
    song.play();
    
    // Return the song
    return song
}


fn generate_major_pentatonic(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let chords = vec![
        Chord::major(G, 4),
        Chord::major(D, 4),
        Chord::major(C, 4),
        Chord::major(G, 4),
        Chord::major(C, 4),
        Chord::major(G, 4),
        Chord::major(D, 4),
        Chord::major(C, 4),
    ];
    let chords = chords.iter().map(|chord| chord.with_duration(4.0)).collect();

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
        Chord::minor(G, 4),
        Chord::minor(D, 4),
        Chord::minor(C, 4),
        Chord::minor(G, 4),
        Chord::minor(C, 4),
        Chord::minor(G, 4),
        Chord::minor(D, 4),
        Chord::minor(C, 4),
    ];
    let chords = chords.iter().map(|chord| chord.with_duration(4.0)).collect();

    generate(chords, Scale::MinorPentatonic.notes(G).take(12).collect(), bpm)
}

fn generate_diminished(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let chords = vec![
        Chord::major7(DSharp, 4),
        Chord::major7(DSharp, 4),
        Chord::major7(DSharp, 4),
        Chord::major7(DSharp, 4),
        Chord::half_diminished7(C, 4),
        Chord::minor7(F, 4),
        Chord::half_diminished7(B, 4),
        Chord::major7(DSharp, 4),
        Chord::minor7b5(D, 4),
        Chord::major7b9(G, 4),
        Chord::minor7(C, 4),
        Chord::major7b9(F, 4),
        Chord::minor7(ASharp, 4),
        Chord::major7b9(DSharp, 4),
        Chord::major7(GSharp, 4),
        Chord::major7sharp11(ASharp, 4),
        Chord::major7(DSharp, 4),
        Chord::major7(DSharp, 4),
    ];

    let chords = chords.iter().map(|chord| chord.with_duration(4.0)).collect();
    let scale = Scale::Diminished;
    generate(chords, scale.notes(DSharp).take(scale.note_count() * 5).collect(), bpm)
}

fn generate_augmented(bpm: f32) -> Song {
    // Randomly sample the G pentatonic iterator
    let chords = vec![
        Chord::augmented(G, 4),
        Chord::augmented(ASharp, 4),
        Chord::major7(DSharp, 4),
        Chord::augmented(D, 4),
        Chord::augmented(G, 4),
        Chord::augmented(ASharp, 4),
        Chord::major7(DSharp, 4),
        Chord::augmented(D, 4),
        Chord::augmented(G, 4),
        Chord::augmented(ASharp, 4),
        Chord::major7(DSharp, 4),
        Chord::augmented(D, 4),
    ];

    let chords = chords.iter().map(|chord| chord.with_duration(4.0)).collect();
    let scale = Scale::Augmented;
    generate(chords, scale.notes(G).take(scale.note_count() * 5).collect(), bpm)
}

fn main() {
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
    let cross_fade = 0.01;

    let sample_generators: Vec<(Box<dyn Fn(&Song, f32, f32, u16) -> Vec<f32>>, &str)> = vec![
        (Box::new(Song::generate_frequency_samples_sine_wave), "sine"),
        (Box::new(Song::generate_frequency_samples_square_wave), "square"),
        (Box::new(Song::generate_frequency_samples_sawtooth_wave), "sawtooth"),
    ];

    let songs = vec![
        (generate_augmented(bpm), "augmented"),
        (generate_diminished(bpm), "diminished"),
        (generate_major_blues(bpm), "major_blues"),
        (generate_minor_blues(bpm), "minor_blues"),
        (generate_major_pentatonic(bpm), "major_pentatonic"),
        (generate_minor_pentatonic(bpm), "minor_pentatonic"),
    ];

    for (sample_generator, wave_name) in sample_generators {
        for (song, song_name) in &songs {
            let song_name = format!("{}_{}", wave_name, song_name);
            let path = PathBuf::from(&format!("{}.wav", song_name));
            song.save(&path, cross_fade, &sample_generator).unwrap();
            println!("Saved {}", path.display())
        }
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
