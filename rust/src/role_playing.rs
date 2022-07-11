const DEFAULT_HEALTH: u32 = 100;
const DEFAULT_MANA: u32 = 100;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.is_alive() {
            return None;
        }

        Some(Player {
            health: DEFAULT_HEALTH,
            mana: self.mana.map(|_| DEFAULT_MANA),
            level: self.level,
        })
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana_pool) if mana_pool >= mana_cost => {
                self.mana = Some(mana_pool - mana_cost);
                mana_cost * 2
            }
            // insufficient mana
            Some(_) => 0,
            // not a wizard yet
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_mana_pool() {
        let mut not_a_wizard_yet = Player {
            health: 100,
            mana: None,
            level: 8,
        };
        assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
        assert_eq!(not_a_wizard_yet.health, 95);
    }

    #[test]
    fn test_insufficient_mana() {
        let mut wizard_not_enough_mana = Player {
            health: 100,
            mana: Some(5),
            level: 11,
        };
        assert_eq!(wizard_not_enough_mana.cast_spell(10), 0);
        assert_eq!(wizard_not_enough_mana.health, 100);
        assert_eq!(wizard_not_enough_mana.mana, Some(5));
    }

    #[test]
    fn test_cast_spell_with_no_mana_pool() {
        const MANA_COST: u32 = 10;

        let mut underleveled_player = Player {
            health: 87,
            mana: None,
            level: 6,
        };

        let clone = Player {
            ..underleveled_player
        };

        assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
        assert_eq!(underleveled_player.health, clone.health - MANA_COST);
        assert_eq!(underleveled_player.mana, clone.mana);
        assert_eq!(underleveled_player.level, clone.level);
    }

    #[test]
    fn test_cast_large_spell_with_no_mana_pool() {
        const MANA_COST: u32 = 30;

        let mut underleveled_player = Player {
            health: 20,
            mana: None,
            level: 6,
        };

        assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
        assert_eq!(underleveled_player.health, 0);
        assert_eq!(underleveled_player.mana, None);
        assert_eq!(underleveled_player.level, 6);
    }

    #[test]
    fn test_cast_spell_with_enough_mana() {
        const HEALTH: u32 = 99;
        const MANA: u32 = 100;
        const LEVEL: u32 = 100;
        const MANA_COST: u32 = 3;

        let mut accomplished_wizard = Player {
            health: HEALTH,
            mana: Some(MANA),
            level: LEVEL,
        };

        assert_eq!(accomplished_wizard.cast_spell(MANA_COST), MANA_COST * 2);
        assert_eq!(accomplished_wizard.health, HEALTH);
        assert_eq!(accomplished_wizard.mana, Some(MANA - MANA_COST));
        assert_eq!(accomplished_wizard.level, LEVEL);
    }
}
