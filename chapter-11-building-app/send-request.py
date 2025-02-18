def send_request(api_url, headers, payload):
    """
    Sends a POST request to the OpenAI API.

    Args:
        api_url (str): The OpenAI API endpoint URL.
        headers (dict): HTTP headers.
        payload (dict): JSON payload.

    Returns:
        requests.Response: The API response.

    Raises:
        requests.exceptions.RequestException: For network-related errors.
    """
    response = requests.post(api_url, headers=headers, data=json.dumps(payload))
    response.raise_for_status()  # Raises HTTPError for bad responses (4xx or 5xx)
    return response
