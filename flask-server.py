from flask import Flask, request, jsonify
import requests
import json

app = Flask(__name__)

# Configuration
HYLE_BLOCKCHAIN_URL = "http://localhost:4321/v1"

@app.route('/')
def home():
    return 'Hello World!'

@app.route('/create-meetup', methods=['POST'])
def create_meetup():
    try:
        # Check if we received any data
        if not request.data:
            return jsonify({"error": "No data provided"}), 400

        # Get the JSON data from the incoming request
        data = request.get_json(force=True)  # force=True will handle content-type issues
        
        print(f"Received data: {data}")  # Debug print
        
        # Forward the request to the hyle
        response = requests.post(
            f"{HYLE_BLOCKCHAIN_URL}/contract/register",
            json=data,
            headers={'Content-Type': 'application/json'}
        )   
        
        print(f"Hyle response: {response.text}")  # Debug print
        
        # Return the response from the Hyle server
        return jsonify(response.json()), response.status_code
    
    except json.JSONDecodeError as e:
        return jsonify({"error": f"Invalid JSON format: {str(e)}"}), 400
    except requests.RequestException as e:
        return jsonify({"error": f"Failed to connect to Hyle server: {str(e)}"}), 502
    except Exception as e:
        # Handle any other errors
        return jsonify({"error": str(e)}), 500

@app.route('/post-root', methods=['POST'])
def post_root():
    try:
        # Check if we received any data
        if not request.data:
            return jsonify({"error": "No data provided"}), 400

        # Get the JSON data from the incoming request
        data = request.get_json(force=True)
        
        print(f"Received root data: {data}")  # Debug print
        
        # Forward the request to the hyle
        response = requests.post(
            f"{HYLE_BLOCKCHAIN_URL}/contract/root",
            json=data,
            headers={'Content-Type': 'application/json'}
        )   
        
        print(f"Hyle root response: {response.text}")  # Debug print
        
        # Return the response from the Hyle server
        return jsonify(response.json()), response.status_code
    
    except json.JSONDecodeError as e:
        return jsonify({"error": f"Invalid JSON format: {str(e)}"}), 400
    except requests.RequestException as e:
        return jsonify({"error": f"Failed to connect to Hyle server: {str(e)}"}), 502
    except Exception as e:
        # Handle any other errors
        return jsonify({"error": str(e)}), 500


# let vercel server handle hashed interests from client
@app.route('/receive-hashed-interests', methods=['POST'])
def send_hashed_interests():
    try:
        # Check if we received any data
        if not request.data:
            return jsonify({"error": "No data provided"}), 400

        # Get the JSON data from the incoming request
        data = request.get_json(force=True)
        
        print(f"Received root data: {data}")  # Debug print

        # Return the response from the Hyle server
        return jsonify(response.json()), response.status_code
    
    except json.JSONDecodeError as e:
        return jsonify({"error": f"Invalid JSON format: {str(e)}"}), 400
    except Exception as e:
        # Handle any other errors
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(host='localhost', port=5000, debug=True)
