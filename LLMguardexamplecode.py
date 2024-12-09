import boto3
from llm_guard import scan_output, scan_prompt
from llm_guard.input_scanners import Anonymize, PromptInjection, TokenLimit, Toxicity
from llm_guard.output_scanners import Deanonymize, NoRefusal, Relevance, Sensitive
from llm_guard.vault import Vault

# Specify the AWS region, Bedrock agent ID, alias ID, and session ID
REGION = "us-east-1"
AGENT_ID = "your-bedrock-agent-id"
AGENT_ALIAS_ID = "your-agent-alias-id"
SESSION_ID = "your-session-id"

def invoke_agent(sanitized_prompt, region, agent_id, agent_alias_id, session_id):
    """
    Invokes the Bedrock agent with the provided parameters.

    Args:
        sanitized_prompt (str): The sanitized prompt to send to the agent.
        region (str): The AWS region where the agent is located.
        agent_id (str): The ID of the Bedrock agent.
        agent_alias_id (str): The alias ID of the Bedrock agent.
        session_id (str): The session ID for the agent invocation.

    Returns:
        str: The agent's response.
    """
    # Create a Bedrock Agent Runtime client
    bedrock_agent_runtime = boto3.client("bedrock-agent-runtime", region_name=region)

    # Invoke the agent
    response = bedrock_agent_runtime.invoke_agent(
        agentId=agent_id,
        agentAliasId=agent_alias_id,
        sessionId=session_id,
        inputText=sanitized_prompt,
    )

    # Process the agent's response
    completion = ""
    for event in response.get("completion"):
        chunk = event["chunk"]
        completion += chunk["bytes"].decode()

    return completion


VAULT = Vault()
INPUT_SCANNERS = [Anonymize(VAULT), Toxicity(), TokenLimit(), PromptInjection()]
OUTPUT_SCANNERS = [Deanonymize(VAULT), NoRefusal(), Relevance(), Sensitive()]

# Example: Input reviews containing PII, sensitive data, and potentially toxic content
REVIEWS = """
User: John Doe
Email: john.doe@example.com
Review: This product is amazing! I had some issues initially, but the customer service team fixed everything. 
Phone: 555-987-6543
IP: 192.168.0.101
Other comments: Honestly, anyone who doesn't like this product is an idiot.
Credit card: 1234-5678-9876-5432
"""

# Sanitize the reviews before sending to the AI agent
sanitized_reviews, results_valid, results_score = scan_prompt(INPUT_SCANNERS, REVIEWS)
if any(results_valid.values()) is False:
    print(f"Reviews input is not valid, scores: {results_score}")
    exit(1)

print(f"Sanitized Reviews: {sanitized_reviews}")

# Invoke the AI agent with sanitized reviews to generate a summary
response_summary = invoke_agent(sanitized_reviews, REGION, AGENT_ID, AGENT_ALIAS_ID, SESSION_ID)

# Scan and validate the output (the generated summary)
sanitized_response_summary, results_valid, results_score = scan_output(
    OUTPUT_SCANNERS, sanitized_reviews, response_summary
)
if any(results_valid.values()) is False:
    print(f"Generated summary is not valid, scores: {results_score}")
    exit(1)

# Print the validated summary
print(f"Generated Summary: {sanitized_response_summary}\n")
