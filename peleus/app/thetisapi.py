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

class BackendPermissionError(BackendError):
	pass

class UnknownUserError(BackendError):
	pass

class RealThetisBackend:
	def __init__(self, api_base, token):
		self.api_base = api_base
		self.token = token
		self.auth = BearerAuth(token) if token else None

	def _api_post(self, path, json={}, expect_code=204):
		try:
			r = requests.post(self.api_base+path, json=json, auth=self.auth)
		except RequestException as e:
			raise BackendError("Request Failed - Network Error") from e

		if r.status_code == 403:
			raise BackendPermissionError(r.json())

		if r.status_code != expect_code:
			raise BackendError("API Call Failed (unknown error; %d not 204)" % r.status_code)

	def _api_get(self, path, expect_code=204):
		try:
			r = requests.get(self.api_base+path, auth=self.auth)
		except RequestException as e:
			raise BackendError("Request Failed - Network Error") from e

		if r.status_code == 403:
			raise BackendPermissionError(r.json())

		if r.status_code != expect_code:
			raise BackendError("API Call Failed (unknown error; %d not 204)" % r.status_code)

	def api_ping(self):
		self._api_get('/ping')

	def api_issue_link(self, x500):
		self._api_post('/auth/login', json={'x500':x500})

class StubThetisBackend:
	def __init__(self, api_base, token):
		self.api_base = api_base
		self.token = token

	def api_ping(self):
		pass

	def api_issue_link(self, x500):
		pass

	def api_get_user_by_x500(self, x500):
		pass

ThetisBackend = StubThetisBackend