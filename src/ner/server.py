from flask import Flask, escape, request, jsonify
from ner import FlairNERParser

app = Flask(__name__)

parser = None

@app.before_first_request
def prepare_ner():
    global parser
    parser = FlairNERParser()

@app.route('/')
def hello():
    name = request.args.get("name", "World")
    return f'Hello, {escape(name)}!'

@app.route('/parse', methods=["POST"])
def parse():
    if request.json:
        ners = []
        for begin, end in parser.parse(request.json['data']):
            ners.append({"begin": begin, "end": end})
        print(ners)
        return jsonify(ners)
    return jsonify([{}])
