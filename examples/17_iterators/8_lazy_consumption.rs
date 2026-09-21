#[test]
fn experiment_without_consumption() {
    let mut calls = 0;
    {
        let _pipeline = (1..=8).map(|number| {
            calls += 1;
            number * 10
        });
        println!("pipeline constructed");
    }
    println!("closure calls: {calls}");
    let expected_calls: usize = todo!("Predict the closure calls without consuming the pipeline");
    assert_eq!(calls, expected_calls);
}

#[test]
fn experiment_partial_consumption() {
    let mut visited = Vec::new();
    let (first, batch, next) = {
        let mut pipeline = (1..=8)
            .map(|number| {
                visited.push(number);
                println!("map visits {number}");
                number * 10
            })
            .filter(|number| number % 20 == 0);
        println!("pipeline constructed");
        let first = pipeline.next();
        println!("first: {first:?}");
        let batch: Vec<_> = pipeline.by_ref().take(1).collect();
        println!("batch: {batch:?}");
        let next = pipeline.next();
        println!("next: {next:?}");
        (first, batch, next)
    };
    println!("visited: {visited:?}");

    let expected_first: Option<i32> = todo!("What does the first next() return?");
    let expected_batch: Vec<i32> = todo!("Predict the batch collected after the first next()");
    let expected_next: Option<i32> = todo!("Predict next() after collecting the batch");
    let expected_visited: Vec<i32> = todo!("Predict the visited inputs in order");
    assert_eq!(first, expected_first);
    assert_eq!(batch, expected_batch);
    assert_eq!(next, expected_next);
    assert_eq!(visited, expected_visited);
}
