fn main(){
    let v=vec![1,2,3,4,5];
    let third=&v[2];
    println!("The third element is {}",third);
    let third=v.get(2);
    match third {
        Some(third)=>println!("The third element is {third}"),
        None=>println!("There is no third element"),
    }
}