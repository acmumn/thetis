cap(Id, _) :- tag(Id, admin).
cap(Id, Cap) :- inGoodStanding(Id), grant(Cap, Tag), tag(Id, Tag).
cap(Id, Cap) :- inGoodStanding(Id), memberCap(Cap).
cap(Id, Cap) :- tag(Id, officer), committee(Tag), grant(Cap, Tag).

inGoodStanding(Id) :- paid(Id), notBanned(Id).

committee(administrative).
committee(financial).
committee(pr).
committee(systems).

capAlsoExists(tags.add).
capAlsoExists(tags.remove).
capAlsoExists(mail.send.queue_count).

memberCap(auth.issue).
memberCap(discord.access).
memberCap(door.access).
memberCap(website.access).

grant(mail.list.users.add,    administrative).
grant(mail.list.users.remove, administrative).
grant(user.list,              administrative).

grant(user.list,          financial).
grant(user.payments.list, financial).

grant(user.bans.add,        officer).
grant(user.bans.edit_notes, officer).
grant(user.bans.invalidate, officer).

grant(mail.global_unsub.add,    pr).
grant(mail.global_unsub.list,   pr).
grant(mail.global_unsub.remove, pr).
grant(mail.lists.add,           pr).
grant(mail.lists.list,          pr).
grant(mail.lists.remove,        pr).
grant(mail.lists.users.add,     pr).
grant(mail.lists.users.list,    pr).
grant(mail.lists.users.remove,  pr).
grant(mail.send,                pr).
grant(mail.template.add,        pr).
grant(mail.template.edit,       pr).
grant(mail.template.delete,     pr).
grant(mail.template.list,       pr).
grant(mail.template.render,     pr).
grant(user.add,                 pr).
grant(user.list,                pr).
grant(user.modify.card,         pr).
grant(user.modify.email,        pr).
grant(user.modify.name,         pr).
grant(user.modify.studentId,    pr).
grant(user.modify.x500,         pr).
grant(user.payments.add,        pr).
grant(user.payments.delete,     pr).
grant(user.payments.list,       pr).
grant(user.payments.modify,     pr).
