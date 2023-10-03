pub fn nth_fibonacci(num: usize) -> usize {
    // if num <= 1 {
    //     return num;
    // }

    let calc_result = dynamic_fib_calculator(num);
    
    let result = match calc_result.last() {
        Some(x) => x,
        None => panic!("xD"),
    };

    *result
}

pub fn fib_sequence(num: usize) -> Vec<usize> {

    dynamic_fib_calculator(num)

}

fn dynamic_fib_calculator(num: usize) -> Vec<usize> {
    let mut results:Vec<usize> = vec![0, 1];
    if num == 0 {
        return vec![0];
    } else if num == 1 {
        return vec![0, 1];
    } else if num > 1 {
        for n in 2..=num {
            results.push(results[n-2] + results[n-1]);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test1(){
        let result = nth_fibonacci(11);
        assert_eq!(result, 89)
    }
}