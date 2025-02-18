def set_headers(api_key):
    """
    Sets up the headers for the HTTP request.

    Args:
        api_key (str): OpenAI API key.

    Returns:
        dict: Headers dictionary.
    """
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
    }
    return headers
