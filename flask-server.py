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

        # Load existing hashed interests from file
        try:
            with open('hashed-interests.json', 'r') as f:
                hashed_interests = json.load(f)
        except FileNotFoundError:
            hashed_interests = {}  # Changed to empty dict

        # Initialize the address key if it doesn't exist
        if data["address"] not in hashed_interests:
            hashed_interests[data["address"]] = {}

        # Append new hashed interests to the user data
        if "m1" in data and "m2" in data and "m3" in data and "m4" in data:
            hashed_interests[data["address"]].update({
                "m1": data["m1"],
                "m2": data["m2"], 
                "m3": data["m3"],
                "m4": data["m4"]
            })
        
        # Save updated hashed interests back to file
        with open('hashed-interests.json', 'w') as f:
            json.dump(hashed_interests, f, indent=4)

        # Added success response
        return jsonify({"message": "Hashed interests saved successfully"}), 200
    
    except json.JSONDecodeError as e:
        return jsonify({"error": f"Invalid JSON format: {str(e)}"}), 400
    except Exception as e:
        # Handle any other errors
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(host='localhost', port=5000, debug=True)
