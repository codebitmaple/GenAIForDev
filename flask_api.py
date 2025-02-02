from flask import Flask, request, jsonify

app = Flask(__name__)

users = {}

# Create a user
@app.route('/users', methods=['POST'])
def create_user():
    data = request.get_json()
    if 'id' not in data or 'name' not in data:
        return jsonify({'error': 'ID and name are required'}), 400
    users[data['id']] = data['name']
    return jsonify({'message': 'User created successfully'}), 201

# Read a user
@app.route('/users/<int:user_id>', methods=['GET'])
def get_user(user_id):
    if user_id not in users:
        return jsonify({'error': 'User not found'}), 404
    return jsonify({'id': user_id, 'name': users[user_id]}), 200

# Update a user
@app.route('/users/<int:user_id>', methods=['PUT'])
def update_user(user_id):
    if user_id not in users:
        return jsonify({'error': 'User not found'}), 404
    data = request.get_json()
    users[user_id] = data.get('name', users[user_id])
    return jsonify({'message': 'User updated successfully'}), 200

# Delete a user
@app.route('/users/<int:user_id>', methods=['DELETE'])
def delete_user(user_id):
    if user_id not in users:
        return jsonify({'error': 'User not found'}), 404
    del users[user_id]
    return jsonify({'message': 'User deleted successfully'}), 200

if __name__ == '__main__':
    app.run(debug=True)
