# COURSE LINK : https://www.youtube.com/watch?v=FueXoIewxg0
---

# Git & GitHub: The Complete Guide

> This note is a comprehensive summary and detailed breakdown of the "Git and GitHub | Complete Guide" course by Badr from the Git Unleashed channel. It covers everything from the foundational concepts of version control to advanced Git tools and collaborative workflows on GitHub.

**Tags:** `#Git` `#GitHub` `#VCS` `#SoftwareDevelopment` `#CourseNotes`

---

## Part I: Introduction to Git

### 1. Course Overview

This course is a complete guide designed to cover all the essential concepts you need to start using [[Git]] and [[GitHub]] effectively. The content is structured into six chapters that build upon one another, so it's recommended to follow them in order.

- **Chapter 1:** Introduction to [[Git]], its purpose in building a project timeline, its history, and its internal workings.
- **Chapter 2:** Using Git to store and revert changes to your work.
- **Chapter 3:** Working on multiple features simultaneously using branches.
- **Chapter 4:** Reshaping, restructuring, and searching through your project's history.
- **Chapter 5:** How to share your work with others using remote repositories.
- **Chapter 6:** The role of [[GitHub]] in collaboration, plus its many additional features.

### 2. What is a Version Control System (VCS)?

A **Version Control System (VCS)** is software that tracks changes to a file or a set of files over time. This allows you to recall specific versions later, compare changes, and revert to previous states if something goes wrong.

There are three main types of VCS:

#### a. Local Version Control Systems

This was the earliest form of version control, replacing the traditional method of manually copying files and renaming them (e.g., `report_v1.docx`, `report_v2.docx`).

- **Limitation:** It is designed for a single user and does not support collaboration with others.

#### b. Centralised Version Control Systems (CVCS)

To enable collaboration, Centralised Version Control Systems were developed. The core idea was to have a single central server that stores the entire version history of a project in a place called a **repository**.

- **How it works:** Collaborators connect to this central server to get the latest version of the files and to save their changes.

--Image of: --Centralised Version Control

- **Major Drawbacks:**
    - **Single Point of Failure:** If the central server goes down, no one can collaborate, and if its disk gets corrupted, the entire project history could be lost permanently.
    - **Internet Dependency:** You must be connected to the internet to record (commit) your changes.
    - **Slowness:** Nearly every operation requires communication with the central server, which is limited by network speed.

#### c. Distributed Version Control Systems (DVCS)

DVCSs, like **Git**, were created to solve the problems of CVCSs.

- **How it works:** Instead of just one central repository, **every collaborator has a full, local copy (a clone) of the entire repository**, including its complete history.

--Image of: --Distributed Version Control

- **Key Advantages:**
    - **Redundancy:** The history is distributed across multiple machines. If the central server fails, any collaborator's local repository can be used to restore it.
    - **Speed:** Most operations, like committing changes, are performed locally, making them incredibly fast.
    - **Offline Capability:** You can create a complete version history on your local machine without any internet access. The server is only needed when you want to **share** your changes with others.

> **Git is a Distributed Version Control System**.

### 3. A Brief History of Git

- **Creator:** Linus Torvalds, the creator of the Linux kernel.
- **The Context:** In 2002, the Linux kernel development team used a DVCS called **BitKeeper**.
- **The Catalyst:** In 2005, the company behind BitKeeper revoked the team's free license.
- **The Creation:** In response, Linus Torvalds decided to create his own VCS. He developed the first version of Git in approximately **two weeks**, with the initial release on **8 April 2005**.

#### What does "Git" mean?

Linus explained that the name "Git" can mean several things:

- A random, three-letter name that is easy to type.
- In British slang, "git" means a stupid or unpleasant person.
- A potential backronym: **"Global Information Tracker"**.

---

## Part II: Getting Started and Git Internals

### 1. Installation and Initial Configuration

#### a. Installation

1. Navigate to the **official Git website** (`git-scm.com`).
2. Go to the Downloads section; the website will likely auto-detect your operating system.
3. Follow the straightforward installation steps.
4. To verify the installation, open your terminal (or Command Line/Git Bash on Windows) and run:

```
   git --version
```

    This should return the installed Git version.

#### b. Configuration (`git config`)

Before you start, you need to tell Git who you are. This information is attached to every change you make.

