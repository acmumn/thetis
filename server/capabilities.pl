cap(Id, _) :- tag(Id, admin).
cap(Id, Cap) :- inGoodStanding(Id), grant(Cap, Tag), tag(Id, Tag).
inGoodStanding(Id) :- paid(Id), notBanned(Id).

cap(Id, door.access) :- inGoodStanding(Id).

grant(mail.send, officer).
grant(auth.issue, systems).
