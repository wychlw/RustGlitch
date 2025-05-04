import os
import urllib.parse
import requests
import json
from zhipuai import ZhipuAI

PROMPT = """You are a rust professor aimed in finding bugs in rust compilers. You need to give rust code which makes rust compiler throw Internal Compiler Error. You can use any nightly feature and items in std crate. All features has been enabled, no need to enable them. Generate codes as strange as possible, and contains various structures and features. Just contains exectuable codes."""


class ModelFuzzerBase:
    def __init__(self, args):
        """
        Initializes the ModelFuzzerBase with the provided arguments.

        Args:
            args: Arguments for the model fuzzer.
        """

    def infill(self, code_prefix: str, code_suffix: str) -> str:
        """
        Infills the code between the prefix and suffix.

        Args:
            code_prefix (str): The prefix of the code.
            code_suffix (str): The suffix of the code.

        Returns:
            str: The infilled code.
        """
        raise NotImplementedError("Subclasses should implement this method.")


class ModelFuzzerZhipuAI(ModelFuzzerBase):
    def __init__(self, args):
        API_KEY = os.getenv("API_KEY")
        if API_KEY is None:
            raise ValueError("API_KEY environment variable is not set.")
        self.client = ZhipuAI(api_key=API_KEY)

    def infill(self, code_prefix: str, code_suffix: str) -> str:
        r_code_prefix = "// " + PROMPT + "\n" + code_prefix
        resp = self.client.chat.completions.create(
            model="codegeex-4",
            messages=[
                # {
                #     "role": "system",
                #     "content": "You are a rust professor aimed in finding bugs in rust compilers. You need to give rust code which makes rust compiler throw Internal Compiler Error. You can use any nightly feature and items in std crate. All features has been enabled. Generate codes as strange as possible, and contains various structures and features."
                # },
            ],
            extra={
                "target": {
                    "path": "fuzzer_code.rs",
                    "language": "rust",
                    "code_prefix": r_code_prefix,
                    "code_suffix": code_suffix,
                },
                "contexts": []
            },
            top_p=0.8,
            temperature=0.95,
            max_tokens=1024,
            stop=["<|endoftext|>", "<|user|>",
                  "<|assistant|>", "<|observation|>"]
        )
        code_inner = resp.choices[0].message.content
        code = code_prefix + code_inner + code_suffix
        return code


class ModelFuzzerLlama(ModelFuzzerBase):
    def __init__(self, args):
        url = os.getenv("LLAMA_URL", "http://localhost:8080")
        self.url = urllib.parse.urljoin(url, "completion")

    def infill(self, code_prefix: str, code_suffix: str) -> str:
        fim_prefix = '<|fim_prefix|>'
        fim_suffix = '<|fim_suffix|>'
        fim_middle = '<|fim_middle|>'

        msg = [
            fim_prefix,
            code_prefix,
            fim_suffix,
            code_suffix,
            fim_middle,
        ]
        msg = "".join(msg)
        res = requests.post(self.url, json={"prompt": msg}, timeout=10)
        msg = res.json()
        code_middle = msg["content"]
        code = code_prefix + code_middle + code_suffix
        return code


class ModelFuzzer:
    def __init__(self, args):
        """
        Magic class init different model based on args.model
        """
        if args.model.lower() == "zhipuai":
            self.fuzzer = ModelFuzzerZhipuAI(args)
        elif args.model.lower() == "llama":
            self.fuzzer = ModelFuzzerLlama(args)
        else:
            raise ValueError(f"Unknown model: {args.model}")

    def infill(self, code_prefix: str, code_suffix: str) -> str:
        """
        Infills the code between the prefix and suffix.

        Args:
            code_prefix (str): The prefix of the code.
            code_suffix (str): The suffix of the code.

        Returns:
            str: The infilled code.
        """
        return self.fuzzer.infill(code_prefix, code_suffix)


if __name__ == "__main__":
    fuzzer = ModelFuzzer({
        "model": "llama"
    })
    infilled_code = fuzzer.infill("", "")
    print(infilled_code)
