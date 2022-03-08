
## Architecture

[784] Input MNIST vector --> [196] Layer 1 --> [49] Hidden Layer --> [10] Output Layer

**Layer 1**

- Input Dimensions: [784]
- Output Dimensions: [196]
- Weight Dimensions: [784,196]
- Bias Dimensions: [10]
- Activation Function: Softmax

**Hidden Layer**

- Input Dimensions: [196]
- Output Dimensions: [49]
- Weight Dimensions: [196,49]
- Bias Dimensions: [10]
- Activation Function: Softmax

**Output Layer**

- Input Dimensions: [49]
- Output Dimensions: [10]
- Weight Dimensions: [49,10]
- Bias Dimensions: [10]
- Activation Function: Softmax

The cost function used is the cross entropy cost function. Apparently it works well for NN's that use softmax activation functions (which this one does).

WIP
