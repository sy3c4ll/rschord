use rschord::note::*;

fn main() {
    let d = dbg!(Diatonic::B);
    let a = dbg!(Accidental::DOUBLE_SHARP);
    let pc = dbg!(PitchClass::new(d, a));
    let c = dbg!(pc.chromatic());
    println!("[#] {d} + {a} => {pc} == {c}");
    let o = dbg!(Octave::new(4));
    let p = dbg!(Pitch::new(pc, o));
    let mn = dbg!(p.to_midi_note());
    let t = dbg!(mn.to_tone());
    println!("[#] {pc} + {o} => {p} == {t} == {mn}");
}
