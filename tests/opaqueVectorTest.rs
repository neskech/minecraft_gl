#![allow(non_snake_case)]

use minecraft_gl::util::opaqueVector::OpaqueVector;

#[path = "../src/main.rs"]
mod minecraft_gl;


#[test]
fn SimpleTest() {
    let size = std::mem::size_of::<i32>();
    let mut vec = OpaqueVector::New::<i32>(1);

    vec.Push(1);
    assert_eq!(*vec.Get::<i32>(0), 1);
    assert_eq!(vec.GetLength(), 1);
    assert_eq!(vec.GetCapacity(), 1);

    vec.Push(2);
    assert_eq!(*vec.Get::<i32>(1), 2);
    assert_eq!(vec.GetLength(), 2);
    assert_eq!(vec.GetCapacity(), 2);
}

#[derive(PartialEq, Eq, Debug)]
struct Testing(i32);

impl Drop for Testing {
    fn drop(&mut self) {
        println!("Dropped!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
    }
}

#[test]
fn DropTest() {
    let mut vec = OpaqueVector::New::<Testing>(1);

    vec.Push(Testing(1));
    assert_eq!(vec.GetLength(), 1);
    assert_eq!(vec.GetCapacity(), 1);

    for _ in 0..3 {
        vec.Push(Testing(1));
    }

    let t = vec.Pop::<Testing>();
    let x = vec.Pop::<Testing>();
    vec.DropElements::<Testing>();
}
