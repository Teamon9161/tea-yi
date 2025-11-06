use std::iter::FromIterator;

/// 读取顺序：从“上爻”到“下爻”（MSB-first），true=阳(1)，false=阴(0)。
#[inline]
fn bits6_msb<I: IntoIterator<Item = bool>>(iter: I) -> u8 {
    iter.into_iter().fold(0u8, |acc, b| (acc << 1) | (b as u8))
}

/// 文王序（1..=64）卦名（标准次序）
pub const KW_NAMES: [&str; 64] = [
    "乾","坤","屯","蒙","需","讼","师","比","小畜","履","泰","否","同人","大有","谦","豫",
    "随","蛊","临","观","噬嗑","贲","剥","复","无妄","大畜","颐","大过","坎","离","咸","恒",
    "遯","大壮","晋","明夷","家人","睽","蹇","解","损","益","夬","姤","萃","升","困","井","革",
    "鼎","震","艮","渐","归妹","丰","旅","巽","兑","涣","节","中孚","小过","既济","未济",
];

/// 0..63（按上→下读成二进制的值） → 文王序号（1..=64）
/// 数据来自维基“周易后天六十四卦：将每卦由上到下看成二进位数字的对应表”的倒排
pub const KW_BY_BIN: [u8; 64] = [
     2, 24,  7, 19, 15, 36, 46, 11,
    16, 51, 40, 54, 62, 55, 31, 33,
     8,  3, 29, 60, 39, 63, 48,  5,
    45, 17, 47, 58, 30, 49, 28, 43,
    23, 27,  4, 41, 52, 22, 18, 26,
    35, 21, 64, 38, 56, 32, 50, 14,
    20, 42, 59, 61, 53, 37, 57,  9,
    12, 25,  6, 10, 34, 13, 44,  1,
];

/// 由 6 个 bool（上→下）得到文王序号（1..=64）、卦名、Unicode 卦符号（U+4DC0..）
#[derive(Debug, Clone, Copy)]
pub struct Hexagram(pub u8);

impl Hexagram {
    #[inline]
    /// 文王卦序
    pub fn kw(&self) -> u8 {
        KW_BY_BIN[self.0 as usize] // 1..=64
    }

    #[inline]
    // ䷀..䷿
    pub fn unicode(&self) -> char {
        char::from_u32(0x4DC0 + (self.kw() as u32) - 1).unwrap()
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        KW_NAMES[(self.kw() - 1) as usize]
    }

    #[inline]
    pub fn from_slice(slice: &[bool]) -> Self {
        debug_assert!(slice.len() == 6);
        Self(bits6_msb(slice.iter().cloned()))
    }
}


impl FromIterator<bool> for Hexagram {
    #[inline]
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Self(bits6_msb(iter))
    }
}

impl From<&[bool; 6]> for Hexagram {
    #[inline]
    fn from(r: &[bool; 6]) -> Self {
        Self(bits6_msb(r.iter().cloned()))
    }
}

impl From<[bool; 6]> for Hexagram {
    #[inline]
    fn from(r: [bool; 6]) -> Self {
        Self(bits6_msb(r.into_iter()))
    }
}


// impl<I: IntoIterator<Item=bool>> From<I> for Hexagram {
//     #[inline]
//     fn from(i: I) -> Self {
//         Self(bits6_msb(i))
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        // 乾为 1 号卦，六阳，上→下：111111（二进制 63）
        let h = Hexagram::from([true; 6]);
        assert_eq!(h.kw(), 1);
        assert_eq!(h.name(), "乾");
        assert_eq!(h.unicode(), '䷀');

        // 坤为 2 号卦，六阴，上→下：000000（二进制 0）
        let h = Hexagram::from([false; 6]);
        assert_eq!(h.kw(), 2);
        assert_eq!(h.name(), "坤");
        assert_eq!(h.unicode(), '䷁');

        let h = Hexagram::from([true, false, false, false, true, false]);
        assert_eq!(h.kw(), 4);
        assert_eq!(h.name(), "蒙");
        assert_eq!(h.unicode(), '䷃');

        let h = Hexagram::from([false, true, true, false, false, false]);
        assert_eq!(h.kw(), 45);
        assert_eq!(h.name(), "萃");
        assert_eq!(h.unicode(), '䷬');

    }
}
