from flask import flash

def flash_form_errors(form):
	"""Show all errors in a WTForm nicely"""
	[flash("Invalid data in item '%s': %s" % (k,", ".join(v)), 'error')
		for k,v in form.errors.items()]
