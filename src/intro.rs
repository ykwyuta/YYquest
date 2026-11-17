use crate::island::Position;

pub(crate) mod script;

pub(crate) const HEARTH: Position = (3, 2);
pub(crate) const MEETING: Position = (13, 6);
pub(crate) const SLUICE: Position = (10, 7);
pub(crate) const MEMORIAL: Position = (14, 2);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum Intro {
    #[default]
    Hearth,
    Meeting,
    Sluice,
    Memorial,
    Choice,
    Return,
    Nox,
}

impl Intro {
    pub(crate) fn destination(self) -> (Position, &'static str) {
        match self {
            Self::Hearth => (HEARTH, "案内精霊トモル：炉番に水門のことを聞こう"),
            Self::Meeting => (MEETING, "案内精霊トモル：東の森で笛の音を探そう"),
            Self::Sluice => (SLUICE, "案内精霊トモル：森の旧水門へ向かおう"),
            Self::Memorial => (MEMORIAL, "案内精霊トモル：北の石碑でリリスと合流しよう"),
            Self::Choice => (MEMORIAL, "案内精霊トモル：石碑の前でリリスの話を聞こう"),
            Self::Return => (HEARTH, "案内精霊トモル：二人で炉番に報告しよう"),
            Self::Nox => (HEARTH, "地図（M）でノクスの各地を巡ろう"),
        }
    }

    pub(crate) fn waiting_lilith(self) -> Position {
        match self {
            Self::Hearth | Self::Meeting => MEETING,
            Self::Sluice => (SLUICE.0 + 1, SLUICE.1),
            _ => (MEMORIAL.0 - 1, MEMORIAL.1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lesson {
    Sluice,
    Memorial,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum After {
    #[default]
    None,
    HearthTask,
    Continue(Intro),
    Fight(Lesson),
    Ask,
    Reply,
    Recruit,
    BeginNox,
}
