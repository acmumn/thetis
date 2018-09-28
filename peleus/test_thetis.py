from app.thetisapi import ThetisBackend, BackendError

REMOTE = "http://nathan.members.acm.umn.edu:8080/api"

thetis_noauth = ThetisBackend(REMOTE, None)
thetis_noauth.api_ping()
# thetis.api_issue_link('goess006')