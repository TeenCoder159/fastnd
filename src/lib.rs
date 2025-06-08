mod tests;

pub fn zeroes(count: usize) -> Vec<isize> {
    vec![0; count]
}

pub fn linspace(start: isize, end: isize, step: isize) -> Vec<isize> {
    if step == 0 {
        return vec![];
    }

    let mut output: Vec<isize> = vec![];

    let mut min = start;
    let mut max = end;

    if step < 0 {
        min = end;
        max = start;
    }

    for i in min..max {
        if (i - start) % step == 0 {
            output.push(i);
        }
    }

    output
}
