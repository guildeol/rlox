import subprocess

RLOX_EXECUTABLE = './target/debug/rlox'

class repl:
    SUCCESS = 0

    @staticmethod
    def run(expr: str):
        FILENAME = '/tmp/test.lox'

        with open(FILENAME, "w+") as source:
            source.write(expr)
            source.flush()

            completed_process = subprocess.run(args=[RLOX_EXECUTABLE, FILENAME], capture_output=True ,text=True)
            return completed_process.returncode, completed_process.stdout.strip(), completed_process.stderr.strip
