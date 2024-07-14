import subprocess

def run_git_commands():
    try:
        # Git add .
        print("Running: git add .")
        subprocess.run(["git", "add", "."], check=True)
        
        # Git commit -m "commit"
        print(f'Running: git commit -m "{input("Commit message: ")}"')
        subprocess.run(["git", "commit", "-m", "commit"], check=True)
        
        # Git pull --rebase origin main
        print("Running: git pull --rebase origin main")
        subprocess.run(["git", "pull", "--rebase", "origin", "main"], check=True)
        
        # Git push origin main
        print("Running: git push origin main")
        subprocess.run(["git", "push", "origin", "main"], check=True)
        
        print("All commands executed successfully.")
    
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while executing a git command: {e}")

if __name__ == "__main__":
    run_git_commands()
