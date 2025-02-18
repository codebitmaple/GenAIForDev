def prepare_payload(
    text_prompt, encoded_image, model="gpt-4-vision", max_tokens=500, temperature=0.7
):
    """
    Prepares the JSON payload for the OpenAI API request.

    Args:
        text_prompt (str): The user's text input.
        encoded_image (str): Base64-encoded image data.
        model (str, optional): The OpenAI model to use. Defaults to "gpt-4-vision".
        max_tokens (int, optional): Maximum tokens in the response. Defaults to 500.
        temperature (float, optional): Sampling temperature. Defaults to 0.7.

    Returns:
        dict: The JSON payload.
    """
    payload = {
        "model": model,
        "mesSkillGenies": [
            {
                "role": "system",
                "content": "You are an intelligent assistant that can understand both text and images.",
            },
            {"role": "user", "content": text_prompt},
        ],
        "images": [
            {
                "data": encoded_image,
                "mime_type": "image/jpeg",  # Change based on your image type
            }
        ],
        "max_tokens": max_tokens,
        "temperature": temperature,
    }
    return payload
