cap(Id, _) :- tag(Id, admin).
cap(Id, Cap) :- inGoodStanding(Id), tagCap(Cap, Tag), tag(Id, Tag).
cap(Id, Cap) :- inGoodStanding(Id), memberCap(Cap).
cap(Id, Cap) :- tag(Id, officer), committee(Tag), tagCap(Cap, Tag).

inGoodStanding(Id) :- paid(Id), notBanned(Id).

cap(Id, mail.lists.users.add(Id)).
cap(Id, mail.lists.users.delete(Id)).
cap(Id, user.modify.email(Id)).
cap(Id, user.modify.name(Id)).

committee(administrative).
committee(financial).
committee(pr).
committee(systems).

capAlsoExists(tags.add).
capAlsoExists(tags.delete).
capAlsoExists(mail.send.queue_count).

memberCap(auth.issue).
memberCap(discord.access).
memberCap(door.access).
memberCap(website.access).

tagCap(mail.list.users.add,    administrative).
tagCap(mail.list.users.delete, administrative).
tagCap(user.list,              administrative).

tagCap(user.list,          financial).
tagCap(user.payments.list, financial).

tagCap(user.bans.add,        officer).
tagCap(user.bans.edit_notes, officer).
tagCap(user.bans.invalidate, officer).

tagCap(mail.global_unsub.add,      pr).
tagCap(mail.global_unsub.list,     pr).
tagCap(mail.global_unsub.delete,   pr).
tagCap(mail.lists.add,             pr).
tagCap(mail.lists.list,            pr).
tagCap(mail.lists.delete,          pr).
tagCap(mail.lists.users.add(_),    pr).
tagCap(mail.lists.users.list,      pr).
tagCap(mail.lists.users.delete(_), pr).
tagCap(mail.send,                  pr).
tagCap(mail.template.add,          pr).
tagCap(mail.template.edit,         pr).
tagCap(mail.template.delete,       pr).
tagCap(mail.template.list,         pr).
tagCap(mail.template.render(_),    pr).
tagCap(user.add,                   pr).
tagCap(user.list,                  pr).
tagCap(user.modify.card(_),        pr).
tagCap(user.modify.email(_),       pr).
tagCap(user.modify.name(_),        pr).
tagCap(user.modify.studentId(_),   pr).
tagCap(user.modify.x500(_),        pr).
tagCap(user.payments.add,          pr).
tagCap(user.payments.delete,       pr).
tagCap(user.payments.list,         pr).
tagCap(user.payments.modify,       pr).
