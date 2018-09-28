from flask_wtf import FlaskForm
from wtforms import StringField
from wtforms.validators import DataRequired
from flask import render_template, redirect, request, flash, session
from .util import flash_form_errors
from . import app, thetis, ROUTE_PREFIX
from .thetisapi import BackendError, UnknownUserError

class UserForm(FlaskForm):
	x500 = StringField('x500', validators=[DataRequired()])

@app.route(ROUTE_PREFIX + '/user/<int:db_id>', methods=('GET', 'POST'))
def render_user(db_id):
	form = UserForm()
	if request.method=="GET":
		return render_template('user.html', db_id=db_id, form=form)
	else:
		if form.validate():
			pass
		else:
			flash_form_errors(form)
			return render_template('user.html', db_id=db_id, form=form)

# @app.route('/')