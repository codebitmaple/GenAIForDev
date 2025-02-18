import re


def sanitize_user_input(user_input):
    """
    Sanitizes user input to prevent prompt injection.

    Args:
        user_input (str): The raw input from the user.

    Returns:
        str: The sanitized user input.
    """
    # Remove special characters that could alter the prompt structure
    sanitized = re.sub(r"[<>]", "", user_input)
    return sanitized
