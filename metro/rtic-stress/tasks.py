#! /usr/bin/env python3
import argparse

parser = argparse.ArgumentParser(description="Generates any unumber of embassy task functions and spawn calls.")
parser.add_argument("number", metavar="N", type=int, help="Number of tasks to generate")
args = parser.parse_args()

print("""#[inline]
fn spawn_tasks(spawner: Spawner, display: &'static SharedDisplay) {""")
for k in range(args.number) :
    print(f"    spawner.must_spawn(test_task_{k+1}(display));")
print("}")
for k in range(args.number) :
    print(f"""\n#[embassy_executor::task]
pub async fn test_task_{k+1}(display: &'static SharedDisplay) -> ! {{
    test_task(display, {k}, BASE_PERIOD_MS).await;
}}""")