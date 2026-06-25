use std::fmt::{self, Display};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ItemBehaviour {
    stuff: bool,
    item: bool,
    on_floor: bool,
    on_wall: bool,
    passive_object: bool,
    invisible: bool,
    trigger: bool,
    can_sit_on_top: bool,
    can_lay_on_top: bool,
    can_stand_on_top: bool,
    can_stack_on_top: bool,
    requires_rights_for_interaction: bool,
    requires_touching_for_interaction: bool,
    decoration: bool,
    post_it: bool,
    photo: bool,
    door: bool,
    teleporter: bool,
    dice: bool,
    prize_trophy: bool,
}

impl ItemBehaviour {
    pub fn parse(flags: &str) -> Self {
        let mut behaviour = Self::default();

        for flag in flags.chars() {
            match flag {
                'S' => behaviour.stuff = true,
                'I' => behaviour.item = true,
                'F' => behaviour.on_floor = true,
                'W' => behaviour.on_wall = true,
                'P' => behaviour.passive_object = true,
                'E' => behaviour.invisible = true,
                'M' => behaviour.trigger = true,
                'C' => behaviour.can_sit_on_top = true,
                'B' => behaviour.can_lay_on_top = true,
                'K' => behaviour.can_stand_on_top = true,
                'G' => behaviour.requires_rights_for_interaction = true,
                'T' => behaviour.requires_touching_for_interaction = true,
                'H' => behaviour.can_stack_on_top = true,
                'V' => behaviour.decoration = true,
                'J' => behaviour.post_it = true,
                'N' => behaviour.photo = true,
                'D' => behaviour.door = true,
                'X' => behaviour.teleporter = true,
                'L' => behaviour.dice = true,
                'Y' => behaviour.prize_trophy = true,
                _ => {}
            }
        }

        behaviour
    }

    pub fn is_stuff(&self) -> bool {
        self.stuff
    }

    pub fn is_item(&self) -> bool {
        self.item
    }

    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }

    pub fn is_on_wall(&self) -> bool {
        self.on_wall
    }

    pub fn is_passive_object(&self) -> bool {
        self.passive_object
    }

    pub fn is_invisible(&self) -> bool {
        self.invisible
    }

    pub fn is_trigger(&self) -> bool {
        self.trigger
    }

    pub fn can_sit_on_top(&self) -> bool {
        self.can_sit_on_top
    }

    pub fn can_lay_on_top(&self) -> bool {
        self.can_lay_on_top
    }

    pub fn can_stand_on_top(&self) -> bool {
        self.can_stand_on_top
    }

    pub fn can_stack_on_top(&self) -> bool {
        self.can_stack_on_top
    }

    pub fn requires_rights_for_interaction(&self) -> bool {
        self.requires_rights_for_interaction
    }

    pub fn requires_touching_for_interaction(&self) -> bool {
        self.requires_touching_for_interaction
    }

    pub fn is_decoration(&self) -> bool {
        self.decoration
    }

    pub fn is_post_it(&self) -> bool {
        self.post_it
    }

    pub fn is_photo(&self) -> bool {
        self.photo
    }

    pub fn is_door(&self) -> bool {
        self.door
    }

    pub fn is_teleporter(&self) -> bool {
        self.teleporter
    }

    pub fn is_dice(&self) -> bool {
        self.dice
    }

    pub fn is_prize_trophy(&self) -> bool {
        self.prize_trophy
    }
}

impl Display for ItemBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = [
            (self.stuff, 'S'),
            (self.item, 'I'),
            (self.on_floor, 'F'),
            (self.on_wall, 'W'),
            (self.can_sit_on_top, 'C'),
            (self.can_lay_on_top, 'B'),
            (self.can_stand_on_top, 'K'),
            (self.passive_object, 'P'),
            (self.invisible, 'E'),
            (self.trigger, 'M'),
            (self.requires_rights_for_interaction, 'G'),
            (self.requires_touching_for_interaction, 'T'),
            (self.can_stack_on_top, 'H'),
            (self.decoration, 'V'),
            (self.post_it, 'J'),
            (self.photo, 'N'),
            (self.door, 'D'),
            (self.teleporter, 'X'),
            (self.dice, 'L'),
            (self.prize_trophy, 'Y'),
        ];

        for (enabled, flag) in flags {
            if enabled {
                write!(f, "{flag}")?;
            }
        }

        Ok(())
    }
}
