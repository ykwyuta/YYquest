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
    pub(crate) const fn name(self) -> &'static str {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Delivery {
    Speech {
        name: &'static str,
        face: Option<Portrait>,
    },
    Scene,
    Record(&'static str),
    Guide,
}

impl Delivery {
    pub(crate) const fn spoken(
        name: &'static str,
        speaker: Speaker,
        expression: Expression,
    ) -> Self {
        Self::Speech {
            name,
            face: Some(Portrait::new(speaker, expression)),
        }
    }

    pub(crate) fn is_speech(self) -> bool {
        matches!(self, Self::Speech { .. })
    }

    pub(crate) fn portrait(self) -> Option<Portrait> {
        match self {
            Self::Speech { face, .. } => face,
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Message {
    pub lines: [&'static str; 2],
    pub delivery: Delivery,
}

impl Message {
    pub(crate) const fn line(text: &'static str, delivery: Delivery) -> Self {
        Self {
            lines: [text, ""],
            delivery,
        }
    }

    pub(crate) fn text(&self) -> &[&'static str] {
        if self.lines[1].is_empty() {
            &self.lines[..1]
        } else {
            &self.lines
        }
    }

    pub(crate) const fn narration(lines: [&'static str; 2]) -> Self {
        Self {
            lines,
            delivery: Delivery::Scene,
        }
    }

    pub(crate) const fn guide(lines: [&'static str; 2]) -> Self {
        Self {
            lines,
            delivery: Delivery::Guide,
        }
    }

    pub(crate) const fn record(source: &'static str, lines: [&'static str; 2]) -> Self {
        Self {
            lines,
            delivery: Delivery::Record(source),
        }
    }

    pub(crate) const fn spoken(
        speaker: Speaker,
        expression: Expression,
        lines: [&'static str; 2],
    ) -> Self {
        Self {
            lines,
            delivery: Delivery::spoken(speaker.name(), speaker, expression),
        }
    }
}
