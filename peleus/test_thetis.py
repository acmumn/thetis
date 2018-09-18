from thetis import ThetisBackend, BackendError

thetis = ThetisBackend("http://argo.acm.umn.edu:9999/api/", "chungus")
thetis.api_ping()