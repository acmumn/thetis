import os
from flask import Flask, render_template
import jinja2
from .thetisapi import ThetisBackend, BackendError

app = Flask('peleus')
app.config["SECRET_KEY"]=os.environ.get("FLASK_SECRET_KEY", os.urandom(24))

ROUTE_PREFIX = '/members'

thetis = ThetisBackend("http://nathan.members.acm.umn.edu:8080/api", "chungus")

@app.route(ROUTE_PREFIX + '/static/<path:filename>')
def serve_static(filename):
	return send_from_directory('static', filename)

@app.route(ROUTE_PREFIX + '/')
def render_root():
	return render_template("home.html")

@app.route('/')
def render_bad_conf():
	return "if you are seeing this, we mondo beefed the site.\n call me at 240 813 8477"

from . import login