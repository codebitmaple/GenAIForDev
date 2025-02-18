import requests
import json
import os


def get_api_key():
    """
    Retrieves the OpenAI API key from environment variables.
    """
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise EnvironmentError("Please set the OPENAI_API_KEY environment variable.")
    return api_key


def moderate_content(text):
    """
    Sends text to OpenAI's Moderation API to check for inappropriate content.

    Args:
        text (str): The user-provided text input.

    Returns:
        dict: The moderation results.
    """
    api_key = get_api_key()
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
    }
    payload = {"input": text}
    response = requests.post(
        "https://api.openai.com/v1/moderations",
        headers=headers,
        data=json.dumps(payload),
    )
    response.raise_for_status()
    return response.json()


def is_content_acceptable(moderation_result):
    """
    Determines if the content is acceptable based on Moderation API results.

    Args:
        moderation_result (dict): The response from the Moderation API.

    Returns:
        bool: True if content is acceptable, False otherwise.
    """
    results = moderation_result.get("results", [])
    if not results:
        return False
    categories = results[0].get("categories", {})
    # If any category is flagged, consider the content unacceptable
    return not any(categories.values())


def main():
    user_input = "User's input text goes here."

    try:
        moderation_result = moderate_content(user_input)
        if is_content_acceptable(moderation_result):
            print("Content is acceptable. Proceeding with processing.")
            # Proceed with further processing (e.g., sending to AI Engine)
        else:
            print("Inappropriate content detected. Please revise your input.")
            # Handle inappropriate content (e.g., notify user, log incident)
    except Exception as e:
        print(f"An error occurred during moderation: {e}")


if __name__ == "__main__":
    main()
