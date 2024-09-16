from sentence_transformers import SentenceTransformer

model = SentenceTransformer("Salesforce/SFR-Embedding-2_R")


def get_detailed_instruct(task_description: str, query: str) -> str:
    return f"Instruct: {task_description}\nQuery: {query}"


# Each query must come with a one-sentence instruction that describes the task
task = "Given a short haiku as a query, retrieve haiku that use similar vocabulary, imagery, or themes"
queries = [
    get_detailed_instruct(task, "tonight standing on an earth that shines"),
    get_detailed_instruct(task, "our kind, the corner makers"),
    get_detailed_instruct(task, "street corner tree corner"),
]
# No need to add instruction for retrieval documents
passages = [
    "who lets the land into the sunlight",
    "which plant or animal gave us our love of rectangles",
]

query_embeddings = model.encode(queries)
print(query_embeddings)
passage_embeddings = model.encode(passages)
scores = model.similarity(query_embeddings, passage_embeddings) * 100
# print(scores.tolist())
# [[40.13203811645508, 25.032546997070312], [15.00684642791748, 39.937339782714844]]
