import requests
from requests.auth import AuthBase
from requests.exceptions import RequestException

class BearerAuth(AuthBase):
	def __init__(self, token):
		self.headervalue = "Bearer "+token

	def __call__(self, r):
		r.headers['Authorization'] = self.headervalue
		return r

class BackendError(Exception):
	pass

class ThetisBackend:
	def __init__(self, api_base, token):
		self.api_base = api_base
		self.token = token
		self.auth = BearerAuth(token)
		self.s = requests.Session()
		self.s.auth = self.auth

	def api_ping(self):
		try:
			return requests.get(self.api_base+"/ping").status_code == 204
		except RequestException as e:
			raise BackendError("Ping Failed") from e