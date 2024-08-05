// Copyright 2021-2022 Sebastian Ramacher
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::mem::size_of;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

const SBOX10: [u16; 1024] = [144, 348, 448, 658, 257, 777, 552, 879, 527, 654, 764, 889, 532, 133, 702, 617, 914,
    856, 865, 713, 571, 748, 440, 85, 145, 424, 138, 642, 847, 878, 814, 175, 530, 892,
    248, 275, 688, 473, 866, 715, 153, 488, 667, 623, 284, 117, 620, 443, 1004, 39, 915,
    744, 996, 590, 670, 851, 373, 846, 410, 62, 211, 806, 694, 550, 849, 724, 984, 863,
    353, 499, 975, 45, 1016, 279, 420, 901, 467, 946, 525, 178, 197, 971, 995, 671, 743,
    166, 627, 668, 355, 528, 29, 131, 414, 407, 633, 818, 720, 905, 951, 904, 733, 397,
    83, 893, 850, 326, 434, 985, 585, 219, 752, 1018, 293, 480, 108, 1017, 115, 465, 290,
    611, 278, 130, 693, 40, 297, 181, 840, 782, 90, 291, 909, 333, 706, 137, 925, 444,
    343, 150, 430, 616, 100, 89, 217, 988, 651, 684, 351, 136, 831, 378, 907, 141, 601,
    867, 339, 731, 871, 119, 120, 789, 96, 952, 1013, 943, 404, 347, 68, 938, 769, 286,
    86, 700, 287, 625, 819, 560, 657, 301, 69, 853, 884, 56, 146, 63, 935, 182, 610, 209,
    88, 898, 506, 220, 803, 168, 340, 194, 405, 493, 15, 796, 942, 594, 994, 349, 756,
    656, 312, 635, 810, 740, 712, 957, 180, 111, 931, 595, 189, 451, 299, 362, 841, 330,
    1014, 380, 169, 49, 398, 763, 963, 33, 55, 123, 770, 652, 887, 229, 118, 1022, 714,
    732, 513, 599, 323, 880, 269, 184, 545, 1023, 50, 563, 983, 171, 531, 270, 554, 381,
    728, 852, 762, 875, 198, 711, 953, 266, 861, 66, 576, 441, 730, 214, 188, 110, 402,
    854, 567, 246, 32, 105, 215, 586, 549, 802, 127, 544, 237, 534, 366, 741, 508, 896,
    559, 813, 325, 225, 222, 59, 109, 77, 103, 494, 483, 239, 183, 645, 675, 439, 132,
    704, 913, 628, 600, 945, 792, 778, 547, 2, 855, 210, 459, 476, 363, 466, 725, 641,
    755, 507, 981, 172, 470, 1006, 234, 766, 102, 318, 540, 450, 479, 129, 372, 848, 604,
    565, 122, 746, 317, 155, 780, 719, 710, 10, 251, 71, 663, 504, 218, 964, 1011, 679,
    553, 939, 421, 692, 661, 798, 622, 42, 304, 1002, 543, 751, 640, 230, 80, 192, 902,
    361, 717, 163, 329, 804, 413, 542, 159, 899, 314, 636, 191, 474, 445, 44, 774, 162,
    815, 358, 306, 352, 515, 775, 701, 95, 240, 344, 212, 557, 360, 82, 612, 614, 322,
    566, 930, 858, 60, 650, 808, 447, 937, 665, 512, 1003, 596, 660, 947, 365, 707, 295,
    485, 468, 797, 271, 759, 475, 779, 319, 788, 76, 575, 579, 516, 890, 4, 573, 918, 258,
    991, 461, 630, 598, 140, 106, 541, 336, 838, 597, 519, 998, 621, 65, 976, 979, 511,
    114, 453, 917, 829, 997, 1020, 772, 151, 265, 341, 70, 221, 649, 332, 205, 428, 462,
    609, 548, 388, 411, 303, 773, 767, 92, 23, 589, 941, 820, 555, 750, 249, 500, 735,
    683, 199, 569, 521, 584, 412, 261, 881, 227, 524, 509, 510, 877, 18, 19, 911, 256,
    568, 311, 5, 327, 164, 972, 900, 961, 113, 35, 771, 280, 870, 520, 533, 375, 370, 910,
    26, 124, 882, 765, 170, 452, 934, 501, 385, 432, 53, 860, 390, 583, 177, 320, 674,
    999, 1012, 529, 978, 800, 37, 634, 624, 139, 505, 817, 236, 1019, 536, 469, 51, 666,
    967, 79, 416, 174, 613, 956, 273, 389, 958, 982, 38, 201, 677, 906, 523, 274, 152, 7,
    394, 570, 223, 669, 41, 101, 307, 824, 438, 25, 383, 455, 302, 369, 337, 262, 833,
    538, 72, 897, 646, 655, 167, 267, 927, 791, 582, 776, 843, 309, 43, 648, 28, 57, 176,
    300, 97, 342, 696, 783, 20, 729, 705, 496, 960, 208, 615, 231, 781, 929, 673, 753,
    790, 409, 919, 949, 206, 698, 695, 874, 374, 491, 54, 16, 845, 433, 832, 517, 591,
    173, 31, 738, 1000, 264, 367, 727, 664, 989, 425, 442, 437, 886, 276, 837, 73, 232,
    785, 959, 12, 193, 161, 632, 46, 872, 64, 449, 992, 894, 418, 709, 382, 328, 522, 827,
    247, 253, 406, 93, 966, 30, 537, 844, 526, 842, 87, 202, 415, 179, 546, 203, 316, 928,
    149, 825, 379, 626, 446, 268, 954, 721, 143, 386, 535, 431, 263, 686, 243, 795, 495,
    422, 134, 245, 460, 703, 34, 395, 1009, 252, 933, 716, 315, 260, 241, 1015, 313, 292,
    400, 754, 126, 259, 244, 916, 723, 490, 204, 472, 310, 644, 637, 940, 213, 98, 187,
    27, 24, 821, 639, 603, 52, 484, 396, 1010, 761, 125, 857, 393, 94, 298, 74, 346, 487,
    399, 659, 384, 185, 112, 148, 392, 593, 359, 895, 969, 158, 14, 224, 742, 9, 47, 908,
    17, 830, 368, 21, 834, 48, 156, 0, 699, 196, 514, 678, 685, 973, 1001, 334, 962, 868,
    377, 11, 672, 793, 13, 285, 948, 690, 587, 993, 1007, 558, 607, 277, 936, 294, 749,
    1021, 350, 605, 435, 869, 228, 401, 970, 3, 602, 436, 335, 75, 681, 662, 426, 331,
    739, 456, 489, 1008, 356, 142, 619, 708, 691, 760, 955, 423, 990, 932, 128, 987, 592,
    321, 891, 454, 482, 556, 308, 497, 835, 84, 481, 147, 836, 968, 977, 689, 697, 226,
    477, 903, 794, 200, 417, 680, 408, 238, 564, 471, 747, 687, 920, 888, 885, 859, 154,
    458, 653, 726, 580, 345, 67, 354, 876, 486, 561, 403, 816, 809, 578, 463, 577, 822,
    736, 283, 250, 272, 376, 364, 492, 950, 581, 608, 737, 427, 682, 165, 464, 6, 107,
    912, 338, 207, 1, 768, 457, 288, 562, 305, 387, 255, 539, 242, 498, 58, 757, 391, 638,
    986, 121, 864, 190, 324, 980, 826, 676, 965, 718, 91, 186, 921, 923, 81, 216, 787,
    157, 784, 812, 289, 643, 629, 235, 588, 296, 786, 839, 135, 503, 551, 758, 371, 722,
    734, 922, 944, 807, 281, 572, 233, 883, 99, 618, 502, 805, 8, 574, 606, 647, 478, 745,
    823, 22, 862, 61, 873, 429, 254, 974, 36, 195, 828, 357, 518, 160, 78, 116, 924, 811,
    631, 419, 282, 799, 104, 1005, 926, 801];

