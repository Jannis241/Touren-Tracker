import subprocess

def run_command(command):
    """
    Execute a shell command and return its output, raise an exception if it fails.
    """
    result = subprocess.run(command, shell=True, text=True, capture_output=True)
    if result.returncode != 0:
        print(f"Command failed with error: {result.stderr}")
        raise Exception(result.stderr)
    return result.stdout

def main():
    try:
        commit_message=input("commit msg: ")
        print(f"Committing changes with message: '{commit_message}'")

        run_command(f'git commit -a -m "{commit_message}"')
        run_command("git pull --rebase origin main")
        run_command("git push")

        print("All changes have been successfully pushed!")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()

