use rand::{self, seq::SliceRandom};

#[derive(Debug, Copy, Clone)]
pub struct Prisoner {
    pub number: usize,
}

impl Prisoner {
    fn visit_room(&self, room: &Room, nprisoners: usize) -> bool {
        assert!(nprisoners >= 100);
        let first_box = room.boxes[self.number - 1]; // gets the box with our number on it
        let mut prev = first_box;
        let mut success = false;
        for _ in 0..nprisoners / 2 {
            let curr = room.boxes[prev.slip - 1];
            if curr == first_box {
                success = true;
                break;
            }
            prev = curr;
        }
        success
    }
}

pub struct Room {
    pub boxes: Vec<NumberedBox>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NumberedBox {
    pub number: usize,
    pub slip: usize,
}

pub fn simulate(nprisoners: usize) -> (usize, usize) {
    assert!(nprisoners >= 100);
    let mut numbers: Vec<_> = (1..=nprisoners).collect();
    numbers.shuffle(&mut rand::thread_rng());
    let boxes = (1..=nprisoners)
        .map(|number| NumberedBox {
            number,
            slip: numbers[number - 1],
        })
        .collect();
    let room = Room { boxes };
    let prisoners = (1..=nprisoners).map(|number| Prisoner { number });
    let results: Vec<_> = prisoners
        .map(|prisoner| prisoner.visit_room(&room, nprisoners))
        .collect();
    let nwon = results.iter().filter(|&&res| res).count();
    let nlost = results.len() - nwon;
    (nwon, nlost)
}
