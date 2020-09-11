// const vec4 bitEnc = vec4(1.,255.,65025.,16581375.);
// const vec4 bitDec = 1./bitEnc;
// vec4 EncodeFloatRGBA (float v) {
// 	vec4 enc = bitEnc * v;
// 	enc = fract(enc);
// 	enc -= enc.yzww * vec2(1./255., 0.).xxxy;
// 	return enc;
// }

const bitEnc: [f32; 4] = [1., 255., 65025., 16581375.];
const bitDec: [f32; 4] = [1., 1.0 / 255., 1.0 / 65025., 1.0 / 16581375.];

pub fn float2rgba(v: f32) -> [f32; 4] {
    // 	vec4 enc = bitEnc * v;
    let mut enc = [bitEnc[0] * v, bitEnc[1] * v, bitEnc[2] * v, bitEnc[3] * v];
    // 	enc = fract(enc);
    for en in enc.iter_mut() {
        *en = *en - f32::floor(*en);
    }
    // println!("{:#?}", enc);
    // enc -= enc.yzww * vec2(1./255., 0.).xxxy;
    let yzww = [enc[1], enc[2], enc[3], enc[3]];
    let vec2 = [1.0 / 255., 0.0];
    let xxxy = [vec2[0], vec2[0], vec2[0], vec2[1]];
    let ret = [
        yzww[0] * xxxy[0],
        yzww[1] * xxxy[1],
        yzww[2] * xxxy[2],
        yzww[3] * xxxy[3],
    ];
    [
        enc[0] - ret[0],
        enc[1] - ret[1],
        enc[2] - ret[2],
        enc[3] - ret[3],
    ]
}
// float DecodeFloatRGBA (vec4 v) {
// 	return dot(v, bitDec);
// }

pub fn rgba2float(v: [f32; 4]) -> f32 {
    bitDec[0] * v[0] + bitDec[1] * v[1] + bitDec[2] * v[2] + bitDec[3] * v[3]
}

fn main() {
    let ret = float2rgba(0.66666666);
    println!("{:#?}", ret);

    let ret = rgba2float(ret);
    println!("{:#?}", ret);
}
