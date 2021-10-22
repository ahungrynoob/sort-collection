use sort_lib::async_lib;

pub async fn bubble_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::bubble_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}

pub async fn selection_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::selection_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}

pub async fn insertion_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::insertion_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}

pub async fn heap_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::heap_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}

pub async fn merge_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::merge_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}

pub async fn quick_sort(mut arr: Vec<i32>, function_name: &str) {
    let start = time::Instant::now();
    async_lib::quick_sort(&mut arr).await;
    let elapsed_ms = start.elapsed().whole_milliseconds();
    println!("{} duration: {:?}", function_name, elapsed_ms);
}
