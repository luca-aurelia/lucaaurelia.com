import onnxruntime as ort
from sentence_transformers import SentenceTransformer

# Load the SentenceTransformer tokenizer
model = SentenceTransformer("Salesforce/SFR-Embedding-2_R")
tokenizer = model.tokenizer


def get_detailed_instruct(task_description: str, query: str) -> str:
    return f"Instruct: {task_description}\nQuery: {query}"


# Prepare input text and tokenize
task = "Given a short haiku as a query, retrieve haiku that use similar vocabulary, imagery, or themes"
query = get_detailed_instruct(task, "tonight standing on an earth that shines")
inputs = tokenizer(query, return_tensors="pt")

# Log token IDs and sequence length
print(f"Token IDs (input_ids): {inputs['input_ids']}")
print(f"Sequence length: {inputs['input_ids'].shape[1]}")

onnx_inputs = {
    "input_ids": inputs["input_ids"].numpy(),
    "attention_mask": inputs["attention_mask"].numpy(),
}

# Load ONNX model
ort_session = ort.InferenceSession("model.onnx")

# Run the ONNX model with prepared inputs
outputs = ort_session.run(None, onnx_inputs)

print(f"Output shape: {outputs[0].shape}")
print(f"Output values: {outputs[0]}")

# Extract the embedding of the last token (ignoring padding)
last_token_embedding = outputs[0][:, -1, :]  # Shape: (1, 4096)
print(f"Last token embedding shape: {last_token_embedding.shape}")

# Print the model output
print(f"Last token embedding: {last_token_embedding}")
