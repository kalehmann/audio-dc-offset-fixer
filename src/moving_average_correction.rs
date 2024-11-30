pub struct MovingAverageDCOffsetCorrection<'a, T> {
    moving_average: f64,
    index: usize,
    samples_to_consider: usize,
    samples_for_average: Vec<f64>,
    iterator_consumed: bool,
    reader: Box<dyn Iterator<Item = T> + 'a >,
}

impl<T> MovingAverageDCOffsetCorrection<'_, T>
  where f64: From<T>
{
    pub fn new<'a>(
	mut samples: impl Iterator<Item = T> + 'a,
	mut samples_to_consider: usize,
    ) -> MovingAverageDCOffsetCorrection<'a, T> {
	let mut samples_for_average = Vec::new();
	let mut moving_average = 0.0;
	let mut consumed = false;
	for i in 0..samples_to_consider {
	    match samples.next() {
		Some(s) => {
		    let sample = f64::from(s);
		    samples_for_average.push(sample);
		    moving_average += sample
		},
		None => {
		    consumed = true;
		    samples_to_consider = i;

		    break;
		}
	    }
	}

	MovingAverageDCOffsetCorrection::<T> {
	    reader: Box::new(samples),
	    moving_average: moving_average / samples_to_consider as f64,
	    index: 0,
	    samples_to_consider: samples_to_consider,
	    samples_for_average: samples_for_average,
	    iterator_consumed: consumed,
	}
    }

    fn center(&self) -> usize {
	&self.samples_to_consider / 2
    }

    fn next_corrected_sample(&mut self) -> Option<f64> {
	if self.iterator_consumed {
	    self.index += 1;
	    if self.index == self.samples_to_consider {
		return None
	    }
	    let s = self.samples_for_average[self.index] - self.moving_average;

	    return Some(s)
	}

	let sample = self.reader.next();
	let sample_to_add = match sample {
	    None => {
		self.iterator_consumed = true;
		self.index = self.center() + 1;

		return Some(
		    self.samples_for_average[self.index] - self.moving_average
		)
	    },
	    Some(x) => f64::from(x)
	};
	let write_index = self.index % self.samples_to_consider;
	let sample_to_remove = self.samples_for_average[write_index];
	let difference = sample_to_add - sample_to_remove;

	self.moving_average += difference / self.samples_to_consider as f64;
	self.samples_for_average[write_index] = sample_to_add;

	let read_index = (self.index + self.center()) % self.samples_to_consider;
	let original_sample = self.samples_for_average[read_index];
	self.index += 1;

	return Some(original_sample - self.moving_average)
    }
}

impl Iterator for MovingAverageDCOffsetCorrection<'_, i16>
{
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
	return self.next_corrected_sample().map(|s| s as i16)
    }
}

impl Iterator for MovingAverageDCOffsetCorrection<'_, i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
	return self.next_corrected_sample().map(|s| s as i32)
    }
}


impl Iterator for MovingAverageDCOffsetCorrection<'_, f32>
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
	return self.next_corrected_sample().map(|s| s as f32)
    }
}
