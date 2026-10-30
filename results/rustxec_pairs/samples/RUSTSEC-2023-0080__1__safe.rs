pub fn transpose_inplace<T: Copy>(buffer: &mut [T], scratch: &mut [T], width: usize, height: usize) {
	assert_eq!(width.checked_mul(height), Some(buffer.len()));
	assert_eq!(core::cmp::max(width, height), scratch.len());

	let gcd = StrengthReducedUsize::new(num_integer::gcd(width, height));
	let a = StrengthReducedUsize::new(height / gcd);
	let b = StrengthReducedUsize::new(width / gcd);
	let a_inverse = multiplicative_inverse(a.get(), b.get());
	let strength_reduced_height = StrengthReducedUsize::new(height);

	let index_fn = |x, y| x + y * width;

	if gcd.get() > 1 {
		for x in 0..width {
			let column_offset = (x / b) % strength_reduced_height;
			let wrapping_point = height - column_offset;

			// wrapped rotation -- do the "right half" of the array, then the "left half"
	        for y in 0..wrapping_point {
	            scratch[y] = buffer[index_fn(x, y + column_offset)];
	        }
	        for y in wrapping_point..height {
	            scratch[y] = buffer[index_fn(x, y + column_offset - height)];
	        }

	        // copy the data back into the column
	        for y in 0..height {
	            buffer[index_fn(x, y)] = scratch[y];
	        }
	    }
	}

	// Permute the rows
	{
		let row_scratch = &mut scratch[0..width];
		
		for (y, row) in buffer.chunks_exact_mut(width).enumerate() {
		    for x in 0..width {
		    	let helper_val = if y <= height + x%gcd - gcd.get() { x + y*(width-1) } else { x + y*(width-1) + height };
		    	let (helper_div, helper_mod) = StrengthReducedUsize::div_rem(helper_val, gcd);

		    	let gather_x = (a_inverse * helper_div)%b + b.get()*helper_mod;
		        row_scratch[x] = row[gather_x];
		    }

		    row.copy_from_slice(row_scratch);
		}
	}

	// Permute the columns
	for x in 0..width {
		let column_offset = x % strength_reduced_height;
		let wrapping_point = height - column_offset;

		// wrapped rotation -- do the "right half" of the array, then the "left half"
	    for y in 0..wrapping_point {
	        scratch[y] = buffer[index_fn(x, y + column_offset)];
	    }
	    for y in wrapping_point..height {
            scratch[y] = buffer[index_fn(x, y + column_offset - height)];
        }

        // Copy the data back to the buffer, but shuffle it as we do so
	    for y in 0..height {
	    	let shuffled_y = (y * width - (y / a)) % strength_reduced_height;
	        buffer[index_fn(x, y)] = scratch[shuffled_y];
	    }
	}
}