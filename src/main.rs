

mod rconsole;
mod queue;
use  queue::Queue;

struct Struct {
    b: i32,
}

struct StructTest {
    c: i32,
    b: i32,
}


fn main() {

    let mut q = Queue::create();

    q.push('1');
    q.push('2');
    q.push('3');

    match q.pop() {
        Some(c) => rconsole::info(&c.to_string()),
        None =>  {
            rconsole::info("pop is empty")
        }
    }

    // rconsole::info("asd");
    

    // let is_serve = |x| -> bool { x % 2 == 0 } ;
    // is_serve(2);
    
    
    // string_str();
    // split_test();
    // build_vector();
    // i32_max_check_test();
    // let mut a = "aaa";
    // println!("Hello, world! {}", a);
    // let (z,x,c,v,b);
    // (z,x) = (4,3);
    // [c, .., v, _] = [1,2,3,4,5,6,7];

    // Struct {b, ..} = Struct { b: 4 }; // 需要类型完全相同
    // // Struct {b , ..} = StructTest {c: 98, b: 4};

    // let _list = [z,x,c,v,b];
    // assert_eq!([4,3,1,6,4], _list);

    // a = "aa";
    // println!("a {} ", a);
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn i32_max_check_test() {
    let big_val = std::i32::MAX;
    // let x = big_val + 1;
    // println!("x val {}", x);
}

fn split_test() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(10);
    println!("split_test {}", head);
    println!("split_test {}", tail);

    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt)
        }
    }
    let _vec = vec![0.0, 0.1, 0.2];
    print(&_vec);
}

/**
 * 字符串解释
 */
fn string_str() {
    let mut noodles = "noodles".to_string();
    let oodles = &mut noodles[1..];
    let poodles = "(￣(工)￣)";

    println!("{}", oodles);
    println!(
        "{} len:{} count:{}",
        poodles,
        poodles.len(),           // len 获取字符字节长度
        poodles.chars().count()  // chars().count() 获取字符量（字数）
    );
}


fn quicksort<T: Ord + std::fmt::Debug>(slice: &mut [T]){
     //TODO: 实现快速排序

     if slice.len() < 1 {
        return ;
     }

     let pivot_index = partition(slice);
     
     quicksort(&mut  slice[.. pivot_index]);
     quicksort(&mut  slice[pivot_index+1 ..])
}

fn partition<T: Ord + std::fmt::Debug>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot = len - 1;

    let mut i = 0 ;

    for j in 0..len - 1 {

        if slice[j] <= slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }

    

    slice.swap(i, pivot);

    i
}