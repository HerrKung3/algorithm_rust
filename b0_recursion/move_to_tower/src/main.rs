//汉洛塔问题
//src起始杆 des目标杆 mid中间杆
//抽象过程
//1借助目标杆将height-1个盘子移动中间杆
//2将最后一个盘子移动到目标杆
//3借助起始杆将height-1个盘子从中间杆移动到目标杆
fn move_to_tower(height: u32, src: &str, des: &str, mid: &str) {
    if height >= 1 {
        //借助目标杆将height-1个盘子移动中间杆
        move_to_tower(height-1, src, mid, des);
        println!("Moving disk from {} to {}", src, des);
        //借助起始杆将height-1个盘子从中间杆移动到目标杆
        move_to_tower(height-1, mid, des, src)
    }
}

fn main() {
    // move_to_tower(1, "A", "B", "C");
    // move_to_tower(2, "A", "B", "C");
    // move_to_tower(3, "A", "B", "C");
    move_to_tower(4, "A", "B", "C");
}