- **Configuration Scopes:**
    
    - `--local`: Settings for a single, specific project.
    - `--global`: Settings for the current user on the machine (this is the most common).
    - `--system`: Settings for all users on the system.
- **Commands to set your identity:**
    
 ```
   # Set your name (use quotes if it contains spaces)
   git config --global user.name "Your Name"
   
   # Set your email
   git config --global user.email "you@example.com"
  ```
    

### 2. Initialising a Repository (`git init`)

To start tracking a project, you need to initialise a Git repository.

- `git init`: Initialises a new repository in the **current directory**.
- `git init <project-name>`: Creates a new directory named `<project-name>` and initialises a repository inside it.

This command creates a hidden sub-directory named **`.git`**. This directory contains all the necessary repository files—the entire history and metadata for your project. To see it, you can use `ls -a` (on Linux/macOS).

### 3. How Git Thinks: The Internal Data Model

To truly master Git, it's essential to understand how it works internally. Git is fundamentally a **content-addressable file system**. This means that when you give Git some content to store, it gives you back a unique key (a hash) that you can use to retrieve that content later.

Git is composed of two main components:

1. **Object Database:** Stores your project's data (file contents, directory structures, commits).
2. **References:** Pointers (like branch or tag names) to objects in the database, making them easier to find.

#### a. The Object Database & SHA-1 Hashing

Git stores everything as **objects**. The "name" or ID of each object is a **SHA-1 hash**—a unique 40-character hexadecimal string—calculated based on the object's content.

- This means that if the content changes even slightly, the hash will change completely.
- Conversely, identical content will always produce the exact same hash.

An object in Git consists of:

1. A header specifying the **object type** (e.g., `blob`).
2. The **size** of the content.
3. A **null byte** separator.
4. The actual **content** (data).

This entire block is then compressed with **zlib** before being written to the database to save space.

#### b. Git Object Types

There are three main object types that form the foundation of a Git repository.

**1. The Blob (Binary Large Object)**

- **Purpose:** A blob stores the raw **content of a file**.
- **Key Detail:** It contains _only_ the file's data, not its name, path, or any other metadata.
- **Efficiency:** If you have multiple files with the exact same content, Git stores only **one blob object** and has all those files reference it, which saves a significant amount of space.
- **Low-level command:** `git hash-object -w <filename>` creates a blob from a file's content and writes it to the database.

**2. The Tree**

- **Purpose:** A tree represents a **directory snapshot**. It's like a folder.
- **Content:** A tree object contains a list of entries, where each entry includes:
    - The file/directory **mode** (permissions, e.g., `100644` for a normal file, `040000` for a subdirectory).
    - The object **type** (`blob` or `tree`).
    - The **SHA-1 hash** of the blob or sub-tree.
    - The **filename** or subdirectory name.
- **Low-level command:** `git write-tree` creates a tree object from the current state of the [[The Index (Staging Area)]].

--Image of: --Git Object Structure

**3. The Commit**

- **Purpose:** A commit object represents a complete snapshot of your project at a specific moment in time.
- **Content:** It stores metadata about the snapshot:
    - The **SHA-1 hash** of the root **tree** object for that snapshot.
    - The **SHA-1 hash** of the **parent commit(s)**. This is what links commits together to form a history.
    - **Author** and **Committer** information (name, email, timestamp).
    - A **commit message** explaining the changes.
- **Low-level command:** `git commit-tree <tree-hash> -m "message"` creates a commit object.

#### c. The Index (Staging Area)

Before a change becomes part of a commit, it must be placed in a special area called the **Index** or **Staging Area**.

- **Analogy:** The index is like the frame of a photograph. Anything you place inside the frame will be captured in the next photo (commit). Anything outside the frame will be ignored.
- **Function:** It's a staging ground where you build up your next commit. You can add and remove files until you are satisfied with the snapshot you want to record.
- It stores detailed information about each file, including permissions, owner, size, path, and the hash of its content (the blob).
- **Low-level command:** `git update-index --add ...` adds a file to the index.
- **High-level command:** `git ls-files --stage` shows the content of the index.

#### d. References: Making Hashes Human-Readable

SHA-1 hashes are great for computers but terrible for humans. **References** are pointers that give human-readable names to specific commit hashes.

