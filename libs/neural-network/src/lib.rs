struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl Neuron {
    fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }
}
struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}
pub struct Network {
    // activation_function:
    layers: Vec<Layer>,
}
impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(
            inputs,
            |inputs /*Accumulator */, layer /*Element extracted from the iterator*/| {
                layer.propagate(inputs)
            },
        )
    }
}
