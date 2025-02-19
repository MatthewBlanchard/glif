pub const _KNIFE: &[u8] = &[239, 128, 128, 0]; // U+F000
pub const _MEASURE: &[u8] = &[239, 128, 129, 0]; // U+F001
pub const _PAN: &[u8] = &[239, 128, 130, 0]; // U+F002
pub const _PEN: &[u8] = &[239, 128, 131, 0]; // U+F003
pub const _PENCIL: &[u8] = &[239, 128, 132, 0]; // U+F004
pub const _SELECT: &[u8] = &[239, 128, 133, 0]; // U+F005
pub const _SHAPES: &[u8] = &[239, 128, 134, 0]; // U+F006
pub const _TEXT: &[u8] = &[239, 128, 135, 0]; // U+F007
pub const _ZOOM: &[u8] = &[239, 128, 136, 0]; // U+F008
pub const _VWS: &[u8] = &[239, 128, 137, 0]; // U+F009
pub const _ANCHOR: &[u8] = &[239, 128, 138, 0]; // U+F00A
pub const _PAP: &[u8] = &[239, 128, 139, 0]; // U+F00B
pub const _OPENEYE: &[u8] = &[239, 128, 140, 0]; // U+F00C
pub const _CLOSEDEYE: &[u8] = &[239, 128, 141, 0]; // U+F00D
pub const _MINUS: &[u8] = &[239, 128, 142, 0]; // U+F00E
pub const _PLUS: &[u8] = &[239, 128, 143, 0]; // U+F00F
pub const _ARROWLEFT: &[u8] = &[239, 128, 144, 0]; // U+F010
pub const _ARROWDOWN: &[u8] = &[239, 128, 145, 0]; // U+F011
pub const _ARROWUP: &[u8] = &[239, 128, 146, 0]; // U+F012
pub const _ARROWRIGHT: &[u8] = &[239, 128, 148, 0]; // U+F014
pub const _RENAME: &[u8] = &[239, 128, 149, 0]; // U+F015
pub const _LAYERUNION: &[u8] = &[239, 128, 150, 0]; // U+F016
pub const _LAYERCOMBINE: &[u8] = &[239, 128, 151, 0]; // U+F017
pub const _LAYERDIFFERENCE: &[u8] = &[239, 128, 152, 0]; // U+F018
pub const _LAYERINTERSECTION: &[u8] = &[239, 128, 153, 0]; // U+F019
pub const _LAYERXOR: &[u8] = &[239, 128, 154, 0]; // U+F01A
pub const _IMAGES: &[u8] = &[239, 128, 156, 0]; // U+F01C
pub const _GRID: &[u8] = &[239, 128, 169, 0]; // U+F029
pub const _GUIDELINES: &[u8] = &[239, 128, 176, 0]; // U+F030
pub const _DASH: &[u8] = &[239, 128, 181, 0]; // U+F035
pub const _GLOBE: &[u8] = &[239, 128, 182, 0]; // U+F036
pub const _UFO: &[u8] = &[239, 128, 183, 0]; // U+F037

pub fn chain(icons: &[&[u8]]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(icons.len() * 4);
    for (i, icon) in icons.into_iter().enumerate() {
        ret.extend(*icon);
        if i != icons.len() - 1 {
            assert_eq!(ret.pop(), Some(0));
        }
    }
    ret
}