- **HEAD:** A special, symbolic reference that always points to **your current location** in the project history. Usually, HEAD points to a branch, meaning you are working on that branch.
- **Branches:** These are simply movable pointers to a specific commit, typically the latest commit in a line of work. When you make a new commit, the branch pointer moves forward automatically. The default branch is typically named `main` or `master`.
- **Tags:** These are references that point to a specific commit to mark a point in history as important, such as a version release (e.g., `v1.0.0`). Unlike branches, tags generally do not move once created.
    - **Lightweight Tag:** A simple pointer to a commit.
    - **Annotated Tag:** A full Git object that contains extra metadata (tagger name, email, date, and a tagging message) and is generally preferred for official releases.

--Image of: --Git History with Objects and References

---

## Part III: The Basic Git Workflow

This section covers the day-to-day commands for tracking and managing changes in your project.

### 1. The Three-Tree Model in Practice

Git's workflow is best understood through its management of three key "trees" or states:

1. **Working Directory:** The actual files on your computer that you can see and edit.
2. **Index (Staging Area):** A staging file where Git stores information about what will go into your next commit.
3. **HEAD:** A pointer to the last commit you made. It represents the last saved state of your project.

--Image of: --The Three Trees of Git

- `git status`: This command compares these three trees to give you the status of your files. It tells you which files are modified but not staged, which are staged and ready to be committed, and which are untracked.
- `git diff`:
    - `git diff`: Shows the differences between your **Working Directory** and the **Index**. These are your unstaged changes.
    - `git diff --staged` (or `--cached`): Shows the differences between the **Index** and **HEAD**. These are the changes you've staged and are ready to commit.

### 2. Staging and Committing Changes

The fundamental Git workflow is a two-step process:

