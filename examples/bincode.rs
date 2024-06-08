use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct World(Vec<Entity>);

fn main() {
    let entitis = vec![
        Entity { x: 1.0, y: 2.0 },
        Entity { x: 2.0, y: 3.0 },
        Entity { x: 3.0, y: 4.0 },
        Entity { x: 4.0, y: 5.0 },
    ];
    let target = World(entitis);

    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();

    println!("type World size {}", std::mem::size_of::<World>());
    println!("variable encode size {}", std::mem::size_of_val(&encoded));
    println!("variable encode vec size {}", encoded.len()); // 8 + 4*8

    assert_eq!(target, decoded);
}
