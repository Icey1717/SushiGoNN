use rulinalg::matrix::{Matrix, BaseMatrixMut};
use rand::Rng;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const MUTATE_AMOUNT: f32 = 0.01;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct NeuralNetworkProperties
{
    // A vector of inputs relating to which cards the player has in their hand.
    pub input_node_count: usize,

    // A vector of hidden layer node activations.
    pub hidden_node_count: usize,

    // A vector of outputs used to determine what the nn does. 
    pub output_node_count: usize,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct NeuralNetworkData
{
    // A matrix of weights connecting the input layer to the hidden layer.
    weights_ih: Vec<f32>,

    // A matrix of weights connecting the hidden layer to the output.
    weights_ho: Vec<f32>,

    // A matrix of biases for the hidden layer.
    bias_h: Vec<f32>,
    
    // A matrix of biases for the output layer.
    bias_o: Vec<f32>,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct NeuralNetworkSerializable
{
    properties: NeuralNetworkProperties,
    data: NeuralNetworkData
}

#[derive(Clone)]
pub struct NeuralNetwork
{
    // A unique ID used to track this network. 
    id: usize,

    // Properties of the neural network
    properties: NeuralNetworkProperties,

    // A matrix of weights connecting the input layer to the hidden layer.
    weights_ih: Matrix<f32>,

    // A matrix of weights connecting the hidden layer to the output.
    weights_ho: Matrix<f32>,

    // A matrix of biases for the hidden layer.
    bias_h: Matrix<f32>,
    
    // A matrix of biases for the output layer.
    bias_o: Matrix<f32>,
}



impl NeuralNetwork
{
    pub fn feed_forward(&self, input: &[f32]) -> Vec<f32>
    {
        //---- Generating the activations of the hidden nodes.

        assert!(input.len() == self.properties.input_node_count, "The number of input nodes does not equal the expected number of nodes.");
        
        // Convert the inputs to a matrix.
        let input_matrix = Matrix::new(input.len(), 1, input);

        // Calculate the activations of the hidden layer, based of the weights and biases connecting each input node to each hidden node.
        let mut hidden = &self.weights_ih * &input_matrix;
        hidden = &hidden + &self.bias_h;

        // Apply the sigmoid function to every element. This flattens the activation to a number between 0.0 and 1.0.
        hidden = hidden.apply(&sigmoid);

        //---- Generating the activations of the output layer.
        // Our hidden layer is already an appropriate matrix so no need to convert it.

        // Calculate the activations of the output layer, based of the weights and biases connecting each hidden node to each output node.
        let mut output = &self.weights_ho * &hidden;
        output = &output + &self.bias_o;
        
        // Apply the sigmoid function to every element. This flattens the activation to a number between 0.0 and 1.0.
        output = output.apply(&sigmoid);

        // Create a new vector to hold the outputs.
        let mut output_vector = Vec::new();

        // Push all the values in the output matrix (which is 8 by 1) into a vector.
        for x in output.iter_mut()
        {
            output_vector.push(*x);
        }

        return output_vector;
    }

    pub fn clone(&self) -> NeuralNetwork
    {
        //return NeuralNetwork{id: self.id, input_node_count: self.input_node_count, hidden_node_count: self.hidden_node_count, output_node_count: self.output_node_count, weights_ih: self.weights_ih.clone(), weights_ho: self.weights_ho.clone(), bias_h: self.bias_h.clone(), bias_o: self.bias_o.clone()};
        return NeuralNetwork{id: self.id, properties: self.properties.clone(), weights_ih: self.weights_ih.clone(), weights_ho: self.weights_ho.clone(), bias_h: self.bias_h.clone(), bias_o: self.bias_o.clone()};
    }

    pub fn get_id(&self) -> usize
    {
        return self.id;
    }

    pub fn set_id(&mut self, new_id: usize)
    {
        self.id = new_id;
    }

    fn get_save_data(&self) -> String
    {
        // Copy all the matrix data into vectors.
        let wih = self.weights_ih.data().clone();
        let who = self.weights_ho.data().clone();
        let bh = self.bias_h.data().clone();
        let bo = self.bias_o.data().clone();

        // Construct the properties and data
        let properties = NeuralNetworkProperties{input_node_count: self.properties.input_node_count, hidden_node_count: self.properties.hidden_node_count, output_node_count: self.properties.output_node_count};
        let data = NeuralNetworkData{weights_ih: wih, weights_ho: who, bias_h: bh, bias_o: bo};

        // Create a neural network serializable struct
        let nns = NeuralNetworkSerializable{properties: properties, data: data};

        // Serialize and return.
        return serde_json::to_string(&nns).unwrap();
    }

    pub fn save_nn_to_file(&self, file_name: String)
    {
        let s = file_name + ".txt";
        let path = Path::new(&s);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            display,
                            why.description()),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(self.get_save_data().as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    pub fn mutate(&mut self)
    {
        self.bias_o = self.bias_o.clone().apply(&mutate);
        self.bias_h = self.bias_h.clone().apply(&mutate);

        self.weights_ho = self.weights_ho.clone().apply(&mutate);
        self.weights_ih = self.weights_ih.clone().apply(&mutate);
    }
}

pub fn new_neural_network(id: usize, in_input_node_count: usize, in_hidden_node_count: usize, in_output_node_count: usize) -> NeuralNetwork
{
    let mut rng = rand::thread_rng();

    let initial_weights_ih: Vec<f32> = (0..in_input_node_count * in_hidden_node_count).map(|_| 
    {
        rng.gen_range(-1.0, 1.0)
    }).collect();

    let initial_weights_ho: Vec<f32> = (0..in_hidden_node_count * in_output_node_count).map(|_| 
    {
        rng.gen_range(-1.0, 1.0)
    }).collect();

    let weights_ih = Matrix::new(in_hidden_node_count, in_input_node_count, initial_weights_ih);
    let weights_ho = Matrix::new(in_output_node_count, in_hidden_node_count, initial_weights_ho);

    let initial_bias_h: Vec<f32> = (0..in_hidden_node_count).map(|_| 
    {
        rng.gen_range(-1.0, 1.0)
    }).collect();

    let initial_bias_o: Vec<f32> = (0..in_output_node_count).map(|_| 
    {
        rng.gen_range(-1.0, 1.0)
    }).collect();

    let bias_h = Matrix::new(in_hidden_node_count, 1, initial_bias_h);
    let bias_o = Matrix::new(in_output_node_count, 1, initial_bias_o);

    let properties = NeuralNetworkProperties{input_node_count: in_input_node_count, hidden_node_count: in_hidden_node_count, output_node_count: in_output_node_count};

    return NeuralNetwork{id: id, properties: properties, weights_ih: weights_ih, weights_ho: weights_ho, bias_h: bias_h, bias_o: bias_o};
}

pub fn load_save_data(id: usize, data_string: String) -> NeuralNetwork
{
    let loaded_data: NeuralNetworkSerializable = serde_json::from_str(&data_string).unwrap();

    let properties = NeuralNetworkProperties{input_node_count: loaded_data.properties.input_node_count, hidden_node_count: loaded_data.properties.hidden_node_count, output_node_count: loaded_data.properties.output_node_count};

    let weights_ih = Matrix::new(properties.hidden_node_count, properties.input_node_count, loaded_data.data.weights_ih);
    let weights_ho = Matrix::new(properties.output_node_count, properties.hidden_node_count, loaded_data.data.weights_ho);
    let bias_h = Matrix::new(properties.hidden_node_count, 1, loaded_data.data.bias_h);
    let bias_o = Matrix::new(properties.output_node_count, 1, loaded_data.data.bias_o);

    return NeuralNetwork{id: id, properties: properties, weights_ih: weights_ih, weights_ho: weights_ho, bias_h: bias_h, bias_o: bias_o};
}

fn sigmoid(x: f32) -> f32
{
	return 1.0 / (1.0 + (-x).exp());
}

fn mutate(x: f32) -> f32
{
    let mut rng = rand::thread_rng();
    return x + rng.gen_range(MUTATE_AMOUNT * -1.0, MUTATE_AMOUNT);
}

pub fn load_nn_from_file(file_name: &str) -> NeuralNetwork
{
	let path = Path::new(file_name);
	let mut data = String::new();
    let mut f = File::open(&path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return load_save_data(0, data);
}