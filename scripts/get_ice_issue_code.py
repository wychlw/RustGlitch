import os
import requests
import json
from typing import TypedDict


class IssueWithCode(TypedDict):
    id: int
    code: str


def get_rust_issue_list(page=1, token=None):
    url = "https://api.github.com/repos/rust-lang/rust/issues"

    headers = {
        "Accept": "application/vnd.github.v3+json",
        "X-GitHub-Api-Version": "2022-11-28"
    }
    if token:
        headers["Authorization"] = f"Bearer {token}"

    query = {
        "state": "all",
        "labels": "I-ICE",
        "page": f"{page}"
    }

    res = requests.get(url, headers=headers, params=query, timeout=10)
    res.raise_for_status()
    data = res.json()
    return data


def map_issue_to_body(issue):
    return issue["body"]


def map_issue_to_id(issue):
    return issue["number"]


def map_issue_body_to_code(issue_body):
    CODE_BEGIN = "```Rust"
    CODE_END = "```"
    s1 = issue_body.split(CODE_BEGIN)
    if len(s1) < 2:
        return ""
    s2 = s1[1].split(CODE_END)
    if len(s2) < 1:
        return ""
    code = s2[0]
    return code


def map_issue_to_store(issue):
    code = map_issue_body_to_code(map_issue_to_body(issue))
    if not code:
        return None
    return IssueWithCode(
        id=map_issue_to_id(issue),
        code=code
    )


def main(args):
    issues_with_code = []
    page = args.page

    while len(issues_with_code) < args.count:
        issues = get_rust_issue_list(page)
        page += 1
        if not issues:
            break
        issues_with_code += list(filter(None, map(map_issue_to_store, issues)))

    print("Total issues with code:", len(issues_with_code))
    print("To page:", page)

    if args.output:
        with open(args.output, "w", encoding="utf-8") as f:
            json.dump(issues_with_code, f)

    if args.dir:
        os.makedirs(args.dir, exist_ok=True)
        for issue in issues_with_code:
            with open(f"{args.dir}/issue_{issue["id"]}.rs", "w", encoding="utf-8") as f:
                f.write(issue["code"])
                f.write(f"\n\n\\\\[Issue: {issue['id']}]\n\n")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--token", type=str, help="Github token")
    parser.add_argument("-o", "--output", type=str, help="Output file")
    parser.add_argument("-p", "--page", type=int,
                        help="From page", default=1, )
    parser.add_argument("-c", "--count", type=int,
                        help="Count of issues", default=60)
    parser.add_argument("-d", "--dir", type=str,
                        help="Output rust source code directory")
    args = parser.parse_args()
    main(args)
