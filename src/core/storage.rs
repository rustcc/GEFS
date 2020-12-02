use std::{any::Any, collections::HashMap, any::TypeId};
use rayon::prelude::*;



pub struct GameStorage{
    // TODO: single object
    pub resources: HashMap<TypeId,Vec<Box<dyn Any>>>
}
impl GameStorage{
    pub fn new()->Self{
        Self{
            resources:HashMap::new()
        }
    }
    pub fn add<T:Any>(&mut self,value: Box<dyn Any>){
        let id = TypeId::of::<T>();
        if self.resources.get(&id).is_none() {
            self.resources.insert(id, vec![value]);
        }else{
            self.resources.get_mut(&id).unwrap().push(value);
        }
    }
    // f:返回true的保留
    pub fn retain<T:Any>(&mut self,mut f:Box<dyn FnMut(&T)->bool>) -> Result<(),()>{
        if let Some(r)  = self.resources.get_mut(&TypeId::of::<T>() ){
            r.retain(|x|{
                let casted = x.downcast_ref::<T>().unwrap();
                f(casted)
            });
            Ok(())
        }else{
            Err(())
        }
    }
    pub fn select_by_typeid(&mut self,typeid:&TypeId,f:Box<dyn FnMut(&&Box<dyn Any>)->bool>)->Option<Vec<&Box<dyn Any>>>{
        let v = self.resources.get(typeid)?;
        Some(v.iter().filter(f).collect::<Vec<_>>())
    }
    pub fn select_by_typeid_mut(&mut self,typeid:&TypeId,f:Box<dyn FnMut(&&mut Box<dyn Any>)->bool>)->Option<Vec<&mut Box<dyn Any>>>{
        let v = self.resources.get_mut(typeid)?;
        Some(v.iter_mut().filter(f).collect::<Vec<_>>())
    }
}

#[test]
fn test_storage(){
    let mut g = GameStorage::new();
    g.add::<String>(Box::new("shit".to_owned()));
    g.add::<String>(Box::new("fucking gay".to_owned()));
    g.retain::<String>(Box::new(|x|{
        *x == "shit".to_owned()
    }));
    let v = g.resources.get(&TypeId::of::<String>()).unwrap();
    assert!(v.len() == 1);
    g.add::<u32>(Box::new(1u32));
    g.add::<u32>(Box::new(2u32));
    g.add::<u32>(Box::new(3u32));
    g.add::<u32>(Box::new(4u32));
    g.add::<u32>(Box::new(5u32));
    g.add::<u32>(Box::new(6u32));
    g.add::<u32>(Box::new(7u32));
    g.add::<u32>(Box::new(8u32));
    g.retain::<u32>(Box::new(|x| *x > 4 ));
    let v = g.resources.get(&TypeId::of::<u32>()).unwrap();
    assert!(v.len() == 4);
}