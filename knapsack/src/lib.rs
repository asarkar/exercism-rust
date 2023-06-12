use std::cmp;

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    /* dp[i][j] is the maximum value obtainable by choosing
     * the first i items not exceeding total weight j.
     */
    let mut dp: Vec<Vec<u32>> = Vec::with_capacity(items.len());
    for _ in 0..=items.len() {
        dp.push(vec![0; (1 + max_weight) as usize]);
    }

    for i in 1..=items.len() {
        for j in 1..=max_weight as usize {
            /* Include the item at index 'i' if its weight is not
             * more than the value. In this case, we include its value
             * plus whatever value we get from the remaining weight
             * and from remaining items.
             */
            let w = items[i - 1].weight as usize;
            if w <= j {
                dp[i][j] = dp[i - 1][j - w] + items[i - 1].value;
            }
            /* Exclude the item at index 'i'. In this case, we will
             * take whatever value we get from the sub-array
             * excluding this item.
             */
            dp[i][j] = cmp::max(dp[i][j], dp[i - 1][j]);
        }
    }
    *dp.last().and_then(|x| x.last()).unwrap()
}
