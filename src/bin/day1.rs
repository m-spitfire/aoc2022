use anyhow::Result;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./data/day1.input")?;
    let lines = file.lines();
    let mut result_vec = Vec::new();
    let mut index = 0;
    let mut first_time = true;
    for line in lines {
        if !line.is_empty() {
            if first_time {
                result_vec.push(line.parse::<u32>()?);
                first_time = false;
            } else {
                result_vec[index] += line.parse::<u32>()?;
            }
        } else {
            first_time = true;
            index += 1;
        }
    }
    result_vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}", result_vec[0] + result_vec[1] + result_vec[2]);
    println!("{:?}", result_vec.iter().max());
    Ok(())
}
