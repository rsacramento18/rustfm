fn practice(nums: Vec<usize>, idx: usize) -> usize {
    return nums.get(idx).unwrap_or(&idx) * 5;
}

fn main() {

    let num = practice(vec![1,2,3], 4);
    println!("{:?}", num);

}
