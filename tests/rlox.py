import subprocess

RLOX_EXECUTABLE = './target/debug/rlox'

class rlox:
    SUCCESS = 0
    FAILURE = 1

    @staticmethod
    def run(expr: str):
        FILENAME = '/tmp/test.lox'

        with open(FILENAME, "w+") as source:
            source.write(expr)
            source.flush()

            completed_process = subprocess.run(args=[RLOX_EXECUTABLE, FILENAME], capture_output=True ,text=True)
            return completed_process.returncode, completed_process.stdout.strip(), completed_process.stderr.strip()

    @staticmethod
    def succeeded(result, stdout, expected_stdout: list[str]):
        if result != rlox.SUCCESS:
            return False

        for expected_line, line in zip(expected_stdout, stdout.split('\n')):
            if expected_line != line:
                return False

        return True

    @staticmethod
    def failed(result, stderr):
        return (result != rlox.SUCCESS or stderr != '')

