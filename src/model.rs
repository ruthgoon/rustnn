pub struct Layer{
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>
}
impl Layer {
    /// Feed forward step
    /// Takes an input vector, multiplies it with the current weights vector and passes
    /// it through a sigmoid function. It then flattens the 2D vector --> 1D vector along
    /// the plane
    pub fn forward(&mut self, input_vector : Vec<f32>){
       let dot = self._dot(input_vector, self.weight);
    }

    /// Backpropagation step
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

    /// Calcuates the exp for each x
    fn _exp(&self, input : Vec<Vec<f32>>) -> Vec<Vec<f32>>{
        let mut exp =  vec![vec![0.0; input[0].len()]; input.len()];
        let e : f32 = 2.718281;
        for i in 0..input.len(){
            for j in 0..input[i].len(){
                exp[i][j] = e.powf(input[i][j]);
            }
        }
        return exp;
    }
}

pub struct Model{
    layers: Vec<Layer>
}
impl Model{

}