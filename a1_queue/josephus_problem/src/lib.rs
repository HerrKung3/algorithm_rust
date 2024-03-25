use a1_queue::Quenue;

//约瑟夫问题，一列人循环报数至num，每次报num的人淘汰，直到只剩一人
//队列第一个人报数，然后弹出重新加入队尾，报数如果是num则弹出，不再加入队列
//时间复杂度O(N^2)
pub fn josephus_problem(names: Vec<&str>, num: usize) -> &str {
    let mut q = Quenue::new(names.len());
    for name in names {
        _ = q.enqueue(name);
    }

    //循环报数直至队列只剩一人
    while q.size() > 1 {
        for _i in 1..num {
            let name = q.dequeue().unwrap();
            let _ = q.enqueue(name);
        }
        //删除第num个人
        let _ = q.dequeue();
    }
    q.dequeue().unwrap()
}

//递归解法，时间复杂度O(N)
pub fn josephus_problem1(names: Vec<&str>, num: usize) -> &str {
    names[helper(names.len(), num)]
}

fn helper(n: usize, num: usize) -> usize {
    if n == 1 {
        return 0;
    }else {
        return (helper(n-1, num) + num) % n;
    }
}