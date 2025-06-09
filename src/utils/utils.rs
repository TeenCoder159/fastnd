pub fn zeroes(n: usize) -> Vec<isize> {
    vec![0; n]
}

pub fn arange(start: isize, end: isize, step: isize) -> Vec<f64> {
    if step == 0 {
        return vec![];
    }

    let mut min = start;
    let mut max = end;

    if step < 0 {
        min = end;
        max = start;
    }

    let capacity = (max - min) / step;
    let mut output: Vec<f64> = Vec::<f64>::new().alloc(capacity as u32);

    for i in min..max {
        if (i - start) % step == 0 {
            output.push(i as f64);
        }
    }

    output
}

pub fn linspace(start: isize, end: isize, no_of_elements: usize) -> Vec<f64> {
    let step = (end - start) / no_of_elements as isize;

    arange(start, end, step)
}

pub trait VecCreation {
    fn alloc<T>(&self, capacity: u32) -> Vec<T> {
        Vec::with_capacity(capacity as usize)
    }
}

impl<T> VecCreation for Vec<T> {}
