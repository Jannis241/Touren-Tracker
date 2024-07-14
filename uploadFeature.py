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
        # Stash local changes to apply after merge
        print("Stashing local changes...")
        run_command("git stash")

        # Pull the latest changes, automatically merging
        print("Pulling the latest changes from the remote repository...")
        run_command("git pull")

        # Apply stashed changes, if any
        print("Applying stashed local changes...")
        run_command("git stash pop")

        # Add all changes to staging
        print("Adding all local changes to staging...")
        run_command("git add .")

        # Commit the changes
        commit_message = "Auto-commit local changes"
        print(f"Committing changes with message: '{commit_message}'")
        run_command(f"git commit -m '{commit_message}'")

        # Push the changes to the remote repository
        print("Pushing changes to the remote repository...")
        run_command("git push")

        print("All changes have been successfully pushed!")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()

