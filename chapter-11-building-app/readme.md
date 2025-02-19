# SkillGenie - AI-Powered Learning Assistant

## Overview
SkillGenie is an AI-powered learning assistant designed to provide personalized tutoring using Generative AI. This project demonstrates how to build and deploy a GenAI-based application, incorporating multimodal input, adaptive learning paths, and AI-driven feedback loops. The repository contains the implementation for Chapter 11: "Building a Generative AI Application" from the book *GenAI for Developers*.

## Features
- **Multimodal Input:** Supports text, voice, and image-based queries.
- **AI-Powered Explanations:** Uses OpenAI’s Chat Completion API for intelligent tutoring.
- **Feedback Loop:** Learns from user responses to improve explanations over time.
- **Content Moderation:** Implements OpenAI’s Moderation API to ensure safe interactions.
- **Adaptive Learning Paths:** Dynamically adjusts lessons based on student performance.
- **Scalable Deployment:** Uses Docker and Docker Compose for seamless deployment.

## Prerequisites
Before running the application, ensure you have the following installed:
- Python 3.x
- Docker & Docker Compose
- OpenAI API Key (for AI processing)
- PostgreSQL (for user data storage)

## Installation & Setup
### 1. Clone the Repository
```bash
git clone -b bitmaple/chapter-11-building-app https://github.com/codebitmaple/GenAIForDev.git
cd GenAIForDev
```

### 2. Set Up Environment Variables
Create a `.env` file in the root directory and add the following:
```env
OPENAI_API_KEY=your_api_key_here
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=skillgenie_db
```

### 3. Install Python Dependencies
```bash
pip install -r requirements.txt
```

### 4. Run the Application
```bash
python app.py
```

### 5. Deploy with Docker
Build and start all services using Docker Compose:
```bash
docker-compose up -d
```

## API Endpoints
| Method | Endpoint        | Description |
|--------|----------------|-------------|
| POST   | `/inputs`       | Accepts multimodal input (text, audio, image) |
| GET    | `/hints`        | Retrieves hints based on user queries |
| POST   | `/solutions`    | Submits user solutions for evaluation |
| GET    | `/lessons`      | Provides structured lessons |
| POST   | `/evaluate`     | Evaluates user performance |

## Technologies Used
- **Programming Language:** Python
- **AI Integration:** OpenAI GPT-4 API, Whisper API
- **Backend Framework:** Flask
- **Database:** PostgreSQL
- **Deployment:** Docker, Docker Compose
- **Monitoring:** Prometheus, Grafana

## Future Enhancements
- Implement Rust-based backend for memory efficiency.
- Expand AI capabilities to support interactive coding exercises.
- Improve user personalization with reinforcement learning.

## Contributions
We welcome contributions! Please follow these steps:
1. Fork the repository.
2. Create a new branch (`feature-branch`).
3. Commit your changes and push to your fork.
4. Submit a Pull Request.

## License
This project is licensed under the MIT License - see the `LICENSE` file for details.

## Contact
For support or inquiries, please create an issue in the repository or reach out to [maintainer email].

