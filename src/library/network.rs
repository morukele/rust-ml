use matrices::{Matrix, Activation};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation<'a>,
}
impl Network<'_> {
    pub fn new(layers: Vec<usize>, activation: Activation) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], layers[i]))
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Invalid number of inputs");
        }

        // keeps track of the current layer
        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
                .multiply(&current)
                .unwrap()
                .add(&self.biases[i])
                .unwrap()
                .map(self.activation.function);
            self.data.push(current.clone());
        }

        Ok(current.data[0].to_owned())
    }

    pub fn back_propagate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {

    }
}
