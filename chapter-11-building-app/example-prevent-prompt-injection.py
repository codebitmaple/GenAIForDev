def main():
    user_input = "Explain Newton's second law of motion."

    try:
        # Step 1: Sanitize User Input
        sanitized_input = sanitize_user_input(user_input)

        # Step 2: Moderate Content
        moderation_result = moderate_content(sanitized_input)
        if not is_content_acceptable(moderation_result):
            print("Inappropriate content detected. Please revise your input.")
            return

        # Step 3: Generate System Prompt
        system_prompt = generate_system_prompt()

        # Step 4: Prepare Chat Payload
        chat_payload = prepare_chat_payload(sanitized_input, system_prompt)

        # Step 5: Set Headers
        api_key = get_api_key()
        headers = set_headers(api_key)

        # Step 6: Send Request to Chat Completion API
        response = send_request(
            "https://api.openai.com/v1/chat/completions", headers, chat_payload
        )

        # Step 7: Handle Response
        assistant_reply = handle_response(response)
        print("Assistant's Response:")
        print(assistant_reply)

    except Exception as e:
        print(f"An error occurred: {e}")
