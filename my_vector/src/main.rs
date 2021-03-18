use std::ptr::NonNull;
use std::alloc::{alloc, realloc, dealloc, Layout};
 
#[derive(Debug, PartialEq)]
pub struct DropItem {
    value: i32,
}
 
impl Drop for DropItem {
    fn drop(&mut self) {
        println!("  Dropping {}", self.value);
    }
}
 
#[derive(Debug)]
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}
 
impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }
 
    fn resize_capacity(&mut self) {
        if std::mem::size_of::<T>() == 0 {
            panic!("no zero sized type");
        }
 
        let pre_allocated_size = ((self.len() / 4) + 1) * 4;
        if self.capacity != pre_allocated_size {
            unsafe {
                if self.capacity == 0 {
                    let layout = Layout::array::<T>(pre_allocated_size).expect("layout error");
                    let ptr = alloc(layout) as *mut T;
                    self.ptr = NonNull::new(ptr).expect("could not allocate");
                } else {
                    let layout = Layout::from_size_align_unchecked(
                        std::mem::size_of::<T>() * pre_allocated_size,
                        std::mem::align_of::<T>(),
                    );
                    let ptr = realloc(self.ptr.as_ptr() as *mut u8, layout, std::mem::size_of::<T>() * pre_allocated_size);
                    self.ptr = NonNull::new(ptr as *mut T).expect("realloc failed");
                }
            }
            self.capacity = pre_allocated_size;
        }
    }
 
 
    fn push(&mut self, value: T) {
        self.resize_capacity();
        unsafe { self.ptr.as_ptr().add(self.len()).write(value); }
        self.len += 1;
    }
 
    fn get(&self, index: usize) -> Option<&T> {
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }
 
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
 
        let item = unsafe { self.ptr.as_ptr().add(self.len).read() };
        self.len -= 1;
        Some(item)
    }
 
    fn len(&self) -> usize {
        self.len
    }
 
    fn capacity(&self) -> usize {
        self.capacity
    }
}
 
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(
                std::slice::from_raw_parts_mut(self.ptr.as_ptr(),
                self.len)
            );
            let layout = Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );           
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
        self.ptr = NonNull::dangling();
        self.len = 0;
        self.capacity = 0;
    }
}
 
pub struct Iter<'a, T> {
    vector: &'a MyVec<T>,
    index: usize,
}
 
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.len() {
            return None;
        }
 
        let item = unsafe { &*self.vector.ptr.as_ptr().add(self.index) };
        self.index += 1;
        Some(item)
    }
}
 
impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            vector: self,
            index: 0,
        }
    }
}
 
pub struct OwnerIter<T> {
    vector: MyVec<T>,
}
 
impl<T> Iterator for OwnerIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vector.pop()
    }
}
 
impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = OwnerIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        OwnerIter {
            vector: self,
        }
    }
}
 
 
fn main() {
    let mut my_vec = MyVec::new();
    my_vec.push(DropItem {value: 0});
    my_vec.push(DropItem {value: 1});
    my_vec.push(DropItem {value: 2});
    my_vec.push(DropItem {value: 3});
    my_vec.push(DropItem {value: 4});
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec.capacity(), 8);
 
    println!("Loop by get()");
    for index in 0..my_vec.len() {
        println!("{:?}", my_vec.get(index).unwrap());
    }
 
    println!("Loop by ref interator 1");
    let my_vec_ref = &my_vec;
    for item in my_vec_ref.into_iter() {
        println!("{:?}", item);
    }
 
    println!("Loop by owned interator 2");
    for item in my_vec.into_iter() {
        println!("{:?}", item);
    }
    println!("Loop by owned interator 2 done");
 
    println!("end of main");
}
 
 
#[test]
fn my_vec() {
    let mut my_vec = MyVec::new();
    my_vec.push(DropItem {value: 0});
    my_vec.push(DropItem {value: 1});
    my_vec.push(DropItem {value: 2});
    my_vec.push(DropItem {value: 3});
    my_vec.push(DropItem {value: 4});
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec.capacity(), 8);
 
    println!("Loop by get()");
    for index in 0..my_vec.len() {
        println!("{:?}", my_vec.get(index).unwrap());
    }
 
    println!("Loop by ref interator 1");
    let my_vec_ref = &my_vec;
    for item in my_vec_ref.into_iter() {
        println!("{:?}", item);
    }
 
    println!("Loop by owned interator 2");
    for item in my_vec.into_iter() {
        println!("{:?}", item);
    }
    println!("Loop by owned interator 2 done");
 
    println!("end of main");
}