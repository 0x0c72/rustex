use thiserror::Error;
use std::collections::HashMap;
use std::iterator::IntoIter;

#[derive(Debug, PartialEq, Eq)]
enum Stat {
    Strength,
    Dexterity,
    Stamina,
    Energy
}

#[derive(Debug, PartialEq, Eq)]
enum EquipmentSlot {
    Head,
    Body,
    Legs
}

#[derive(Debug, PartialEq, Eq)]
enum EquipmentSlotStatus {
    Occupied(Equipment),
    Empty
}

#[derive(Debug, PartialEq, Eq)]
enum InventorySlotStatus {
    Occupied(Item),
    Empty
}

#[derive(Debug, PartialEq, Eq, Error)]
enum InventorySlotError {
    #[error("Slot is occupied")]
    SlotOccupied(InventorySlotStatus)
}

#[derive(Debug, PartialEq, Eq, Error)]
enum InventoryError {
    #[error("Inventory is full")]
    InventoryFull
}

#[derive(Debug, PartialEq, Eq, Error)]
enum EquipError {
    #[error("Item cannot be equipped in this slot")]
    WrongSlot(EquipmentSlot),
    #[error("Item requirements not met")]
    Requirements(RequirementsError)
}

#[derive(Debug, PartialEq, Eq)]
struct RequirementsError {
    requirements: HashMap<Stat, u16>
 }

// do I need these 3 impls for RequirementsError?  Since it isn't a standalone error but a value for an error enum
// I think I need new, but i guess not the others?
impl RequirementsError {
    fn new(requirements: HashMap<Stat, u16>, details: String) -> Self {
        RequirementsError{
            requirements
        }
    }
}

impl fmt::Display for RequirementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for RequirementsError {
    fn description(&self) -> &str {
        &self.requirements // not sure what goes here
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    name: String,
    inventory: Inventory::new(),
    equipped: HashMap<EquipmentSlot, EquipmentSlotStatus>,
    stats: HashMap<Stat, u16>,
}

#[derive(Debug, PartialEq, Eq)]
struct Inventory {
    slots: HashMap<u8, InventorySlotStatus>
}

impl Inventory {
    fn new(&self) -> Self {
        // how do i initialize the HashMap to have 40 slots with InventorySlotStatus::Empty?
        let inventory = HashMap::<u8, InventorySlotStatus>new();
        for i in 1..=40 {
            inventory.insert(i, InventorySlotStatus::Empty);
        }
        inventory
    }
    // self is &mut because I'm changing what's in the hashmap??
    fn add<T: CanHold>(&mut self, item: &T) -> Result<(), InventoryError> {
        // checking for open slot, add it to the hashmap otherwise return error InventoryFull
        self.slots.iter().map(|k, v| 
            if v == InventorySlotStatus::Empty { 
                v = InventorySlotStatus::Occupied(&item) 
            } else { 
                InventorySlotError::SlotOccupied(InventorySlotStatus::Occupied(&item))
            });
        // how do i return Ok and Err from this? (let alone any fn that returns Result<>)
    }
}

trait CanHold {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>;
}

trait CanEquip {
    fn equip(&self) -> Result<(), EquipError>;
    fn check_requirements(&self, player: &Player) -> Result<(), EquipError>;
}

#[derive(Debug, PartialEq, Eq)]
struct Item {

}

impl Item {

}

#[derive(Debug, PartialEq, Eq)]
struct Equipment {
    name: String,
    owner: Player, // should this be &Player do i use & in struct field type definitions?
    requirements: HashMap<Stat, u16>,
    slot: EquipmentSlot
}

impl Equipment {
    name: String,
    owner: Player
}

impl CanEquip for Equipment {
    fn equip(&self) -> Result<(), Self::Error> {
        // check if slot is empty
        self.check_requirements()?; // can I call this like this?  If it returns an error how can I make this fn exit?
        if let slot_status = self.owner.equipped.get(self.slot) { // instead of match (slot_status) on next line
            EquipmentSlotStatus::Empty => self.owner.equipped.get_mut(self.slot) = &self, // do i need unwrap?  should i pass by reference?
            EquipmentSlotStatus::Occupied(value) => { 
                add_to_inventory(value, self.owner); //do i need & for these references to fields?
                self.owner.equipped.get_mut(self.slot) = &self;
            }// does a comma go here?
        }       
    }
    // i just know i'm doing something wrong here lol
    fn check_requirements(&self) -> Result<(), EquipError> {
        //  this approach?
        self.requirements.all(|k, v| 
            if self.owner.stats.contains_key(k) { 
                if self.owner.stats.get(k) < requirements.get(k) {
                    Err(EquipError::Requirements(RequirementsError::new(self.requirements)))
                } else {
                    Ok()
                }
            } else  {
                Err(EquipError:Requirements(RequirementsError::new(requirements)))
            });
        // or this approach? which is more rust like?
        for (stat, value) in &*self.requirements { // immutable borrow so the original still exists?
            if self.owner.stats.get(stat) < value {
                Err(EquipError::Requirements(RequirementsError::new(self.requirements)))
            }
        }
    }
}

impl CanHold for Equipment {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>; {
            Ok(player.inventory.add(&self)?) // how can I match on the Result return value of fn add() to return either Ok or Err will ? operator work?
        }
}

impl CanHold for Item {
    fn add_to_inventory(&self, player: &Player) -> Result<(), InventoryError>; {
            Ok(player.inventory.add(&self)?) 
        }
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            Inventory::new(),
            HashMap::<EquipmentSlot, EquipmentSlotStatus>new(),
            // can I create an anonymous HashMap like this?  Is there a better way?
            HashMap::<Stat, u16>::from_iter(IntoIter::new([
                (Stat::Strength, 5),
                (Stat::Dexterity, 5),
                (Stat::Stamina, 5),
                (Stat::Energy, 5)
            ]))
        }
    }
}

// what should these return? how can i handle any errors returned?
fn equip<T: CanEquip>(equipment: &T, player: &Player) {
    equipment.equip(&player);
}   

fn add_to_inventory<T: CanHold>(item: &T, player: &Player) {
    item.add_to_inventory(&player);
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_player_fields() {
        let p = Player::new(String::from("John"));
        assert_eq!(p.name, "John");
        assert_eq!(p.inventory, Inventory::new());
        assert_eq!(p.equipped, HashMap::<EquipmentSlot, EquipmentSlotStatus>new());
        assert_eq!(p.stats, HashMap::<Stat, u16>::from_iter(IntoIter::new([
            (Stat::Strength, 5),
            (Stat::Dexterity, 5),
            (Stat::Stamina, 5),
            (Stat::Energy, 5)
        ])));
    }
}