/// Produce mask for padding.
#[inline(always)]
pub const fn pad(n: usize) -> u64 {
    0x80_u64 << (56 - 8 * n)
}

/// Compute round constant
#[inline(always)]
const fn round_constant(round: u64) -> u64 {
    ((0xfu64 - round) << 4) | round
}

/// The state of Ascon's permutation.
///
/// The permutation operates on a state of 320 bits represented as 5 64 bit words.
#[derive(Clone, Debug, Default)]
pub struct State {
    x: [u64; 5],
}

/// Ascon's round function
fn round(mut x: [u64; 5], c: u64) -> [u64; 5] {
    x[2] = x[2] ^ c;

    let arr10u32: [u32; 10] = unsafe { core::mem::transmute(x) };
    let mut tarray10u32: [u32; 10] = [0u32; 10];

    // S-box layer
    for bit_index in 0..32 {
        let mut sbox_input = 0u16;
        for cell_index in 0..10 {
            sbox_input |= (((arr10u32[cell_index] & (1 << bit_index)) != 0) as u16) << cell_index;
        }
        let sbox_output = SBOX10[sbox_input as usize];
        for cell_index in 0..10 {
            tarray10u32[cell_index] |= (((sbox_output >> cell_index) & 1) as u32) << bit_index;
        }
    }

    let x: [u64; 5] = unsafe { core::mem::transmute(tarray10u32) };

    let tx0 = x[0];
    let tx1 = x[1];
    let tx2 = x[2];
    let tx3 = x[3];
    let tx4 = x[4];

    // linear layer
    let x0 = tx0 ^ tx0.rotate_right(9);
    let x1 = tx1 ^ tx1.rotate_right(22);
    let x2 = tx2 ^ tx2.rotate_right(5);
    let x3 = tx3 ^ tx3.rotate_right(7);
    let x4 = tx4 ^ tx4.rotate_right(34);
    [
        tx0 ^ x0.rotate_right(19),
        tx1 ^ x1.rotate_right(39),
        !(tx2 ^ x2.rotate_right(1)),
        tx3 ^ x3.rotate_right(10),
        tx4 ^ x4.rotate_right(7),
    ]
}

