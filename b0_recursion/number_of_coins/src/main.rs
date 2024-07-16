// 找零硬币数量最少
//                      1 + numCoins(amount-1)
//                      1 + numCoins(amount-5)
// numCoins(amount) =   1 + numCoins(amount-10)
//                      1 + numCoins(amount-20)
//                      1 + numCoins(amount-50)

fn rec_mc1(coins: &[u32], amount: u32) -> u32 {
    //全用1元硬币时找零硬币数
    let mut min_coins = amount;

    //剩余的找零值恰好有对应面值的硬币，直接返回1
    if coins.contains(&amount) {
        return 1;
    }else {
        //提取符合条件的币种，找零的币值必须小于找零值
        for c in coins.iter().filter(|&c| *c <= amount).collect::<Vec<&u32>>(){
            let num_coins = 1 + rec_mc1(&coins, amount - c);
            //如果num_coins小于min_coins，则更新min_coins
            if num_coins < min_coins {
                min_coins = num_coins;
            }
        }
    }
    min_coins
}

fn rec_mc2(coins: &[u32], amount: u32, min_coins: &mut[u32]) -> u32 {
    //全用1元硬币时找零硬币数
    let mut min_coin_num = amount;

    if coins.contains(&amount) {
        //将找零过程中剩余找零值恰好等于币值的情况记录下来，之后遇见该情况直接返回，避免重复工作
        min_coins[amount as usize] = 1;
        return 1;
    }else if min_coins[amount as usize] > 0 {
        return min_coins[amount as usize];
    }else {
        for c in coins.iter().filter(|&c| *c < amount).collect::<Vec<&u32>>() {
            let num_coins = 1 + rec_mc2(coins, amount-c, min_coins);
            if num_coins < min_coin_num {
                min_coin_num = num_coins;
                //将已经得出来的可以使找零硬币最少情况记录下来
                min_coins[amount as usize] = min_coin_num;
            }
        }
    }
    min_coin_num
}

fn main() {
    // let coins = [1, 5, 10, 20, 50];
    // let amount: u32 = 81;
    // let min_number_coins = rec_mc1(&coins, amount);
    // println!("need refund {} number of coins", min_number_coins);

    let coins = [1, 5, 10, 20, 50];
    let amount: u32 = 81;
    let mut min_coins: [u32; 82] = [0; 82]; 
    let min_number_coins = rec_mc2(&coins, amount, &mut min_coins);
    println!("need refund {} number of coins", min_number_coins);
}