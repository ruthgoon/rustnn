use std::mem;
pub struct Layer{
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>
}
impl Layer {
    /// Feed forward step
    /// Takes an input vector, multiplies it with the current weights vector and passes
    /// it through a sigmoid function. It then flattens the 2D vector --> 1D vector along
    /// the vertical (x??) plane
    pub fn forward(&mut self, input_vector : Vec<f32>){
        
        // we must copy over the weight and bias vectors into a new memory address so we
        // can use and mutate them here
        let weight_buffer = mem::take(&mut self.weight);
        let bias_buffer = mem::take(&mut self.bias);
        let mut state_vector = self._dot(input_vector, weight_buffer);
        state_vector = self._add(state_vector, bias_buffer);
        state_vector = self._softmax(state_vector);

        
    }

    /// Backpropagation step
    /// Computes the loss function, returning it so the gradient can be calculated
    pub fn backward(){}

    /// Calculates the dot product.
    /// Takes a 1D input vector along with a 2D vector, multiplies them together
    /// and then returns the resultant 2d vector
    fn _dot(&self, input : Vec<f32>, weights : Vec<Vec<f32>>) -> Vec<Vec<f32>>{
        assert_eq!(input.len(), weights.len());
        let mut dot = vec![vec![0.0;input.len()]; weights.len()];
        for r in 0..input.len(){
            for c in 0..weights.len(){
                for j in 0..weights[c].len(){
                    dot[r][c] = input[r] * weights[c][j];
                }
            }
        }
        return dot;
    }

    /// Adds the bias vector to the current state vector
    /// The state vector is the dot product of the input vector and weight vector.
    fn _add(&self, state_vector: Vec<Vec<f32>>, bias_vector: Vec<f32>) -> Vec<Vec<f32>>{
        let mut output_vector = vec![vec![0.0; bias_vector.len()]; state_vector[0].len()];
        for i in 0..bias_vector.len(){
            for j in 0..state_vector[i].len(){
                output_vector[i][j] = state_vector[i][j] + bias_vector[i];
            }
        }
        return output_vector;
    }

    /// Returns the maximum value.
    /// Given a 2d vector, this function returns the maximum value as a float32
    fn _max(&self, input : Vec<Vec<f32>>) -> f32{
        let mut max = 0.0;
        for i in 0..input.len(){
            for j in 0..input[i].len(){
                if input[i][j] > max{
                    max = input[i][j]
                }
            }
        }
        return max;
    }

    /// Calcuates The softmax function for each value in the input vector
    fn _softmax(&self, input : Vec<Vec<f32>>) -> Vec<Vec<f32>>{
        let mut softmax : Vec<Vec<f32>> = vec![vec![0.0; input[0].len()]; input.len()];
        let e : f32 = 2.718281;

        for i in 0..input.len(){
            // compute the sum for the entire row
            let sum : f32 = input[i].iter().map(|&x| e.powf(x)).sum();
            for j in 0..input[i].len(){
                // come back and compute e^x / sum(exp)
                softmax[i][j] = e.powf(input[i][j]) / sum;
            }
        }
        return softmax;
    }
}

pub struct Model{
    layers: Vec<Layer>
}
impl Model{

}