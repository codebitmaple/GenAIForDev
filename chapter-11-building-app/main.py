def main():
    """
    Main function to execute the workflow of submitting text and image data to OpenAI and receiving a response.
    """
    # Configuration
    API_URL = "https://api.openai.com/v1/chat/completions"
    IMAGE_PATH = "path_to_your_image.jpg"  # Update with your image path
    TEXT_PROMPT = "Here is an image of a physics problem. Please help me solve it."
    try:
        # Step 1: Retrieve API Key
        api_key = get_api_key()

        # Step 2: Encode Image
        encoded_image = encode_image(IMAGE_PATH)

        # Step 3: Prepare Payload
        payload = prepare_payload(TEXT_PROMPT, encoded_image)

        # Step 4: Set Headers
        headers = set_headers(api_key)

        # Step 5: Send Request
        response = send_request(API_URL, headers, payload)

        # Step 6: Handle Response
        assistant_reply = handle_response(response)
        print("Assistant's Response:")
        print(assistant_reply)

    except EnvironmentError as env_err:
        print(f"Environment Error: {env_err}")
    except FileNotFoundError as fnf_err:
        print(f"File Error: {fnf_err}")
    except requests.exceptions.HTTPError as http_err:
        print(f"HTTP Error: {http_err} - {http_err.response.text}")
    except requests.exceptions.RequestException as req_err:
        print(f"Request Error: {req_err}")
    except KeyError as key_err:
        print(f"Response Parsing Error: {key_err}")
    except Exception as ex:
        print(f"An unexpected error occurred: {ex}")


if __name__ == "__main__":
    main()
