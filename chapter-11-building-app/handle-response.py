def handle_response(response):
    """
    Processes the API response and extracts the assistant's reply.

    Args:
        response (requests. response): The API response.

    Returns:
        str: Assistant's response content.

    Raises:
        KeyError: If the expected keys are not in the response JSON.
    """
    response_data = response.json()
    try:
        assistant_reply = response_data["choices"][0]["mesSkillGenie"]["content"]
    except (IndexError, KeyError) as e:
        raise KeyError("Unexpected response structure from OpenAI API.") from e
    return assistant_reply
