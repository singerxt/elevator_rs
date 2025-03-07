use amethyst::{
    core::Named,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Door, DoorEntry, DoorState};
use hierarchy::components::Child;
use physics::components::{Collider, Direction};
use player::components::{Player, PlayerState};

pub struct DoorEntryCollisionSystem;

impl<'s> System<'s> for DoorEntryCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Child>,
        ReadStorage<'s, DoorEntry>,
        WriteStorage<'s, Door>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut players,
            children,
            door_entries,
            mut doors,
            colliders,
            directions,
            names,
        ) = data;

        // check if a player is facing the same direction and is idle
        for (_, player, player_collider, player_direction, _name) in
            (&entities, &mut players, &colliders, &directions, &names).join()
        {
            if player.state == PlayerState::Idling {
                for (_, child, door_entry_collider, _door_entry, door_direction) in
                    (&entities, &children, &colliders, &door_entries, &directions).join()
                {
                    // check that user is facing opposite to door
                    if player_collider.is_overlapping_with(door_entry_collider, false)
                        && player_direction.x != door_direction.x
                    {
                        // get the door
                        for (door_entity, door) in (&entities, &mut doors).join() {
                            if child.parent == door_entity {
                                if door.state == DoorState::Closed {
                                    println!("Opening door...");
                                    door.state = DoorState::Open;
                                    player.state = PlayerState::EnteringRoom;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
