use to_do::{ItemTypes, to_do_factory, structs::traits::create::Create};

mod to_do;
mod state;
// use to_do::structs::done::Done;
// use to_do::structs::pending::Pending;

fn main() {
    // let done = Done::new("shopping");
    // println!("{}",done.super_struct.title);
    // println!("{}",done.super_struct.status);

    // let pending = Pending::new("laundry");
    // println!("{}",pending.super_struct.title);
    // println!("{}",pending.super_struct.status);

    // println!("{:?}",pending);
    // println!("{:?}",done);
    let to_do_item:Result<ItemTypes,&'static str> = to_do_factory("pending", "make");
    match  to_do_item.unwrap() {
        ItemTypes::Pending(item) =>item.create(&item.super_struct.title),
        ItemTypes::Done(item) => {
            println!("this is pending title: {}",item.super_struct.title)
        },
    }
}
