cap(Id, _) :- role(Id, admin).

cap(Id, 'door_access') :- paid(Id), notBanned(Id).
