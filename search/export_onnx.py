from sentence_transformers import SentenceTransformer
import torch

# Check if MPS device is available
device = torch.device("mps") if torch.has_mps else torch.device("cpu")

model = SentenceTransformer("Salesforce/SFR-Embedding-2_R")

huggingface_model = model._first_module().auto_model.to(
    device
)  # Move model to MPS or CPU
tokenizer = model.tokenizer

text = "tonight standing on an earth that shines"
inputs = tokenizer(text, return_tensors="pt")
inputs = {
    key: value.to(device) for key, value in inputs.items()
}  # Move inputs to MPS or CPU

# Export to ONNX with dynamic axes
torch.onnx.export(
    huggingface_model,  # HuggingFace model (PyTorch)
    (
        inputs["input_ids"],
        inputs["attention_mask"],
    ),  # Inputs moved to the correct device
    "model.onnx",  # Output ONNX file
    export_params=True,
    opset_version=20,
    input_names=["input_ids", "attention_mask"],  # Names for the input layers
    output_names=["output"],  # Name for the output layer
    dynamic_axes={
        "input_ids": {
            0: "batch_size",
            1: "sequence_length",
        },  # Allow batch size and sequence length to vary
        "attention_mask": {
            0: "batch_size",
            1: "sequence_length",
        },  # Dynamic axes for attention mask
        "output": {0: "batch_size"},  # Dynamic axis for output batch size
    },
)
