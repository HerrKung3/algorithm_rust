//coins: 面额种类，amount：找零金额，min_coins：从零到找零值之间所有金额找零所需最小纸币数量
fn dp_mc(coins: &[u32], amount: u32, min_coins: &mut[u32]) -> u32 {
    //动态收集找零值从1到amount的最优解
    for denm in 1..=amount {
        //最差的情况是全部用1找零
        let mut min_coin_num = denm;
        for c in coins.iter().filter(|&c|*c <= denm).collect::<Vec<&u32>>() {
            //找零c面额之后，剩余的待找零金额index，先去min_coins中查找，如果找到意味着当前面额的最小找零值就是min_coins[index] + 1
            let index = (denm - c) as usize;
            let coin_num = 1 + min_coins[index];
            if coin_num < min_coin_num {
                min_coin_num = coin_num;
            }
        }
        //循环将1-amount之间的所有面额所需最小找零纸币数存起来
        min_coins[denm as usize] = min_coin_num;
    }
    return min_coins[amount as usize];
}

fn main() {
    let amount = 81u32;
    let coins = [1, 5, 10, 20, 50];

    let mut min_coins: [u32; 82] = [0; 82];
    let coin_num = dp_mc(&coins, amount, &mut min_coins);
    println!("Refund for {:?} need {} coins", amount, coin_num); 
}