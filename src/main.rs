struct Area {
    pub width: usize,
    pub height: usize,
    pub fields: Vec<bool>,
}

impl Area {
    pub fn new(fields: Vec<bool>, width: usize, height: usize) -> Option<Self> {
        if fields.len() == width * height {
            Some(Area{ width, height,fields})
        } else {
            None
        }
    }

    fn get_weight(&self, pos: usize) -> usize {
        let mut weight = 0;
        let pos_y = pos / self.width;
        let pos_x = pos - pos_y * self.width ;
        let min_x = if pos_x == 0 { pos_x } else { pos_x - 1 };
        let max_x = if pos_x + 1 < self.width { pos_x + 1 } else { pos_x };
        let min_y = if pos_y == 0 { pos_y } else { pos_y - 1 };
        let max_y = if pos_y + 1 < self.height { pos_y + 1 } else { pos_y };
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let check_pos = y * self.width + x;
                if check_pos != pos && self.fields[check_pos] == true {
                    weight += 1;
                }
            }
        }
        weight
    }

    fn next(&mut self) {
        let mut fields = vec![false; self.fields.len()];
        for (pos, val) in self.fields.iter().enumerate() {
            match (self.get_weight(pos), val) {
                (3, _) => fields[pos] = true,
                (2, &true) => fields[pos] = true,
                _ => (),
            }
        }
        self.fields = fields;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.fields[x + y * self.width] {
                    true => print!("X"),
                    false => print!("O"),
                }
            }
            println!()
        }
        println!()
    }
}





fn main() {
    let fields = vec![
        false, false, false, false, false, true, false,
        false, false, false, false, false, true, true,
        false, false, false, false, false, false, false,
        false, false, true, false, false, false, false,
        false, false, true, false, false, false, false,
        false, false, true, false, false, false, false,
        false, false, false, false, false, false, false,
    ];
    // let square = vec![
    //     false, true, false,
    //     false, true, true,
    //     false, false, false
    // ];
    let mut area = Area::new(fields, 7, 7).unwrap();
    area.print();
    area.next();
    area.print();
    area.next();
    area.print();
    area.next();
    area.print();
}
