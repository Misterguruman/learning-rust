fn main() {
    let _a: u16 = 1_000;
    let _b: i16 = -126;

    let _c: i32 = 1_000_000_000;
    let _d: f32 = 100.0;

    let _e: char = 'd';
    let _f: &str = "String";

    let g: (u16, u16) = (2, 4);

    let h: [i32; 4] = [1, 2, 3, 4];

    let _direct_tuple_index = g.1;

    let (_un, _pack) = g;

    let _direct_index: i32 = h[1];

}
