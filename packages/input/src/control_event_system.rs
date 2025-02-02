use crate::controls::ControlMapItem;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum GameEvent {
    ControlActivate(ControlMapItem),
    ControlRelease(ControlMapItem),
    CursorMoved(ControlMapItem),
}

#[derive(Debug)]
struct GameEventWrapped {
    pub id: u64,
    pub event: GameEvent,
}

pub struct GameEventSystem {
    events: Arc<Mutex<Vec<GameEventWrapped>>>,
    consumers_cursors: Arc<Mutex<HashMap<TypeId, u64>>>,
    current_id: AtomicU64,
}

impl GameEventSystem {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::from(vec![])),
            consumers_cursors: Arc::new(Mutex::from(HashMap::new())),
            current_id: AtomicU64::new(0),
        }
    }

    pub fn push(&self, event: GameEvent) {
        let id = self.current_id.fetch_add(1, Ordering::SeqCst) + 1;
        // println!("PUSH {:?} with id = {}", event, id);
        self.events
            .lock()
            .unwrap()
            .push(GameEventWrapped { id, event });
    }

    pub fn get_events<CONSUMER: 'static>(&self) -> Vec<GameEvent> {
        let typ = TypeId::of::<CONSUMER>();
        let mut map = self.consumers_cursors.lock().unwrap();
        let current_cursor = map.get_mut(&typ);
        let cursor = match current_cursor {
            None => {
                map.insert(typ, 0);
                0
            }
            Some(cursor) => cursor.clone(),
        };
        *map.get_mut(&typ).unwrap() = self.current_id.load(Ordering::SeqCst);

        let events = self.events.lock().unwrap();
        let events: Vec<GameEvent> = events
            .iter()
            .filter(|x| x.id > cursor)
            .map(|x| x.event.clone())
            .collect();

        events
    }

    pub fn cleanup(&self) {
        let mut events = self.events.lock().unwrap();
        let cursors = self.consumers_cursors.lock().unwrap();
        let values = cursors.values();
        let mut min_id = u64::MAX;
        for value in values {
            if *value < min_id {
                min_id = *value;
            }
        }
        // println!("Removing where id >= {}", min_id);
        events.retain(|x| x.id > min_id);
    }
}
//
// // Some tests because this code is very sketchy
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::core::game::Game;
//     use crate::core::game_event_system::GameEvent::ControlActivate;
//     #[test]
//     fn it_works() {
//         let mut bus = GameEventSystem::new();
//         // this needs to happen so consumers are registered
//         bus.get_events::<Game>();
//         bus.get_events::<GameEventSystem>();
//
//         bus.push(ControlActivate(ControlMapItem::Pause));
//         bus.push(ControlActivate(ControlMapItem::FlightExit));
//
//         let now_events = bus.get_events::<GameEventSystem>();
//         println!("for <GameEventSystem> {:?}", now_events);
//         assert!(now_events.len() == 2);
//
//         bus.cleanup();
//
//         let now_events = bus.get_events::<GameEventSystem>(); // will filter on event frameid > 1, setting cursor to 2
//         println!("for <GameEventSystem> {:?}", now_events);
//         assert!(now_events.len() == 0);
//
//         bus.push(ControlActivate(ControlMapItem::Use));
//
//         bus.cleanup();
//
//         let now_events = bus.get_events::<GameEventSystem>();
//         println!("for <GameEventSystem> {:?}", now_events);
//         assert!(now_events.len() == 1);
//
//         let now_events = bus.get_events::<Game>();
//         println!("for <Game> {:?}", now_events);
//         assert!(now_events.len() == 3);
//
//         println!("{:?}", bus.events);
//         bus.cleanup();
//         println!("{:?}", bus.events);
//     }
// }
