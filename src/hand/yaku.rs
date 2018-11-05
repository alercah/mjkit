use crate::hand::{CompleteHand, WinContext};
use lazy_static::lazy_static;

/// The value of a yaku.
pub enum Val {
    /// The yaku is worth some number of han.
    Han(u8),
    /// The yaku is a fixed mangan.
    Mangan,
    /// A yakuman.
    Yakuman,
    /// Double yakuman.
    DoubleYakuman,
}

/// The value of a yaku in an open hand.
pub enum OpenVal {
    /// The yaku is worth full value.
    Full,
    /// The yaku is worth one fewer han in an open hand.
    Reduced,
    /// The yaku cannot occur in an open hand.
    Invalid,
}

/// A yaku.
pub struct Yaku {
    /// The name of the yaku, in kanji.
    pub kanji: &'static str,
    /// The name of the yaku, in romaji.
    pub romaji: &'static str,
    /// The name of the yaku, in English.
    pub english: &'static str,
    /// The normal value of the yaku.
    pub val: Val,
    /// The modifier of the yaku's value in an open hand.
    pub open_val: OpenVal,
    /// Evaluate whether a hand contains a yaku.
    pub in_hand: fn(&CompleteHand, &WinContext) -> bool,
}

lazy_static! {
    static ref KOKUSHI: Yaku = Yaku {
        kanji: "",
        romaji: "kokushimusō",
        english: "Thirteen Orphans",
        val: Val::Yakuman,
        open_val: OpenVal::Invalid,
        in_hand: |h, _| match h {
            CompleteHand::Kokushi(_) => true,
            _ => false,
        },
    };
    static ref KOKUSHI_13: Yaku = Yaku {
        kanji: "",
        romaji: "kokushimusōjūsanmen",
        english: "Thirteen-Sided Thirteen Orphans",
        val: Val::DoubleYakuman,
        open_val: OpenVal::Invalid,
        in_hand: |h, c| match h {
            CompleteHand::Kokushi(tiles) => tiles.iter().filter(|&&t| t == c.agari).count() == 2,
            _ => false,
        },
    };
}
