# RESUMES_TAG=$(git rev-parse --short HEAD)
AWS_REGION=[your-AWS-region]
AWS_ACCOUNT_ID=[your-AWS-account-id]

## Step 1: Build and push docker images to ECR
docker build -t definite:latest .

# Step 2: authenticate docker to EC2
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com

# Step 3: tag docker images
docker tag definite:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/definite:latest

# Step 4: push docker images to ECR
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/definite:latest

# # create ECR repositories
# aws ecr create-repository --repository-name resumes_web

# Mark docker to start on boot
sudo systemctl enable docker
sudo systemctl start docker

# Redeploy the container
docker stop definite
docker rm definite
docker pull $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/definite:latest
docker run -d \
  --name definite \
  --restart=always \
  -p 9191:9191 \
  $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/definite:latest

