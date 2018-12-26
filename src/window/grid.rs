pub struct Grid {
    width: u32,
    height: u32,
    filled: Vec<u32>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Grid {
            width,
            height,
            filled: vec![],
        }
    }

    pub fn find_space(&mut self, width: u32, height: u32) -> (u32, u32) {
        let width = width.min(self.width);
        let height = height.min(self.height);
        let mut start_index: Option<u32> = None;
        let mut max_column: u32 = 0;
        for (i, f) in self.filled.iter().enumerate() {
            let i = i as u32;
            let space_left = self.width - f;
            if space_left >= width {
                let si = match start_index {
                    Some(si) => si,
                    None => {
                        start_index = Some(i);
                        i
                    }
                };

                max_column = max_column.max(*f);
                if i - si + 1 >= height {
                    self.fill(max_column, si, width, height);
                    return (max_column, si);
                }
            } else if start_index.is_some() {
                start_index = None;
                max_column = 0
            }
        }

        let space_y = self.filled.len() as u32;
        self.filled.append(&mut vec![0; height as usize]);
        self.fill(0, space_y, width, height);
        (0, space_y)
    }

    pub fn fill(&mut self, x: u32, y: u32, width: u32, height: u32) {
        for i in y..(y + height) {
            assert!(self.filled[i as usize] <= x);
            self.filled[i as usize] = x + width;
        }
    }
}
