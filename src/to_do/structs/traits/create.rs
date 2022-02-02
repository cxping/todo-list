pub trait  Create{

    fn create(&self,title:&str){
        println!("create {} is created",title)
    }
}