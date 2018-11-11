pub fn get_and_remove_last_element(stack: &mut Vec<i32>) -> i32 {
    let element = stack.pop().unwrap();
    if stack.is_empty() {
        return element;
    }

    let last = get_and_remove_last_element(stack);
    stack.push(element);
    last
}

pub fn reverse(stack: &mut Vec<i32>) {
    if stack.is_empty() {
        return;
    }

    let last = get_and_remove_last_element(stack);
    reverse(stack);
    stack.push(last);
}