1. **Stage Changes with `git add`:** You use the `git add` command to move changes from your Working Directory to the Staging Area. This tells Git you want to include these updates in the next commit.
    
    ```
    # Stage a specific file
    git add readme.md
    
    # Stage all changes in the current directory and subdirectories
    git add .
    ```
    
    Internally, `git add` creates a [[#1 The Blob (Binary Large Object)|blob]] object for the file's content (if it doesn't already exist) and updates the [[#c The Index (Staging Area)|Index]] to point to that blob.
    
2. **Save Changes with `git commit`:** Once your staged changes are ready, you use `git commit` to permanently store the staged snapshot in your project's history.
    
    ```
    # Commit with an inline message
    git commit -m "Add initial project README"
    ```
    
    This creates a new [[#3 The Commit|commit]] object, which points to the [[#2 The Tree|tree]] representing the state of the Index. It also sets the current commit's parent to the previous commit (what HEAD was pointing to).
    

### 3. Ignoring Files (`.gitignore`)

You often have files or directories in your project that you don't want Git to track. These can include build artifacts, log files, or user-specific editor settings.

- **`.gitignore` File:** This is a plain text file where you list patterns of files and directories to ignore. It's a tracked file, so it's shared with everyone collaborating on the project. Example:
    
    ```
    # Ignore executable files
    *.exe
    
    # Ignore the build output directory
    /build/
    ```
    
- **`.git/info/exclude`:** For personal ignore rules that you don't want to share with the team (like your editor's configuration files), you can add patterns to this file. It is not tracked by Git.
    

> **Important:** Git cannot ignore a file that is already being tracked. You must first remove it from the index using `git rm --cached <file>` before adding it to `.gitignore`.

### 4. Undoing and Reverting Changes

Mistakes happen. Git provides powerful tools to undo changes at every stage of the workflow.

#### a. Unstaging a Staged File

If you staged a file by mistake (`git add`), you can unstage it:

```
# Move the file from the Index back to the Working Directory
git restore --staged <file>...
```

The transcript also notes that an older command, `git rm --cached <file>`, was previously used. The key difference is that `restore --staged` is for reverting a tracked file to its previous state in HEAD, while `rm --cached` is better for completely untracking a newly added file.

#### b. Discarding Changes in the Working Directory

To discard local modifications in a file and revert it to the version in the Index (or HEAD if not staged):

```
# Discard changes in the working directory
git restore <file>...
```

#### c. Amending the Last Commit (`--amend`)

If you just made a commit and realized you forgot something or made a typo in the message, you can "amend" it:

```
# Add the forgotten file
git add forgotten-file.txt

# Amend the last commit to include the new file and/or change the message
git commit --amend
```

This **does not create a new commit**; instead, it **replaces the previous commit** with a new, improved one.

#### d. Reverting a Public Commit (`git revert`)

If you need to undo a commit that has already been pushed and shared with others, the safest way is `git revert`.

```
git revert <commit-hash>
```

This creates a **new commit** that is the inverse of the specified commit. It doesn't rewrite history, which makes it safe for collaboration. You use this to say, "I made a mistake, and here is the commit that fixes it".

#### e. Rewriting History (`git reset`)

`git reset` is a powerful and potentially dangerous command that moves the current branch pointer to a different commit, effectively rewriting history.

> **Warning:** Never use `git reset` on commits that have been pushed to a shared repository unless you have coordinated with your entire team.

- `git reset --soft HEAD~1`: Moves the branch pointer back one commit but leaves your changes from that commit **staged** in the Index.
- `git reset --mixed HEAD~1` (default): Moves the branch pointer and unstages the changes, leaving them as modifications in your **Working Directory**.
- `git reset --hard HEAD~1`: Moves the branch pointer and **discards all changes** from that commit completely (from the Index and Working Directory).

#### f. The Reflog: Your Safety Net

If you accidentally lose work with `git reset --hard`, don't panic! Git keeps a record of where HEAD has been in the **reflog**.

```
git reflog
```

This will show you a history of your commits, even the "lost" ones. You can find the hash of the commit you want to restore and use `git reset --hard <hash>` to get back to it.

### 5. Stashing Changes (`git stash`)

Sometimes you're in the middle of working on a feature when you need to switch contexts quickly to fix an urgent bug. Your work isn't ready to be committed, but you need a clean working directory. This is the perfect use case for `git stash`.

- `git stash`: Takes your modified tracked files (both staged and unstaged), saves them in a "stash," and reverts your working directory to match the last commit (HEAD).
- `git stash list`: View all your stashes.
- `git stash apply`: Apply the most recent stash to your working directory but keep it in the stash list.
- `git stash pop`: Apply the most recent stash and remove it from the list.

---

## Part IV: Branching and Restructuring History

### 1. Understanding Branches

A **branch** in Git is simply a lightweight, movable pointer to a commit. It's a core concept that enables parallel development. You can create a new branch to work on a feature without affecting the main line of development (`main` branch).

--Image of: --Git Branching

- **`git branch`**: List all local branches.
- **`git branch <branch-name>`**: Create a new branch.
- **`git switch <branch-name>`** (or older `git checkout <branch-name>`): Switch to an existing branch.
- **`git switch -c <branch-name>`** (or older `git checkout -b <branch-name>`): Create a new branch and switch to it immediately.

### 2. Navigating History and the "Detached HEAD" State

You can visit any commit in your history using `git checkout <commit-hash>`. When you do this, HEAD no longer points to a branch but directly to a commit. This is known as a **"detached HEAD" state**.

- In this state, you can look around and experiment.
- If you make new commits, they won't belong to any branch. They exist, but they are "floating".
- If you switch back to a branch (e.g., `git switch main`), these new commits will be lost unless you create a new branch to point to them before switching (`git branch <new-branch>`).

### 3. Combining Work: Merging vs. Rebasing

Once a feature branch is complete, you need to integrate its changes back into your main branch. There are two primary ways to do this:

#### a. Merging (`git merge`)

Merging combines the histories of two branches.

- **Fast-Forward Merge:** If the `main` branch has not had any new commits since the feature branch was created, Git will simply move the `main` branch pointer forward to the tip of the feature branch. No new commit is created.
- **Three-Way Merge:** If both branches have new commits (they have diverged), Git creates a new **merge commit**. This special commit has two parents—one from each branch—unifying their histories. This preserves the exact history but can lead to a complex, non-linear log.

**Merge Conflicts** happen when changes in both branches affect the same lines in the same file. Git will stop the merge and ask you to manually resolve the conflicting code before completing the merge.

#### b. Rebasing (`git rebase`)

Rebasing is an alternative to merging that rewrites history.

- **How it works:** `git rebase main` on your feature branch will take all the commits from your feature branch, temporarily save them, rewind your branch to the common ancestor, fast-forward it to the tip of `main`, and then **replay** your commits one by one on top of `main`.

--Image of: --Git Rebasing

- **Result:** A perfectly **linear history**, as if you had developed your feature sequentially after all the work on `main` was done.
- **The Golden Rule of Rebasing:** **Never rebase commits that have already been pushed to a public/shared repository.** Because it creates new commits, it can cause major problems for collaborators who have based their work on the original commits.

|Feature|Merging|Rebasing|
|:--|:--|:--|
|**History**|Preserves the exact, non-linear history. Creates a merge commit.|Rewrites history to be linear and clean. Avoids merge commits.|
|**Collaboration**|Safe for shared/public branches.|Can be dangerous for shared branches; best used on local branches before sharing.|
|**Traceability**|Easy to see where features were branched and merged back in.|Can make it harder to see the original context of feature development.|

### 4. Advanced History Manipulation Tools

Git provides several powerful tools for reshaping and searching your project's history.

#### a. Interactive Rebase (`git rebase -i`)

This is one of Git's most powerful features. It allows you to alter a series of commits in various ways.

```
# Start an interactive rebase for the last 5 commits
git rebase -i HEAD~5
```

This opens an editor with a list of commits and actions you can perform:

- `reword`: Change a commit message.
- `edit`: Stop at a commit to split it, add/remove files, or make other changes.
- `squash`: Combine a commit with the one before it into a single commit.
- `fixup`: Same as `squash`, but discards the commit message of the squashed commit.
- `drop`: Completely remove a commit.
- You can also reorder the lines to change the order of the commits.

#### b. Cherry-Picking (`git cherry-pick`)

If you only need to bring one specific commit from another branch into your current branch, you can use `cherry-pick`.

```
git cherry-pick <commit-hash>
```

This takes the changes introduced in that specific commit and applies them as a new commit on your current branch.

#### c. Finding Bugs with `git bisect`

If you know a bug exists now but it didn't in a past version, `git bisect` can help you find the exact commit that introduced it using a binary search.

1. `git bisect start`
2. `git bisect bad HEAD` (mark the current commit as broken).
3. `git bisect good <commit-hash-of-last-known-good-state>` (mark a past commit as working).
4. Git will check out a commit in the middle. You test it and tell Git `git bisect good` or `git bisect bad`.
5. Repeat until Git pinpoints the exact commit that introduced the bug.
6. `git bisect reset` to end the session.

#### d. Finding Who to Blame (`git blame`)

The `git blame` command examines the contents of a file and shows which commit and author was the last to modify each line.

```
git blame <filename>
```

This is incredibly useful for understanding the history of a piece of code and finding the right person to ask questions about it.

---

## Part V: Collaborating with Remote Repositories

### 1. Remotes and the Basic Workflow

So far, all work has been in a local repository. To collaborate, you need a **remote repository**—a version of your project hosted on a server.

- **`git remote add <name> <url>`**: Connect your local repository to a remote one. By convention, the primary remote is named `origin`.
- **`git clone <url>`**: Creates a local copy of a remote repository. It automatically configures the original URL as the `origin` remote.
- **`git push <remote-name> <branch-name>`**: Uploads your local branch's commits to the remote repository.
- **`git fetch <remote-name>`**: Downloads all the latest data (commits, branches, tags) from the remote but **does not** merge it into your local working branches.
- **`git pull <remote-name> <branch-name>`**: This is a combination of `git fetch` followed by `git merge`. It downloads the latest changes and immediately tries to merge them into your current branch.

### 2. Handling Diverged Histories

A common scenario is when you and a collaborator have both made new commits since you last synchronised. This is called a **diverged history**.

If you try to `push` when the remote has changes you don't have, Git will reject it. You must first integrate the remote changes into your local work.

You can do this by running `git pull`. If there are conflicts, Git will ask you to resolve them. You can configure `git pull` to use either `merge` (default) or `rebase`:

```
# Pull using rebase to maintain a linear history
git pull --rebase
```

---

## Part VI: GitHub

### 1. What is GitHub?

**GitHub** is a web-based hosting service for Git repositories. It provides the "server" for your remote repository but also adds a suite of powerful features for collaboration, project management, and automation. Other popular services include GitLab and Bitbucket.

### 2. Getting Started with GitHub

1. **Create a Repository:**
    
    - Give it a name and description.
    - Choose between **Public** (visible to anyone) and **Private** (you choose who can see and contribute).
    - Initialize it with a `README`, `.gitignore` template, and a `LICENSE` file.
2. **Authentication:**
    
    - GitHub has deprecated password authentication for Git operations.
    - You must use a **Personal Access Token (PAT)** or an **SSH key**.
    - PATs are more secure because you can grant them specific permissions (scopes) and set an expiration date.
3. **Collaboration:**
    
    - You can invite collaborators to your repository via `Settings > Collaborators`. This gives them push (write) access to the repository.

### 3. Project Management on GitHub

GitHub provides tools to plan and track your work.

- **Issues:** An issue is a unit of work to be done—a bug report, a feature request, or a general task.
    
    - Issues can be assigned to team members (**Assignees**).
    - They can be categorised with **Labels** (e.g., `bug`, `feature`, `documentation`).
    - You can link a commit to an issue by mentioning its number (e.g., `#4`) in the commit message. Using keywords like `Closes #4` or `Fixes #4` will automatically close the issue when the commit is merged into the default branch.
- **Milestones:** A milestone is used to group related issues together to track progress towards a larger goal, such as a version release. They have a progress bar and an optional due date.
    

### 4. The GitHub Flow: Collaborating with Pull Requests

The **GitHub Flow** is a lightweight, branch-based workflow for collaboration. It ensures that the `main` branch is always stable and deployable.

1. **Create a Branch:** For any new work, create a descriptive branch off of `main`.
2. **Add Commits:** Make your changes and commit them to your branch. Push the branch to the remote repository on GitHub.
3. **Open a Pull Request (PR):** A Pull Request is a formal request to merge your branch into another (usually `main`). It's a place to discuss the proposed changes.
4. **Discuss and Review:** Team members can review the code, leave comments, and request changes. Automated checks (like tests) can also be run.
5. **Merge:** Once the PR is approved, a project maintainer merges it into the `main` branch.
6. **Delete Branch:** After merging, the feature branch is typically deleted.

### 5. Contributing to Open Source with Forks

What if you want to contribute to a public project but aren't a collaborator? You use a **Fork**.

1. **Fork the Repository:** A fork is a personal copy of another user's repository that lives on your own GitHub account.
2. **Clone Your Fork:** Clone your forked repository to your local machine.
3. **Create a Branch and Commit:** Make your changes on a new branch in your local clone.
4. **Push to Your Fork:** Push your branch and its commits to your remote fork on GitHub.
5. **Open a Pull Request:** From your fork on GitHub, open a pull request to the **original (upstream) repository**. The project's maintainers can then review your contribution, discuss it, and merge it if they approve.

A **Contributor** is someone whose work has been merged into a project, whereas a **Collaborator** is someone with direct write access to the repository.

### 6. Automation with GitHub Actions

**GitHub Actions** allows you to automate your software development workflows directly within your repository. It's used for Continuous Integration (CI) and Continuous Deployment (CD).

- **Workflows:** Automated processes defined in YAML files located in the `.github/workflows/` directory.
- **Triggers:** Workflows are triggered by events, such as a `push` to a branch or the creation of a `pull_request`.
- **Jobs:** A workflow consists of one or more jobs, which run on virtual machines (**runners**).
- **Steps:** A job contains a sequence of steps that execute commands or run actions.

A common use case is to automatically run tests on every pull request to ensure that new changes don't break existing functionality.

### 7. Releasing Your Project

When your project reaches a stable state, you can create a **Release**. Releases are based on Git tags and provide a way to package and distribute your software.

1. **Create a Tag:** It's best practice to create an **annotated tag** for releases, as it can store metadata like a release message.
    
    ```
    git tag -a v1.0.0 -m "Release version 1.0.0"
    ```
    
2. **Push the Tag:** You must explicitly push tags to the remote.
    
    ```
    git push origin v1.0.0
    ```
    
3. **Create Release on GitHub:** In the GitHub UI, go to the "Releases" section and draft a new release based on the tag you just pushed. You can write detailed release notes and attach compiled binaries or other assets.
    

---

### Final Message

Completing this guide is the easy part. The crucial next step is to **consistently apply and practice** these concepts in your own projects. Regular use is the only way to truly master Git and GitHub and prevent the knowledge from fading.