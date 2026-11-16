#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Speaker {
    Allen,
    Lilith,
    Hearth,
    Lightkeeper,
    Mason,
    Baldo,
    Villager,
    Worker,
    Guard,
    Traveler,
}

impl Speaker {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Allen => "アレン",
            Self::Lilith => "リリス",
            Self::Hearth => "炉番",
            Self::Lightkeeper => "灯台守",
            Self::Mason => "拓本師",
            Self::Baldo => "バルド",
            Self::Villager => "村人",
            Self::Worker => "職人",
            Self::Guard => "衛兵",
            Self::Traveler => "旅人",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Expression {
    Neutral,
    Smile,
    Worried,
    Surprised,
    Resolute,
    Sad,
    #[allow(dead_code)]
    Angry,
    Relieved,
    Speaking,
    Thoughtful,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Portrait {
    pub speaker: Speaker,
    pub expression: Expression,
}

impl Portrait {
    pub(crate) const fn new(speaker: Speaker, expression: Expression) -> Self {
        Self {
            speaker,
            expression,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Message {
    pub lines: [&'static str; 2],
    pub face: Option<Portrait>,
}

impl Message {
    pub(crate) const fn narration(lines: [&'static str; 2]) -> Self {
        Self { lines, face: None }
    }

    pub(crate) const fn spoken(
        speaker: Speaker,
        expression: Expression,
        lines: [&'static str; 2],
    ) -> Self {
        Self {
            lines,
            face: Some(Portrait::new(speaker, expression)),
        }
    }
}
