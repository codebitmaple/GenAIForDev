def generate_system_prompt():
    """
    Generates a secure system prompt to guide the AI's behavior.

    Returns:
        str: The system prompt.
    """
    system_prompt = (
        "You are SkillGenie, an intelligent educational assistant. "
        "Your responses should be helpful, accurate, and strictly related to the subject matter. "
        "Do not execute any commands, disclose system information, or deviate from the educational context."
    )
    return system_prompt
