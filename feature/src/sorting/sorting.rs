use core::time;
use std::{time::{Instant, Duration}, thread::sleep};

use rand::Rng;
use serde::Serialize;

pub fn num_generator(size: isize) -> Vec<i32>{
    let mut numbers:Vec<i32> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let num = rng.gen_range(i32::MIN..i32::MAX);
        numbers.push(num);
    }
    numbers
}

#[derive(Serialize)]
pub struct Heap {
    heap_size: usize,
    array: Vec<i32>
}

impl Heap {
    fn new() -> Self {
        Heap { heap_size: 0, array: Vec::new() }
    }
    /// .
    fn sort(&mut self, nums: Vec<i32>) {
        self.heap_size = nums.len();
        self.array = nums;
        self.build_max_heap(self.heap_size);
        let mut i = (self.heap_size - 1) as i32;
        while i >= 0 {
            (self.array[1], self.array[i as usize]) = (self.array[i as usize], self.array[1]);
            self.heap_size -= 1;
            self.max_heapify(1);
            i-=1;
        }

    }

    fn build_max_heap(&mut self, length: usize) {
        let divide:f32 = length as f32/2.0;
        let mut num = divide.floor() as i32;
        while num > 0  {
            self.max_heapify(num as usize);
            num -= 1
        }
    }

    fn max_heapify(&mut self, num: usize) {
        let left = 2*num;
        let right = 2*num+1;
        let mut largest = 0;

        if left < self.heap_size && self.array[left] > self.array[num]{
            largest = left;
        } else {
            largest = num
        }

        if right < self.heap_size && self.array[right] > self.array[largest] {
            largest = right;
        }

        if largest != num {
            (self.array[num], self.array[largest]) = (self.array[largest], self.array[num]);
            self.max_heapify(largest);
        }

    }
}

pub async fn Heapsort(nums: Vec<i32>) -> Duration {
    let start = Instant::now();
    let mut x = Heap::new();
    x.sort(nums);
    println!("{:#?}", x.array);
    start.elapsed()
}
pub async fn MergeSort() -> Duration {
    let start = Instant::now();
    sleep(time::Duration::from_secs(1));
    start.elapsed()
}

pub async fn InsertionSort() -> Duration {
    let start = Instant::now();
    sleep(time::Duration::from_secs(4));
    start.elapsed()
}

