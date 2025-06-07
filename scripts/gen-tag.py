import re
import datetime
import subprocess


def get_existing_tags():
    """
    Returns a list of existing Git tags in the current repository.
    """
    try:
        result = subprocess.run(
            ["git", "tag"], capture_output=True, text=True, check=True
        )
        return result.stdout.strip().split("\n") if result.stdout else []
    except subprocess.CalledProcessError as e:
        print(f"Error retrieving Git tags: {e}")
        return []


def create_git_tag(tag_name):
    """
    Creates a new Git tag with the specified name.
    """
    try:
        subprocess.run(["git", "tag", tag_name], check=True)
        print(f"Tag created: {tag_name}")
    except subprocess.CalledProcessError as e:
        print(f"Failed to create tag '{tag_name}': {e}")


def generate_next_tag():
    """
    Determines the next available tag in the format 'vYYYY-MM-DD[a-z]'.
    Returns the next tag name, or None if today's base tag is exhausted.
    """
    today_str = datetime.date.today().strftime("%Y-%m-%d")
    base_tag = f"v{today_str}"
    tag_pattern = re.compile(rf"{base_tag}[a-z]$")

    existing_tags = get_existing_tags()
    today_tags = sorted(tag for tag in existing_tags if tag_pattern.match(tag))

    if not today_tags:
        return f"{base_tag}a"

    last_tag = today_tags[-1]
    last_suffix = last_tag[-1]

    if last_suffix == "z":
        print("Maximum tags for today reached (a-z).")
        return None

    next_suffix = chr(ord(last_suffix) + 1)
    return f"{base_tag}{next_suffix}"


def confirm_and_create_tag(tag_name):
    """
    Prompts user for confirmation and creates the tag if confirmed.
    """
    print("Recent tags:")
    for tag in sorted(get_existing_tags())[-5:]:
        print(f" - {tag}")

    response = input(f"Create tag '{tag_name}'? (y/n): ").strip().lower()
    if response == "y":
        create_git_tag(tag_name)
        return tag_name
    else:
        print("Tag creation cancelled.")
        return None


def main():
    next_tag = generate_next_tag()
    if next_tag:
        if next_tag not in get_existing_tags():
            created_tag = confirm_and_create_tag(next_tag)
            if created_tag:
                print(f"Created tag: {created_tag}")
        else:
            print(f"Tag '{next_tag}' already exists.")
    else:
        print("No new tag generated.")


if __name__ == "__main__":
    main()