impl State {
    /// Instantiate new state from the given values.
    pub fn new(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> Self {
        State {
            x: [x0, x1, x2, x3, x4],
        }
    }

    #[cfg(not(feature = "no_unroll"))]
    /// Perform permutation with 12 rounds.
    pub fn permute_12(&mut self) {
        // We could in theory iter().fold() over an array of round constants,
        // but the compiler produces better results when optimizing this chain
        // of round function calls.
        self.x = round(
            round(
                round(
                    round(
                        round(
                            round(
                                round(
                                    round(
                                        round(round(round(round(self.x, 0xf0), 0xe1), 0xd2), 0xc3),
                                        0xb4,
                                    ),
                                    0xa5,
                                ),
                                0x96,
                            ),
                            0x87,
                        ),
                        0x78,
                    ),
                    0x69,
                ),
                0x5a,
            ),
            0x4b,
        );
    }

    #[cfg(feature = "no_unroll")]
    /// Perform permutation with 12 rounds.
    pub fn permute_12(&mut self) {
        self.x = [
            0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
        ]
        .into_iter()
        .fold(self.x, round);
    }

    #[cfg(not(feature = "no_unroll"))]
    /// Perform permutation with 8 rounds.
    pub fn permute_8(&mut self) {
        self.x = round(
            round(
                round(
                    round(
                        round(round(round(round(self.x, 0xb4), 0xa5), 0x96), 0x87),
                        0x78,
                    ),
                    0x69,
                ),
                0x5a,
            ),
            0x4b,
        );
    }

    #[cfg(feature = "no_unroll")]
    /// Perform permutation with 8 rounds.
    pub fn permute_8(&mut self) {
        self.x = [0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b]
            .into_iter()
            .fold(self.x, round);
    }

    #[cfg(not(feature = "no_unroll"))]
    /// Perform permutation with 6 rounds.
    pub fn permute_6(&mut self) {
        self.x = round(
            round(
                round(round(round(round(self.x, 0x96), 0x87), 0x78), 0x69),
                0x5a,
            ),
            0x4b,
        );
    }

    #[cfg(feature = "no_unroll")]
    /// Perform permutation with 6 rounds.
    pub fn permute_6(&mut self) {
        self.x = [0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b]
            .into_iter()
            .fold(self.x, round);
    }

    /// Perform permutation with 1 round
    pub fn permute_1(&mut self) {
        self.x = round(self.x, 0x4b);
    }

    /// Perform a given number (up to 12) of permutations
    ///
    /// Panics (in debug mode) if `rounds` is larger than 12.
    pub fn permute_n(&mut self, rounds: usize) {
        debug_assert!(rounds <= 12);

        let start = 12 - rounds;
        self.x = (start..12).fold(self.x, |x, round_index| {
            round(x, round_constant(round_index as u64))
        });
    }

    /// Convert state to bytes.
    pub fn as_bytes(&self) -> [u8; 40] {
        let mut bytes = [0u8; size_of::<u64>() * 5];
        for (dst, src) in bytes.chunks_exact_mut(size_of::<u64>()).zip(self.x) {
            dst.copy_from_slice(&u64::to_be_bytes(src));
        }
        bytes
    }
}

impl core::ops::Index<usize> for State {
    type Output = u64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.x[index]
    }
}

