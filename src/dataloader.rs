
pub struct Image{
    label: u8,
    data: Vec<Vec<f32>>
}

pub struct Dataset{
    n: usize,
    items: Vec<Image>
}
/// Reads the MNIST training files, returning a Dataset of Image structs
pub fn read_mnist_training_set(label_file_path : &String, train_file_path : &String) -> Dataset{
    
    let mut train_data_buffer = std::fs::read(&train_file_path).expect("Could not find the train_file_path");
    let mut label_data_buffer = std::fs::read(&label_file_path).expect("Could not find the label_file_path");

    // drain the metadata fluff at the beginning of each training file
    train_data_buffer.drain(0..16);
    label_data_buffer.drain(0..8);

    let (mut lower, mut upper) = (0, 784); // 28 x 28 = 784. the amount of pixels per mnist image
    let mut output = Vec::with_capacity(60000);
    for i in 0..60000{
        let mut pixels = vec![vec![0.0; 28]; 28];
        let(mut r, mut c) = (0, 0);
        if upper > 23520000{
            break;
        }
        for p in train_data_buffer.drain(lower..upper){
            // per pixel
            if c == 28{
                c = 0;
                r += 1;
            }
            pixels[r][c] = norm(p, 0, 255);
            c+=1;
        }
        lower = upper;
        upper += 784;

        output.push(Image{
            label: label_data_buffer[i],
            data: pixels
        });
    }
    return Dataset { n: output.len(), items: output }
}

/// Normalize an integer from 0 - 255 to a 32 bit float
pub fn norm(n : u8, min : u8, max : u8) -> f32{
    let range = max - min;
    return ((n - min) / range).into();
}