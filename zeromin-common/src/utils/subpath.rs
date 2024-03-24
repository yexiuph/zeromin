pub struct SUBPATH {
    pub glogic: String,
    pub glevel: String,
    pub gnpctalk: String,
    pub gquest: String,
    pub ganimation: String,
    pub geffect: String,
    pub gchareffect: String,
    pub gskinobject: String,
}

impl SUBPATH {
    pub fn new() -> Self {
        Self {
            glogic: "./data/glogic/glogic.rcc".to_string(),
            glevel: "./data/glogic/level.rcc".to_string(),
            gnpctalk: "./data/glogic/npctalk/npctalk.rcc".to_string(),
            gquest: "./data/glogic/quest/quest.rcc".to_string(),
            ganimation: "./data/animation/animation.rcc".to_string(),
            geffect: "./data/effect/effect.rcc".to_string(),
            gchareffect: "./data/effect/char/effectchar.rcc".to_string(),
            gskinobject: "./data/skinobject/skinobject.rcc".to_string(),
        }
    }
}
