from flask import Flask
from thetis import ThetisBackend, BackendError
app = Flask('peleus')

@app.route('/')
def hello_world():
    return "Goodbye, World!"