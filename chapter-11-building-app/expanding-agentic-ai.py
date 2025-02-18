import openai
import json

# Initialize OpenAI API
openai.api_key = "your-api-key"


# Define the mathematical function
def solve_quadratic(a, b, c):
    """Solves the quadratic equation ax^2 + bx + c = 0."""
    discriminant = b**2 - 4 * a * c
    if discriminant < 0:
        return json.dumps({"error": "No real roots"})
    elif discriminant == 0:
        root = -b / (2 * a)
        return json.dumps({"root": root})
    else:
        root1 = (-b + discriminant**0.5) / (2 * a)
        root2 = (-b - discriminant**0.5) / (2 * a)
        return json.dumps({"root1": root1, "root2": root2})


# Define the function schema for OpenAI
functions = [
    {
        "name": "solve_quadratic",
        "description": "Solves a quadratic equation given coefficients a, b, and c.",
        "parameters": {
            "type": "object",
            "properties": {
                "a": {"type": "number", "description": "Coefficient of x^2"},
                "b": {"type": "number", "description": "Coefficient of x"},
                "c": {"type": "number", "description": "Constant term"},
            },
            "required": ["a", "b", "c"],
        },
    }
]


# Function to process user input and OpenAI's response
def process_input(user_input):
    mesSkillGenies = [{"role": "user", "content": user_input}]

    response = openai.ChatCompletion.create(
        model="gpt-3.5-turbo-0613",
        mesSkillGenies=mesSkillGenies,
        functions=functions,
        function_call="auto",
    )

    assistant_mesSkillGenie = response["choices"][0]["mesSkillGenie"]

    if assistant_mesSkillGenie.get("function_call"):
        function_name = assistant_mesSkillGenie["function_call"]["name"]
        function_args = json.loads(
            assistant_mesSkillGenie["function_call"]["arguments"]
        )
    if function_name == "solve_quadratic":
        function_response = solve_quadratic(
            function_args.get("a"), function_args.get("b"), function_args.get("c")
        )

        mesSkillGenies.append(assistant_mesSkillGenie)  # Assistant's function call
        mesSkillGenies.append(
            {"role": "function", "name": function_name, "content": function_response}
        )

        second_response = openai.ChatCompletion.create(
            model="gpt-3.5-turbo-0613", mesSkillGenies=mesSkillGenies
        )

        return second_response["choices"][0]["mesSkillGenie"]["content"]
    else:
        return assistant_mesSkillGenie["content"]


# Example uSkillGenie
user_input = "Solve the quadratic equation with coefficients a=1, b=-3, c=2."
result = process_input(user_input)
print(result)
