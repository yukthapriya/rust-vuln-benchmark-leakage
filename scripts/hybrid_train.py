import pickle
import torch
import torch.nn as nn
import torch.optim as optim
from transformers import AutoModel
from sklearn.metrics import classification_report
import os

# ============================================================
# CONFIG
# ============================================================

DATA_DIR = "ml_ready_dataset"
TRAIN_PATH = os.path.join(DATA_DIR, "train.pkl")
TEST_PATH = os.path.join(DATA_DIR, "test.pkl")
MODEL_NAME = "microsoft/codebert-base"
MODEL_SAVE_PATH = "hybrid_model.pt"

STATIC_FEATURES = [
    "n_unsafe",
    "n_raw_ptr",
    "n_lifetimes",
    "n_loops",
    "n_functions",
    "n_branches",
    "n_lines",
    "n_chars",
]

# ============================================================
# LOAD DATA
# ============================================================

print("Loading ML-ready dataset...")

with open(TRAIN_PATH, "rb") as f:
    train_df = pickle.load(f)

with open(TEST_PATH, "rb") as f:
    test_df = pickle.load(f)

print(f"Train samples: {len(train_df)}")
print(f"Test samples: {len(test_df)}")

# Extract tensors
X_train_ids = torch.stack(train_df["input_ids"].tolist())
X_train_mask = torch.stack(train_df["attention_mask"].tolist())
X_train_static = torch.tensor(train_df[STATIC_FEATURES].values.astype("float32"))
y_train = torch.tensor((train_df["label"] == "vulnerable").astype(int).values)

X_test_ids = torch.stack(test_df["input_ids"].tolist())
X_test_mask = torch.stack(test_df["attention_mask"].tolist())
X_test_static = torch.tensor(test_df[STATIC_FEATURES].values.astype("float32"))
y_test = torch.tensor((test_df["label"] == "vulnerable").astype(int).values)

# ============================================================
# MODEL DEFINITION
# ============================================================

class HybridModel(nn.Module):
    def __init__(self, model_name, static_dim, hidden_dim=128, num_classes=2):
        super().__init__()
        self.encoder = AutoModel.from_pretrained(model_name)
        codebert_dim = self.encoder.config.hidden_size

        # Combined classifier
        self.classifier = nn.Sequential(
            nn.Linear(codebert_dim + static_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, num_classes)
        )

    def forward(self, ids, mask, static_feats):
        out = self.encoder(input_ids=ids, attention_mask=mask)
        cls = out.last_hidden_state[:, 0, :]  # CLS token
        combined = torch.cat([cls, static_feats], dim=1)
        return self.classifier(combined)

model = HybridModel(MODEL_NAME, static_dim=len(STATIC_FEATURES))
criterion = nn.CrossEntropyLoss()
optimizer = optim.Adam(model.parameters(), lr=2e-5)

# ============================================================
# TRAINING LOOP
# ============================================================

print("\nTraining Hybrid CodeBERT + Static model...")

EPOCHS = 3
batch_size = 8

for epoch in range(EPOCHS):
    model.train()
    perm = torch.randperm(X_train_ids.size(0))

    total_loss = 0
    for i in range(0, X_train_ids.size(0), batch_size):
        idx = perm[i:i+batch_size]

        batch_ids = X_train_ids[idx]
        batch_mask = X_train_mask[idx]
        batch_static = X_train_static[idx]
        batch_y = y_train[idx]

        optimizer.zero_grad()
        logits = model(batch_ids, batch_mask, batch_static)
        loss = criterion(logits, batch_y)
        loss.backward()
        optimizer.step()

        total_loss += loss.item() * len(batch_y)

    avg_loss = total_loss / len(X_train_ids)
    print(f"Epoch {epoch+1}/{EPOCHS} - Loss: {avg_loss:.4f}")

# ============================================================
# EVALUATION
# ============================================================

print("\nEvaluating on test set...")

model.eval()
with torch.no_grad():
    logits = model(X_test_ids, X_test_mask, X_test_static)
    preds = torch.argmax(logits, dim=1).numpy()

print("\n==== HYBRID MODEL REPORT ====")
print(classification_report(y_test, preds, target_names=["safe", "vulnerable"]))

# ============================================================
# SAVE MODEL
# ============================================================

torch.save(model.state_dict(), MODEL_SAVE_PATH)
print(f"\nHybrid model saved to {MODEL_SAVE_PATH}")
