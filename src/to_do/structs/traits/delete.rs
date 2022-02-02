pub trait  Delete {
    fn delete(&self,title:&str){
        println!("delete {} is created",title)
    }
}