impl core::ops::IndexMut<usize> for State {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.x[index]
    }
}

impl TryFrom<&[u64]> for State {
    type Error = ();

    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        match value.len() {
            5 => Ok(Self::new(value[0], value[1], value[2], value[3], value[4])),
            _ => Err(()),
        }
    }
}

impl From<&[u64; 5]> for State {
    fn from(value: &[u64; 5]) -> Self {
        Self { x: *value }
    }
}

impl TryFrom<&[u8]> for State {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != core::mem::size_of::<u64>() * 5 {
            return Err(());
        }

        let mut state = Self::default();
        for (src, dst) in value
            .chunks_exact(core::mem::size_of::<u64>())
            .zip(state.x.iter_mut())
        {
            *dst = u64::from_be_bytes(src.try_into().unwrap());
        }
        Ok(state)
    }
}

impl From<&[u8; size_of::<u64>() * 5]> for State {
    fn from(value: &[u8; size_of::<u64>() * 5]) -> Self {
        let mut state = Self::default();
        for (src, dst) in value
            .chunks_exact(core::mem::size_of::<u64>())
            .zip(state.x.iter_mut())
        {
            *dst = u64::from_be_bytes(src.try_into().unwrap());
        }
        state
    }
}

impl AsRef<[u64]> for State {
    fn as_ref(&self) -> &[u64] {
        &self.x
    }
}

#[cfg(feature = "zeroize")]
impl Drop for State {
    fn drop(&mut self) {
        self.x.zeroize();
    }
}

#[cfg(feature = "zeroize")]
impl ZeroizeOnDrop for State {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_0to7() {
        assert_eq!(pad(0), 0x8000000000000000);
        assert_eq!(pad(1), 0x80000000000000);
        assert_eq!(pad(2), 0x800000000000);
        assert_eq!(pad(3), 0x8000000000);
        assert_eq!(pad(4), 0x80000000);
        assert_eq!(pad(5), 0x800000);
        assert_eq!(pad(6), 0x8000);
        assert_eq!(pad(7), 0x80);
    }

