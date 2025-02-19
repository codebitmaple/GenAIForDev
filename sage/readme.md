# Sage - AI-Powered Learning Assistant

## Overview
Sage is a Generative AI-powered learning assistant designed to help students with personalized tutoring. It leverages advanced AI models to provide multimodal (text, voice, and image) input support, interactive learning paths, and adaptive feedback loops to enhance the learning experience.

## Features
- **Multimodal Input Support:** Users can input text, voice, or images to interact with the AI tutor.
- **Adaptive Learning Paths:** Dynamically adjusts learning content based on user progress.
- **Guided Study Mode:** Provides structured lessons and ensures users stay on topic.
- **Self-Study Mode:** Allows users to explore topics, read AI-generated notes, and ask follow-up questions.
- **Evaluation & Feedback:** Includes quizzes and AI-powered feedback to reinforce learning.
- **Content Moderation:** Ensures safe and appropriate interactions using OpenAI's Moderation API.
- **Scalability & Security:** Built with Docker and API-based architecture for easy deployment and management.

## Tech Stack
- **Backend:** Python (FastAPI, Flask)
- **AI Models:** OpenAI GPT-4, Amazon Bedrock
- **Database:** PostgreSQL, Pinecone (Vector Database)
- **Cloud Services:** AWS Lambda, S3, API Gateway
- **Frontend:** React Native (for mobile application)
- **Deployment:** Docker, Docker Compose, Kubernetes (Optional)

## Getting Started
### Prerequisites
- Python 3.9+
- Docker & Docker Compose
- OpenAI API Key
- AWS Account (if deploying on AWS)

### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/codebitmaple/GenAIForDev.git
   cd GenAIForDev/sage
   ```
2. Install dependencies:
   ```sh
   pip install -r requirements.txt
   ```
3. Set up environment variables:
   ```sh
   export OPENAI_API_KEY="your_openai_api_key"
   export DATABASE_URL="your_database_url"
   ```
4. Run the application:
   ```sh
   python app.py
   ```

### Running with Docker
1. Build the Docker image:
   ```sh
   docker build -t sage-ai .
   ```
2. Run the container:
   ```sh
   docker run -p 5000:5000 --env-file .env sage-ai
   ```

### API Endpoints
| Method | Endpoint | Description |
|--------|---------|-------------|
| POST | `/inputs` | Accepts text, audio, or image input |
| GET | `/hints` | Retrieves AI-generated hints |
| POST | `/solutions` | Submits a solution for evaluation |
| GET | `/lessons` | Fetches personalized learning paths |
| POST | `/evaluate` | Conducts a quiz and provides feedback |

## Contributing
We welcome contributions! Please follow these steps:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to your branch (`git push origin feature-name`)
5. Open a Pull Request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact
For questions or suggestions, open an issue or reach out via email at `support@codebitmaple.com`.

---
🚀 **Sage** aims to make learning interactive, engaging, and AI-powered. Get started today and enhance your learning experience! 🚀
