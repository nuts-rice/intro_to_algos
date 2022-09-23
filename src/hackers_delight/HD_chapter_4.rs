///propogating bounds through adds
pub fn boundsAdd(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a + c;
    let mut t = b + d;
    let u = a & c & !s & !(b & d & !t);
    let v = ((a ^ c) | !(a ^ s)) & (!b & !d & t);
    if (u | v) < 0 {
        s = 0x80000000;
        t = 0x7FFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_boundsAdd() {
    let test = [
        [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        ],
        [3, 6, 2, 7, 5, 13],
        [-6, -3, -7, -2, -13, -5],
    ];
    for i in 0..3 {
        let (s, t) = boundsAdd(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(s, test[i][4]);
        assert_eq!(t, test[i][5]);
    }
}

pub fn boundsSignedAdd(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a + c;
    let mut t = b + d;
    if s >= a && t < b {
        s = 0;
        t = 0xFFFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_boundsSignedAdd() {
    let test = [
        [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        ],
        [3, 6, 2, 7, 5, 13],
        [-6, -3, -7, -2, -13, -5],
    ];
    for i in 0..3 {
        let (s, t) = boundsSignedAdd(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(s, test[i][4]);
        assert_eq!(t, test[i][5]);
    }
}

pub fn boundsSub(a: i64, b: i64, c: i64, d: i64) -> (i64, i64) {
    let mut s = a - d;
    let mut t = b - c;
    if s > a && t <= b {
        s = 0;
        t = 0xFFFFFFFF;
    }
    (s, t)
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn boundsSub_test1() {
    let test = [[
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    ]];
    for i in 0..1 {
        let (s, t) = boundsSub(test[i][0], test[i][1], test[i][2], test[i][3]);
        assert_eq!(t, test[i][5]);
    }
}