    #[test]
    fn round_constants() {
        assert_eq!(round_constant(0), 0xf0);
        assert_eq!(round_constant(1), 0xe1);
        assert_eq!(round_constant(2), 0xd2);
        assert_eq!(round_constant(3), 0xc3);
        assert_eq!(round_constant(4), 0xb4);
        assert_eq!(round_constant(5), 0xa5);
        assert_eq!(round_constant(6), 0x96);
        assert_eq!(round_constant(7), 0x87);
        assert_eq!(round_constant(8), 0x78);
        assert_eq!(round_constant(9), 0x69);
        assert_eq!(round_constant(10), 0x5a);
        assert_eq!(round_constant(11), 0x4b);
    }

    #[test]
    fn one_round() {
        let state = round(
            [
                0x0123456789abcdef,
                0x23456789abcdef01,
                0x456789abcdef0123,
                0x6789abcdef012345,
                0x89abcde01234567f,
            ],
            0x1f,
        );
        assert_eq!(
            state,
            [
                0x3c1748c9be2892ce,
                0x5eafb305cd26164f,
                0xf9470254bb3a4213,
                0xf0428daf0c5d3948,
                0x281375af0b294899
            ]
        );
    }

    #[test]
    fn state_permute_12() {
        let mut state = State::new(
            0x0123456789abcdef,
            0xef0123456789abcd,
            0xcdef0123456789ab,
            0xabcdef0123456789,
            0x89abcdef01234567,
        );
        state.permute_12();
        assert_eq!(state[0], 0x206416dfc624bb14);
        assert_eq!(state[1], 0x1b0c47a601058aab);
        assert_eq!(state[2], 0x8934cfc93814cddd);
        assert_eq!(state[3], 0xa9738d287a748e4b);
        assert_eq!(state[4], 0xddd934f058afc7e1);
    }

    #[test]
    fn state_permute_6() {
        let mut state = State::new(
            0x0123456789abcdef,
            0xef0123456789abcd,
            0xcdef0123456789ab,
            0xabcdef0123456789,
            0x89abcdef01234567,
        );
        state.permute_6();
        assert_eq!(state[0], 0xc27b505c635eb07f);
        assert_eq!(state[1], 0xd388f5d2a72046fa);
        assert_eq!(state[2], 0x9e415c204d7b15e7);
        assert_eq!(state[3], 0xce0d71450fe44581);
        assert_eq!(state[4], 0xdd7c5fef57befe48);
    }

    #[test]
    fn state_permute_8() {
        let mut state = State::new(
            0x0123456789abcdef,
            0xef0123456789abcd,
            0xcdef0123456789ab,
            0xabcdef0123456789,
            0x89abcdef01234567,
        );
        state.permute_8();
        assert_eq!(state[0], 0x67ed228272f46eee);
        assert_eq!(state[1], 0x80bc0b097aad7944);
        assert_eq!(state[2], 0x2fa599382c6db215);
        assert_eq!(state[3], 0x368133fae2f7667a);
        assert_eq!(state[4], 0x28cefb195a7c651c);
    }

    #[test]
    fn state_permute_n() {
        let mut state = State::new(
            0x0123456789abcdef,
            0xef0123456789abcd,
            0xcdef0123456789ab,
            0xabcdef0123456789,
            0x89abcdef01234567,
        );
        let mut state2 = state.clone();

        state.permute_6();
        state2.permute_n(6);
        assert_eq!(state.x, state2.x);

        state.permute_8();
        state2.permute_n(8);
        assert_eq!(state.x, state2.x);

        state.permute_12();
        state2.permute_n(12);
        assert_eq!(state.x, state2.x);
    }

    #[test]
    fn state_convert_bytes() {
        let state = State::new(
            0x0123456789abcdef,
            0xef0123456789abcd,
            0xcdef0123456789ab,
            0xabcdef0123456789,
            0x89abcdef01234567,
        );
        let bytes = state.as_bytes();

        // test TryFrom<&[u8]>
        let state2 = State::try_from(&bytes[..]);
        assert_eq!(state2.expect("try_from bytes").x, state.x);

        let state2 = State::from(&bytes);
        assert_eq!(state2.x, state.x);
    }
}
