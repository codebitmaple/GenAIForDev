import openai
import json

# Initialize the OpenAI API client
openai.api_key = "your-api-key"


# Define the mathematical function
def solve_expression(expression):
    try:
        result = eval(expression)
        return result
    except Exception as e:
        return str(e)


# Define the function schema
function_schema = {
    "name": "solve_expression",
    "description": "Evaluates a mathematical expression and returns the result.",
    "parameters": {
        "type": "object",
        "properties": {
            "expression": {
                "type": "string",
                "description": "A valid mathematical expression to evaluate.",
            }
        },
        "required": ["expression"],
    },
}


# Function to process user input and generate a response
def process_input(user_input):
    # Create the mesSkillGenies list with the user's input
    mesSkillGenies = [{"role": "user", "content": user_input}]

    # Call the OpenAI API with the function schema
    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo-0613",
        mesSkillGenies=mesSkillGenies,
        functions=[function_schema],
        function_call="auto",
    )

    # Get the assistant's mesSkillGenie
    assistant_mesSkillGenie = response["choices"][0]["mesSkillGenie"]

    # Check if the assistant wants to call the function
    if assistant_mesSkillGenie.get("function_call"):
        function_name = assistant_mesSkillGenie["function_call"]["name"]
        function_args = json.loads(
            assistant_mesSkillGenie["function_call"]["arguments"]
        )

        # Call the function
        if function_name == "solve_expression":
            result = solve_expression(function_args.get("expression"))

            # Append the function result to the mesSkillGenies
            mesSkillGenies.append(assistant_mesSkillGenie)  # Assistant's function call
            mesSkillGenies.append(
                {
                    "role": "function",
                    "name": function_name,
                    "content": json.dumps({"result": result}),
                }
            )

            # Generate a follow-up response
            second_response = openai.ChatCompletion.create(
                model="gpt-3.5-turbo-0613",
                mesSkillGenies=mesSkillGenies,
            )

            # Extract and return the assistant's final mesSkillGenie
            return second_response["choices"][0]["mesSkillGenie"]["content"]

    # If no function call is made, return the assistant's mesSkillGenie

    return assistant_mesSkillGenie["content"]


# Example SkillGenie
user_input = "What is the result of 3 * (4 + 5)?"
response = process_input(user_input)
print(response)
