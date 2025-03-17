import os
import logging
import random

from tqdm import tqdm
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer, get_scheduler
from torch.utils.data import TensorDataset, DataLoader, Dataset

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def Args():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('-e', '--epochs', type=int, default=10)
    parser.add_argument('-b', '--batch_size', type=int, default=16)
    parser.add_argument('-lr', '--learning_rate', type=float, default=1e-5)
    parser.add_argument('-i', '--input', type=str, help='input dir', nargs='+')
    parser.add_argument('-m', '--model', type=str,
                        help='model path', default='./model')
    parser.add_argument('-s', '--save', type=str,
                        help='save path', default='./model_out')
    parser.add_argument('--layers', type=int, default=15)
    args = parser.parse_args()
    return args


class CodeGeexDataset(Dataset):
    def __collect_file_one_dir(self, input_dir):
        codes = []
        for root, _, files in os.walk(input_dir):
            for file in files:
                if file.endswith('.rs'):
                    with open(os.path.join(root, file), 'r', encoding="utf-8") as f:
                        codes.append(f.read())

        return codes

    def __generate_train_data(self, input_dirs):
        codes = []
        for input_dir in input_dirs:
            codes.extend(self.__collect_file_one_dir(input_dir))
        return codes

    def __gen_tokens(self, tokenizer, code_prefix="", code_middle="", code_suffix=""):
        prefix = \
            f"""
[gMASK]<sop>
<|system|>
You are a rust professor aimed in finding bugs in rust compilers. You need to give rust code which makes rust compiler throw Internal Compiler Error. You can use any nightly feature and items in std crate. Generate codes as strange as possible, and contains various structures and features.
You shall only generate code and nothing else. 
<|user|>
###PATH:ice_gen.rs
###LANGUAGE:Rust
###MODE:BLOCK
<|code_suffix|>{code_suffix}<|code_prefix|>{code_prefix}<|code_middle|><|assistant|>
"""
        prefix_tokens = tokenizer.encode(
            text=prefix, padding=True, truncation=True, add_special_tokens=True
        )
        prefix_len = len(prefix_tokens)
        other_tokens = tokenizer.encode(
            text=code_middle, padding=True, truncation=True, add_special_tokens=True
        )
        other_tokens = other_tokens + [tokenizer.eos_token_id]
        res = prefix_tokens + other_tokens
        return (res, prefix_len)

    def __init__(self, input_dirs, tokenizer, args):
        codes = self.__generate_train_data(input_dirs)

        datas = []
        for code in codes:
            (tokens, prefix_len) = self.__gen_tokens(
                tokenizer,
                code_prefix="",
                code_middle=code,
                code_suffix=""
            )
            labels = [-100] * prefix_len + tokens[prefix_len:]
            datas.append((tokens, labels))

            code_split_point = random.randrange(1, len(code))

            (tokens, prefix_len) = self.__gen_tokens(
                tokenizer,
                code_prefix=code[:code_split_point],
                code_middle=code[code_split_point:],
                code_suffix=""
            )
            labels = [-100] * prefix_len + tokens[prefix_len:]
            datas.append((tokens, labels))

        self.datas = datas

    def __len__(self):
        return len(self.datas)

    def __getitem__(self, idx):
        return self.datas[idx]


class DataCollator(object):
    def __init__(self, tokenizer):
        self.tokenizer = tokenizer
        self.pad_token_id = tokenizer.pad_token_id

    def __call__(self, batch):
        lengths = [
            len(i[0]) for i in batch
        ]
        max_len = max(lengths)

        tokens = []
        labels = []
        for i in batch:
            token = i[0]
            label = i[1]

            padding_len = max_len - len(token)
            token = token + [self.pad_token_id] * padding_len
            label = label + [-100] * padding_len

            tokens.append(token)
            labels.append(label)

        return {
            "input_ids": torch.tensor(tokens, dtype=torch.long),
            "labels": torch.tensor(labels, dtype=torch.long)
        }


def save_model(model, args):
    os.makedirs(args.save, exist_ok=True)
    model.save_pretrained(args.save)
    logger.info("model saved to %s", args.save)


def train(model, tokenizer, dataset, device, args):

    for name, param in model.named_parameters():
        try:
            idx = int(name.split('.')[3])
        except:
            continue
        if idx < args.layers:
            param.requires_grad = False
        else:
            param.requires_grad = True

    model = model.to(device)
    model.train()

    collator = DataCollator(tokenizer)
    loader = DataLoader(dataset, collate_fn=collator,
                        batch_size=args.batch_size, shuffle=True)

    optimizer = torch.optim.AdamW(model.parameters(), lr=args.learning_rate)
    lr_scheduler = get_scheduler(
        name='linear', optimizer=optimizer, num_warmup_steps=0,
        num_training_steps=len(loader) * args.epochs
    )

    for epoch in tqdm(range(args.epochs)):
        model.train()

        losses = 0.
        optimizer.zero_grad()
        for batch in tqdm(loader):
            inputs = batch
            inputs = {k: v.to(device) for k, v in inputs.items()}
            outputs = model(**inputs)
            loss = outputs.loss
            losses += loss.item()
            loss.backward()

            optimizer.step()
            lr_scheduler.step()
            optimizer.zero_grad()

        logger.info("epoch %d, loss: %f", epoch, losses)


def main(args):
    if torch.cuda.is_available():
        device = 'cuda'
    # elif torch.npu.is_available():
        # device = 'npu'
    else:
        device = 'cpu'
    model_path = args.model

    model = AutoModelForCausalLM.from_pretrained(
        model_path, trust_remote_code=True)
    tokenizer = AutoTokenizer.from_pretrained(
        model_path, trust_remote_code=True)
    logger.info("model loaded from %s", model_path)

    logger.info("device: %s", device)

    logger.info("loading codes from %s", args.input)
    dataset = CodeGeexDataset(args.input, tokenizer, args)
    logger.info("train data loaded")
    logger.info("train data size: %d", len(dataset))

    train(model, tokenizer, dataset, device, args)


if __name__ == '__main__':
    a = Args()
    main(a)
