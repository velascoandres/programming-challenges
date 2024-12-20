pub fn minimum_coins(prices: Vec<i32>) -> i32 {
    let mut coins = 0;
    let total = prices.len();
    let mut current_index = 1;

    loop {
        let limit = current_index + current_index;
        if limit >= total {
            break;
        }
        let index = get_min(&prices, current_index, limit);

        coins += prices[index];

        current_index = index + 1;
    }

    coins + prices[0]
}

fn get_min(prices: &[i32], start_indx: usize, end_indx: usize) -> usize {
    let mut min_index = start_indx;
    for j in start_indx..=end_indx {
        if j < end_indx {
            let current = prices[min_index];
            let next = prices[j + 1];

            if current >= next {
                min_index = j + 1;
            }
        }
    }

    min_index
}

fn main() {
    assert_eq!(
        minimum_coins(vec![
            1, 37, 19, 38, 11, 42, 18, 33, 6, 37, 15, 48, 23, 12, 41, 18, 27, 32
        ]),
        37
    );
    assert_eq!(minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45]), 39);
    assert_eq!(minimum_coins(vec![1, 10, 1, 1]), 2);
    assert_eq!(minimum_coins(vec![3, 1, 2]), 4);
}
