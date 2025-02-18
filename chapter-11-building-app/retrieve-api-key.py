import requests
import json
import base64
import os


def get_api_key():
    """
    Retrieves the OpenAI API key from environment variables.

    Returns:
        str: OpenAI API key.

    Raises:
        EnvironmentError: If the API key is not found.
    """
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise EnvironmentError("Please set the OPENAI_API_KEY environment variable.")
    return api_key
