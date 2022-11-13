fn mean(v: &Vec<i32>) -> f32 {
    let mut x: f32 = 0.0;
    for i in v.iter() {
        x += i.clone() as f32;
    }
    x = x/(v.len() as f32);
    x
}

fn median(vec: &Vec<i32>) -> f32 {
    let x: f32;
    let mut v = vec.clone();
    v.sort();
    if v.len() % 2 == 0 {
        let le1 = (v.len() / 2) - 1;
        let le2 = v.len() / 2;
        x = (v.get(le1).unwrap().clone() as f32 + v.get(le2).unwrap().clone() as f32) / 2.0;
    }else {
        let index: usize = ((((v.len() as f32 )/2.0) + 0.5) - 1.0) as usize;
        x = v.get(index).unwrap().clone() as f32;
    }
    x
}

fn mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut mode = HashMap::new();
    for i in v.iter(){
        let count = mode.entry(i.clone()).or_insert(0);
        *count += 1;
    }
    let mut bigger: i32 = mode.get(&v[0]).unwrap().clone() as i32;
    for (key, value) in &mode {
        if value.clone() as i32 > bigger{
            bigger = key.clone();
        }
    }
    bigger
}

fn main() {
    let ve = vec![1, 2, 2, 4, 3, 5, 9];
    let vo = vec![4, 2, 0, 0, 9, 1];
    println!("{0}", mean(&ve));
    println!("{0}", median(&ve));
    println!("{0}", mode(&ve));
    println!("{0}", mean(&vo));
    println!("{0}", median(&vo));
    println!("{0}", mode(&vo));
}
