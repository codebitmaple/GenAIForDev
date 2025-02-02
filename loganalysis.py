import re
import torch
from transformers import pipeline

# Load a pre-trained text classification model (replace with a fine-tuned debugger)
bug_detector = pipeline("text-classification", model="facebook/bart-large-mnli")

# Function to load system logs from a file
def load_system_logs(file_path):
    with open(file_path, "r") as file:
        logs = file.readlines()
    return logs

# Function to detect bugs using AI model
def detect_bugs(logs):
    suspected_issues = []
    for log in logs:
        # Preprocess log data
        log_cleaned = re.sub(r'\d+', '', log)  # Remove timestamps/numbers
        prediction = bug_detector(log_cleaned)
        
        # If AI classifies log as a potential error, add it to suspected issues
        if prediction[0]['label'] == "ERROR" or prediction[0]['score'] > 0.8:
            suspected_issues.append(log)
    
    return suspected_issues

# Load logs from a file (Replace with actual log file path)
log_file = "application_logs.txt"
logs = load_system_logs(log_file)

# Run AI-powered bug detection
suspected_issues = detect_bugs(logs)

# Print potential errors for developers to review
if suspected_issues:
    print("\n🔍 Potential Bug Locations Found:")
    for issue in suspected_issues:
        print(f"- {issue.strip()}")
else:
    print("✅ No critical issues detected in logs.")