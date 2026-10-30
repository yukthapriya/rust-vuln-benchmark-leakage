import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
import pandas as pd
from transformers import AutoModel, AutoTokenizer
from sklearn.metrics import accuracy_score, f1_score
from tqdm import tqdm

# ---------------------------------------------------------
# 1. Load hybrid dataset
# ---------------------------------------------------------
df = pd.read_pickle("hybrid_dataset.pkl")

# Drop rows missing ML features
df = df.dropna(subset=["input_ids", "attention_mask"])

# Convert label to binary
df["label_binary"] = df["label"].apply(lambda x: 1 if x == "vulnerable" else 0)

# ---------------------------------------------------------
# 2. Prepare static feature matrix
# ---------------------------------------------------------
STATIC_FEATURES = [
    "n_unsafe", "n_raw_ptr", "n_lifetimes", "n_loops",
    "n_functions", "n_branches", "n_lines", "n_chars"
]

df[STATIC_FEATURES] = df[STATIC_FEATURES].fillna(0)

# ---------------------------------------------------------
# 3. Dataset class
# ---------------------------------------------------------
class HybridDataset(Dataset):
    def __init__(self, df):
        self.df = df

    def __len__(self):
        return len(self.df)

    def __getitem__(self, idx):
        row = self.df.iloc[idx]

        static_feats = torch.tensor(row[STATIC_FEATURES].values, dtype=torch.float32)
        input_ids = torch.stack(row["input_ids"]).squeeze(1)
        attention_mask = torch.stack(row["attention_mask"]).squeeze(1)
        label = torch.tensor(row["label_binary"], dtype=torch.long)

        return input_ids, attention_mask, static_feats, label

# ---------------------------------------------------------
# 4. Model definitions
# ---------------------------------------------------------
class StaticOnlyModel(nn.Module):
    def __init__(self, input_dim=8, hidden=32):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(input_dim, hidden),
            nn.ReLU(),
            nn.Linear(hidden, 2)
        )

    def forward(self, static_feats):
        return self.net(static_feats)


class HybridModel(nn.Module):
    def __init__(self, transformer_name="microsoft/codebert-base", static_dim=8, hidden=64):
        super().__init__()
        self.encoder = AutoModel.from_pretrained(transformer_name)
        self.static_net = nn.Sequential(
            nn.Linear(static_dim, hidden),
            nn.ReLU()
        )
        self.classifier = nn.Linear(self.encoder.config.hidden_size + hidden, 2)

    def forward(self, input_ids, attention_mask, static_feats):
        enc = self.encoder(input_ids=input_ids, attention_mask=attention_mask)
        cls = enc.last_hidden_state[:, 0, :]  # CLS token

        static_vec = self.static_net(static_feats)

        combined = torch.cat([cls, static_vec], dim=1)
        return self.classifier(combined)

# ---------------------------------------------------------
# 5. Training loop
# ---------------------------------------------------------
def train_model(model, dataloader, epochs=3, lr=2e-5):
    device = "cuda" if torch.cuda.is_available() else "cpu"
    model = model.to(device)

    optimizer = torch.optim.AdamW(model.parameters(), lr=lr)
    criterion = nn.CrossEntropyLoss()

    for epoch in range(epochs):
        model.train()
        losses = []

        for input_ids, attention_mask, static_feats, labels in tqdm(dataloader):
            input_ids = input_ids.to(device)
            attention_mask = attention_mask.to(device)
            static_feats = static_feats.to(device)
            labels = labels.to(device)

            optimizer.zero_grad()

            if isinstance(model, StaticOnlyModel):
                logits = model(static_feats)
            else:
                logits = model(input_ids, attention_mask, static_feats)

            loss = criterion(logits, labels)
            loss.backward()
            optimizer.step()

            losses.append(loss.item())

        print(f"Epoch {epoch+1} Loss: {sum(losses)/len(losses):.4f}")

    return model

# ---------------------------------------------------------
# 6. Train models
# ---------------------------------------------------------
dataset = HybridDataset(df)
loader = DataLoader(dataset, batch_size=4, shuffle=True)

print("Training Static-Only Model...")
static_model = StaticOnlyModel()
static_model = train_model(static_model, loader)

print("Training Hybrid Model...")
hybrid_model = HybridModel()
hybrid_model = train_model(hybrid_model, loader)

# ---------------------------------------------------------
# 7. Save models
# ---------------------------------------------------------
torch.save(static_model.state_dict(), "static_only_model.pt")
torch.save(hybrid_model.state_dict(), "hybrid_model.pt")

print("Training complete!")
