import subprocess
import time


def rust_bench():
    t = time.process_time()
    p = subprocess.Popen(["/home/jferry/misc/rust/grammar/target/release/grammar"])
    while p.poll() is None:
        pass
    elapsed_time = time.process_time() - t
    print("rust => {0:.3f}".format(elapsed_time * 1000) + " ms")


def python_bench():
    t = time.process_time()
    p = subprocess.Popen([
        "/usr/bin/python3",
        "/home/jferry/misc/rust/grammar/benches/grammalect.py"
    ])
    while p.poll() is None:
        pass
    elapsed_time = time.process_time() - t
    print("python => {0:.3f}".format(elapsed_time * 1000) + " ms")

if __name__ == "__main__":
    python_bench()
    rust_bench()
