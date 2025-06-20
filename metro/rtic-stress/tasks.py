#! /usr/bin/env python3
import argparse

parser = argparse.ArgumentParser(
    description="Generates any number of async task functions and spawn calls.")
parser.add_argument("number", metavar="N", type=int,
                    help="Number of tasks to generate")
args = parser.parse_args()

print("""#[inline]
fn spawn_tasks() {""")
for k in range(args.number):
    print(f"    task_{k+1}::spawn().ok().unwrap();")
print("}")
for k in range(args.number):
    print(f"""\n#[task(priority = 1, shared=[display])]
async fn task_{k+1}(cx: task_{k+1}::Context) {{
    test_task(cx.shared.display, {k}, BASE_PERIOD_MS).await
}}""")
