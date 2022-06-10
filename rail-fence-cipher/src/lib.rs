pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let num_rails = self.0 as usize;
        let mut shape = vec![Vec::new(); num_rails];

        // Populate the shape
        self.zig_zag(text, |i, ch| shape[i].push(ch));

        shape.into_iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let num_rails = self.0 as usize;
        let mut rail_len = vec![0; num_rails];

        // Calculate the length of the rails.
        // Example: For cipher="WECRLTEERDSOEEFEAOCAIVDEN",
        // rail_len=[7, 12, 6]
        self.zig_zag(cipher, |i, _| rail_len[i] += 1);

        // Calculate the starting indices of the rails
        // within the cipher text.
        // Example: For cipher="WECRLTEERDSOEEFEAOCAIVDEN",
        // rail_len=[0, 19, 25] (each index is the cumulative sum
        // of the values on its left).
        let mut start = 0;
        for i in &mut rail_len {
            let tmp = *i;
            *i = start;
            start += tmp;
        }

        // Decode using the starting indices of the rails.
        // 1st character of plain text is the 1st character
        // of rail 1, 2nd character of plain text is the
        // 1st character of rail 2, and so on.
        // As we read from a rail, we increment the starting
        // index.
        // This is akin to merging k sorted lists.
        let chars: Vec<char> = cipher.chars().collect();
        let mut text = String::new();
        self.zig_zag(cipher, |i, _| {
            text.push(chars[rail_len[i]]);
            rail_len[i] += 1;
        });

        text
    }

    // Iterate over the rails in a zig-zag manner, and invoke
    // the given function for each rail and character.
    fn zig_zag<F>(&self, text: &str, mut f: F)
    where
        F: FnMut(usize, char),
    {
        let num_rails = self.0 as usize;
        let mut step = -1_i32;
        let mut rail_idx = 0_i32;

        for ch in text.chars() {
            let j = rail_idx as usize;
            f(j, ch);
            if j == 0 || j == num_rails - 1 {
                step *= -1;
            }
            rail_idx += step;
        }
    }
}
