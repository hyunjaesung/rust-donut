// 참고 자료 : https://www.a1k0n.net/2011/07/20/donut-math.html
use std::thread;
use std::time;

fn main() {
    let R1 = 1.0; // 도넛 단면의 반지름
    let R2 = 2.0; // 도넛 중심에서 도넛 단면 중심까지 거리

    let SCREEN_WIDTH = 500.0;
    let SCREEN_HEIGHT = 500.0;

    let PI = 3.14;
    // K 는 뷰어의 위치
    let K2 = 5.0; // 도넛에서 뷰어까지 거리
    let K1 = SCREEN_WIDTH * K2 * 3.0 / (8.0 * (R1 + R2)); // 눈으로 보는 스크린 상 도넛에서 뷰어까지 거리

    let mut A: f64 = 0.0; // x축 기준으로 회전
    let mut B: f64 = 0.0; // z축 기준으로 회전
    loop {
        A += 0.07;
        B += 0.03;

        // https://doc.rust-lang.org/std/primitive.f64.html#method.sin_cos
        let (sinA, cosA) = A.sin_cos();
        let (sinB, cosB) = B.sin_cos();

        let mut b = [' '; 1760];
        let mut z = [0.0; 1760];

        let mut j: f64 = 0.0;

        while j <= PI * 2.0 { // 도넛 단면을 구성하기 위한 세타 θ 값
            // (ox,oy,oz)=(R2,0,0)+(R1cosθ,R1sinθ,0) 도넛 object 단면상 x y z
            let (sinθ, cosθ) = j.sin_cos();
            let mut i: f64 = 0.0;
            while i <= PI * 2.0 { // y축을 기준으로 여러 단면들 회전해서 도넛을 만들기 위한 파이 값
                // ((R2+R1cosθ)cosϕ,R1sinθ,−(R2+R1cosθ)sinϕ)
                let(sinϕ, cosϕ) = i.sin_cos();

                let ox = R2 + R1*cosθ;
                let oy = R1 * sinθ;

                let x = ox * (cosB*cosϕ+sinA*sinB*sinϕ)-oy*cosA*sinB; // 최종 3D상 X 좌표
                let y = ox * (cosϕ*sinB - cosB*sinA*sinϕ) + oy*cosA*cosB; // 최종 3D상 Y 좌표
                let ooz = 1.0 / (K2 + cosA * ox * sinϕ + sinA * oy);

                let screenX = (250.0 + (K1 * ooz * x)) as usize; // 스크린 상 X좌표
                let screenY = (120.0 - (K1 * ooz * y)) as usize;

                let o = (screenX + 80 * screenY) as usize;

                // L=(Nx,Ny,Nz)⋅(0,1,−1)=cosϕcosθsinB−cosAcosθsinϕ−sinAsinθ+cosB(cosAsinθ−cosθsinAsinϕ)
                // 해당 좌표의 밝기 값 인덱싱
                let LIndex = (8.0 * (cosϕ*cosθ*sinB - cosϕ*cosθ*sinB - cosA*cosθ*sinϕ - sinA*sinθ+cosB*(cosA*sinθ - cosθ*sinA*sinϕ))) as usize;
                if y < 22.0 && x < 79.0 && LIndex< 1760 && o < 1760 && ooz > z[o] {
                    z[o] = ooz;
                    b[o] = (".,-~:;=".to_owned() + "!*#$@")
                        .chars()
                        .nth(LIndex as usize)
                        .or(Some('.'))
                        .unwrap();
                }
                i += 0.02;
            }
            j += 0.07;
        }
        print!(
            "\x1B[H{}",
            b.chunks(80)
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
        thread::sleep(time::Duration::from_millis(20));
    }
}

// use std::thread;
// use std::time;
//
// fn main() {
//     let (mut A, mut B): (f64, f64) = (0.0, 0.0);
//
//     loop {
//         A += 0.07;
//         B += 0.03;
//         let ((sinA, cosA), (sinB, cosB), mut b) = (
//             A.sin_cos(),
//             B.sin_cos(),
//             [' '; 1760],);
//
//         let (mut z, mut j): ([f64; 1760], f64) =(
//             [0.0; 1760],
//             0.0,
//         );
//         while j <= 6.28 {
//             let (u, v) = (j.sin_cos());
//             let mut i: f64 = 0.0;
//             while i <= 6.28 {
//                 let (w, c) = (i.sin_cos());
//                 let h = v + 2.0;
//                 let (d, t) = (1.0 / (w * h * sinA + u * cosA + 5.0), w * h * cosA - u * sinA);
//                 let (x, y) = (
//                     (40.0 + 30.0 * d * (c * h * cosB - t * sinB)) as usize,
//                     (12.0 + 15.0 * d * (c * h * sinB + t * cosB)) as usize,
//                 );
//                 let (o, n) = (
//                     x + 80 * y,
//                     8.0 * ((u * sinA - w * v * cosA) * cosB - w * v * sinA - u * cosA - c * v * sinB),
//                 );
//                 if y < 22 && x < 79 && d > z[o] {
//                     z[o] = d;
//                     b[o] = (".,-~:;=".to_owned() + "!*#$@")
//                         .chars()
//                         .nth(n as usize)
//                         .or(Some('.'))
//                         .unwrap();
//                 }
//                 i += 0.02
//             }
//             j += 0.07
//         }
//         print!(
//             "\x1B[H{}",
//             b.chunks(80)
//                 .map(|l| l.iter().collect::<String>())
//                 .collect::<Vec<String>>()
//                 .join("\n")
//         );
//         thread::sleep(time::Duration::from_millis(20));
//     }
// }
