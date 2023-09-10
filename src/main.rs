//直接cargo run即可
#[macro_use]
extern crate std;
use std::collections::HashMap;
macro_rules! hash_map{
    ($($key:expr=>$val:expr),*)=>{
        {
            let mut map=HashMap::new();
            $(
                map.insert($key,$val);
            )*
            map
        }
    };
}

//--------------------------------------------------------
use std::ops::Deref;
struct Meta<T>{
    data:T,
    num:u32,
}
impl<T> Meta<T>{
    fn new(x:T)->Meta<T>{
        Meta{
            data:x,
            num:1,
        }
    }
    fn print(&self){
        println!("ref_count:{}",self.num);
    }   
    fn add(&mut self){
        self.num+=1;
    }
    fn minus(&mut self){
        self.num-=1;
        if self.num==0{
            let x = Box::new(self);
            let ptr = Box::into_raw(x);
            let x = unsafe { Box::from_raw(ptr) };
            println!("dropped");
        }
    }
}
struct MyRc<T>{
    ptr:*mut Meta<T>,
}
impl<T> MyRc<T>{
    fn new(x:T)->MyRc<T>{
        let p=Box::new(Meta::new(x));
        MyRc{
            ptr:Box::into_raw(p),
        }
    }
    fn print(&self){
        unsafe{
            (*(self.ptr)).print();
        }
    }
    fn clone(&self)->MyRc<T>{
        unsafe{
            (*(self.ptr)).add();
            MyRc{
                ptr:self.ptr,
            }
        }
    }
    
}
impl<T>Deref for MyRc<T>{
    type Target=T;
    fn deref(&self)->&T{
        unsafe{
            &((*(self.ptr)).data)
        }
    }
}

impl<T>Drop for MyRc<T>{
    fn drop(&mut self){
        unsafe{
            (*(self.ptr)).minus();
        }
    }
}
//------------------------------------------------------------
use std::cell::RefCell;
#[derive(Debug)]
struct SimpleStack<T>{
    stack:RefCell<Vec<T>>,
}
impl<T>SimpleStack<T>{
    fn new()->SimpleStack<T>{
        SimpleStack{
            stack:RefCell::new(Vec::new()),
        }
    }
    fn push(&self,value:T){
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self)->Option<T>{
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    println!("-------------Test1--------------");
    let map=hash_map!{
      "one"=>1,
      "two"=>2,
      "three"=>3  
    };
    println!("{:?}",map);
    println!("-------------Test2--------------");
    {
        let b1=MyRc::new(100);
        b1.print();
        let b2=b1.clone();
        b1.print();
        println!("data:{}",*b1);
        assert_eq!(*b1,*b2,"data not equall");
    }   
    println!("-------------Test3--------------");
    let stack=SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("popped value:{:?}",stack.pop());
    println!("popped value:{:?}",stack.pop());
    stack.push(4);
    println!("popped value:{:?}",stack.pop());
    println!("popped value:{:?}",stack.pop());
    println!("popped value:{:?}",stack.pop());

}




