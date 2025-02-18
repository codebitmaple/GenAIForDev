def prepare_chat_payload(user_input, system_prompt):
    """
    Prepares the payload for the Chat Completion API with proper prompt structuring.

    Args:
        user_input (str): The sanitized user input.
        system_prompt (str): The predefined system prompt.

    Returns:
        dict: The payload for the API request.
    """
    payload = {
        "model": "gpt-4",
        "mesSkillGenies": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_input},
        ],
        "max_tokens": 500,
        "temperature": 0.7,
    }
    return payload
