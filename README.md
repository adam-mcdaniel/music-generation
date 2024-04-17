# music-generation

![Header Image](assets/header.png)

This repository implements a music generation library in Rust, with the goal of creating a simple and easy-to-use API for generating music. The library is designed to be flexible and extensible, allowing users to create their own custom music generation algorithms. Additionally, it also supports compilation for the web using WebAssembly, allowing users to generate music directly in the browser.

## Features

- Generate `.wav` files from custom music generation algorithms
- Easily manipulating scales, chords, melodies, and harmonies
- Built-in support for common scales and chords, including major, minor, pentatonic, blues, diminished, augmented, and more!
- Music playback using [`rodio`](https://crates.io/crates/rodio) (with the `playback` feature enabled)
- Supports directly sampling the generated audio data with custom wave functions, with builtin support for sine, square, and sawtooth waves
- WebAssembly support for generating music in the browser

## Music Examples

Here are some examples of the music generation library in action, some of them sound pretty cool! I particularly like the chip-tune sound of the square wave. The sawtooth is good for a more aggressive sound, and the sine wave is good for a more mellow sound.

The major and minor scales are the most common scales in Western music, and are great for creating melodies and harmonies. The pentatonic scales are also very common, and are great for creating simple and catchy songs. The blues scales are more distinctive and are great for creating a more soulful sound.

| Major | Examples | Minor | Examples |
|----------|----------|-------|----------|
| Major Blues Scale | ▹ Chiptune Major Blues <br><audio src="examples/square_major_blues.wav" controls></audio><br> ▹ Sawtooth Major Blues <br><audio src="examples/sawtooth_major_blues.wav" controls></audio><br> ▹ Sine Major Blues <br><audio src="examples/sine_major_blues.wav" controls></audio> | Minor Blues Scale | ▹ Chiptune Minor Blues <br><audio src="examples/square_minor_blues.wav" controls></audio><br> ▹ Sawtooth Minor Blues <br><audio src="examples/sawtooth_minor_blues.wav" controls></audio><br> ▹ Sine Minor Blues <br><audio src="examples/sine_minor_blues.wav" controls></audio> |
| Major Pentatonic Scale | ▹ Chiptune Major Pentatonic <br><audio src="examples/square_major_pentatonic.wav" controls></audio><br> ▹ Sawtooth Major Pentatonic <br><audio src="examples/sawtooth_major_pentatonic.wav" controls></audio><br> ▹ Sine Major Pentatonic <br><audio src="examples/sine_major_pentatonic.wav" controls></audio> | Minor Pentatonic Scale | ▹ Chiptune Minor Pentatonic <br><audio src="examples/square_minor_pentatonic.wav" controls></audio><br> ▹ Sawtooth Minor Pentatonic <br><audio src="examples/sawtooth_minor_pentatonic.wav" controls></audio><br> ▹ Sine Minor Pentatonic <br><audio src="examples/sine_minor_pentatonic.wav" controls></audio> |

These examples below are a bit more experimental in their sound due to the nature of the scales. The diminished scale is a bit more dissonant, while the augmented scale is a bit more exotic. They can be used to create some interesting and unique sounds, especially when combined with other scales and chords. They're great for creating a spooky or mysterious atmosphere.

| Diminished | Examples | Augmented | Examples |
|------------|----------|-----------|----------|
| Diminished Scale | ▹ Chiptune Diminished <br><audio src="examples/square_diminished.wav" controls></audio><br> ▹ Sawtooth Diminished <br><audio src="examples/sawtooth_diminished.wav" controls></audio><br> ▹ Sine Diminished <br><audio src="examples/sine_diminished.wav" controls></audio> | Augmented Scale | ▹ Chiptune Augmented <br><audio src="examples/square_augmented.wav" controls></audio><br> ▹ Sawtooth Augmented <br><audio src="examples/sawtooth_augmented.wav" controls></audio><br> ▹ Sine Augmented <br><audio src="examples/sine_augmented.wav" controls></audio> |

## Usage

To use the music generation library, add the following to your `Cargo.toml`:

```toml
[dependencies]
music-generation = { git="https://github.com/adam-mcdaniel/music-generation" }
```

For playback support, enable the `playback` feature:

```toml
[dependencies]
music-generation = { git="https://github.com/adam-mcdaniel/music-generation", features = ["playback"] }
```

For web assembly support, enable the `web` feature:

```toml
[dependencies]
music-generation = { git="https://github.com/adam-mcdaniel/music-generation", features = ["web"] }
```

This crate is a library, but it also includes a binary that randomly generates songs using all the scales mentioned above, even though the library itself supports much more. To run the binary, use the following command:

```bash
cargo run --release --features playback
```

This will generate several `.wav` files in the project directory that you can listen to. I used this to generate the examples above!

## Example Usage

Here's an example of how to generate a simple song using the music generation library:


<table>
<tr>
<th>
Example Major Pentatonic Song Generation
</th>
</tr>
<tr>
<td>

```rust
// Import the music generation library and its dependencies
use music_generator::*;
use NoteType::*;
// Import the random number generator
use rand::prelude::*;

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
    song.play();
    
    // Return the song
    return song
}
```
</td>
</table>

## Author Notes

This project was really fun to work on, I loved getting to experiment with new music theory concepts! A lot of these scales and chords were new to me, and they're more difficult to play on the guitar than they are to generate in code. I'm also very pleased with the results of the music generation, the songs sound pretty good!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.