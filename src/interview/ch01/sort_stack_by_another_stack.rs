pub fn sort_stack(stack: &mut Vec<i32>) {
    let mut local = Vec::new();

    while let Some(cur) = stack.pop() {
        while !local.is_empty() && (local.last().unwrap()) > &cur {
            stack.push(local.pop().unwrap())
        }

        local.push(cur)
    }

    while !local.is_empty() {
        stack.push(local.pop().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::sort_stack;

    #[test]
    fn test_sort_stack() {
        assert_eq!(vec![5, 4, 3, 2, 1], {
            let mut origin = vec![3, 5, 4, 2, 1];
            sort_stack(&mut origin);
            origin
        });
    }

}
