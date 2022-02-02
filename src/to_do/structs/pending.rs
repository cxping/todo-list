use std::fmt::Display;

use super::{base::Base, traits::{get::Get, delete::Delete, edit::Edit, create::Create}};


#[derive(Debug)]
pub struct Pending{
    pub super_struct:Base
}
impl Pending {
    pub fn new(input_title:&str)->Pending{
        let base :Base = Base::new(input_title,"pending" );
        return Pending { super_struct: base }
    }
}

impl Create for Pending {}
impl  Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
