from flask_wtf import FlaskForm
from wtforms import StringField
from wtforms.validators import DataRequired
from flask import render_template, redirect, request, flash, session
from .util import flash_form_errors
from . import app, thetis, ROUTE_PREFIX
from .thetisapi import BackendError, UnknownUserError

class LoginForm(FlaskForm):
	x500 = StringField('x500', validators=[DataRequired()])

@app.route(ROUTE_PREFIX + '/login', methods=('GET', 'POST'))
def render_login():
	form = LoginForm()
	if request.method=="GET":
		return render_template('login.html', form=form)
	else:
		if form.validate():
			try:
				thetis.api_issue_link(form.x500.data)
			except UnknownUserError: pass #Don't leak registered users
			except BackendError as e:
				flash('Unknown backend error ('+str(e)+'). Try again?', 'error')
				return render_template('login.html', form=form)

			return redirect('/login/ok')
		else:
			flash_form_errors(form)
			return render_template('login.html', form=form)

# @app.route('/')