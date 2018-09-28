API Routes
==========

General
-------

~~All POST routes accept a body with a `Content-Type` of either `application/x-www-form-urlencoded` or `application/json`.~~

All POST routes accept a body with a `Content-Type` of `application/json` and corresponding contents. If the contents of the body are invalid, status 400 will be returned.

All routes return a body whose `Content-Type` is `application/json`.

All routes may respond with status 400 if the request body or query string are malformed or invalid. In this case, the response body will be an object describing the error, with a string in the `type` property, or empty.

All routes may respond with status 403 if an authentication token is:

-	required but not present, in which case the response body will be an object with the string `"auth_token_required"` in the `type` property.
-	expired, in which case the response body will be an object with the string `"expired_auth_token"` in the `type` property.
-	invalid, in which case the response body will be an object with the string `"invalid_auth_token"` in the `type` property.
-	insufficient to grant the required capabilities, in which case the response body will be an object with the string `"capabilities_required"` in the `type` property and an array of the required capabilities as strings in the `capabilities` property.

All routes may respond with status 500 if another error occurs. In this case, the response body will be an object describing the error with a string in the `type` property, or empty. In addition, clients should be able to handle 502, 503, or 504 statuses returned by intermediate servers.

If capabilities are specified to be required, an authentication token must be present as a cookie whose name is `auth`. Authentication tokens should be treated as opaque strings.

Authentication
--------------

### POST `/api/thetis/auth/issue`

**TODO DISCUSS**

**UNIMPLEMENTED**

Issues a service token.

#### Required Capabilities

-	`auth.issue`

#### Request Body

-	`name`: A name as a string.
-	`capabilities`: An array of the required capabilities as strings.

#### Response

-	If all is successful, responds with status 200 and a body containing the service token as a JSON string.
-	If the user's capabilities are not a strict superset of the requested capabilities, responds with status 403 and a body containing an object with the string `"capabilities_required"` in the `type` property and an array of the required capabilities as strings in the `capabilities` property.

### POST `/api/thetis/auth/login`

Causes the user to be mailed a "magic link."

#### Request Body

-	`redirect`: A URL as a string. Optional, defaults to `$BASE_URL`. The URL the magic link should redirect to after the user has logged in.
-	`x500`: The user's X.500.

#### Response

-	If all is successful, responds with status 204 and no body.
-	If the user does not have the `login` capability, responds with status 403 and a body containing an object with the string `"capabilities_required"` in the `type` property and an array of the required capabilities as strings in the `capabilities` property.
-	If the user does not exist, responds with status 404 and a body containing an object with the string `"no_such_user"` in the `type` property.

Authorization
-------------

### POST `/api/thetis/auth/check`

Checks an authentication token for capabilities.

#### Required Capabilities

-	`capabilities.check`

#### Request Body

-	`token`: An authentication token as a string. The authentication token to check.
-	`capabilities`: An array of capabilities to require as strings.

#### Response

-	If all is successful, responds with status 200 and a body containing the object `{"type": "ok"}`.
-	If the token does not grant all of the capabilities requested, responds with status 200 and an object that would be returned from the equivalent 403 error (see the "General" section above).

Mail
----

### POST `/api/thetis/mail/enqueue`

**UNIMPLEMENTED**

#### Required Capabilities

-	`mail.send`

#### Request Body

-	`mailing_list`: The ID of the mailing list as an integer.
-	`template`: The ID of the template as an integer.
-	`data`: The data to render into the template as a JSON value.
-	`email`: The email address to send to as a string.
-	`subject`: The subject line of the email as a string.

#### Response

-	If all is successful, responds with status 202 and an empty body.
-	**TODO**: Document other errors.

### POST `/api/thetis/mail/lists/:id/unsubscribe`

**TODO REVIEW AND FORMAT THIS**

**UNIMPLEMENTED**

Adds a row to the `mail_unsubscribes` table, preventing email form being sent to that address from the given mailing list. A request `Content-Type` of `application/x-www-form-urlencoded` is required. The body should contain the same `email` parameter as above.

### POST `/api/thetis/mail/templates/:id/render`

**TODO REVIEW AND FORMAT THIS**

**UNIMPLEMENTED**

Requires an authentication token granting admin privileges. A request `Content-Type` of `application/x-www-form-urlencoded` is required. Renders the template with the data in the body.

User Management
---------------

Most of these APIs operate on the data stored in members' records. The following mapping from database schema to JSON representation (as keys on a user object) is established for use with this API:
 - `id`: The internal database ID is exposed as an integer `db_id`. This may not be null.
 - `name`: The name is exposed as the string `name`. This may not be null.
 - `x500`: The (psuedo-)x500 is exposed as the string `x500`. This may not be null.
 - `card`: The long UCard number is exposed as the string `card`. This must be exactly 17 characters. This may be null instead.
 - `email`: The email is exposed as the string `email`. This may not be null.
 - `discordID`: The discord ID is exposed as the string `discordID`. This (as opposed to an integer) is adopted for the same (bad) reasons as the official Discord API, as well as interop with said API. This may be null.

The term "canonical user object" is used to refer to a user object with all of the preceding keys set. User objects that are missing a key may be used to e.g. represent a change to make. Missing keys do not mean the value of the field is null.

### POST `/api/thetis/users/search`

**UNIMPLEMENTED**

Requires an authentication token granting `user.list`.
Takes the parameter `query`, which is a user object. Returns an object containing the following keys:

 - `found`: A list of canonical user objects, matching the `query`

Matching here means that for all user objects returned, all fields set in the `query` will match the fields set in the user object. Fields that do not have a key set in the `query` are not used to search - setting null requires the field be null, not setting the key imposes no restrictions on the field's value. In psuedocode:

```python
for user in returned:
	for key, value in query:
		user[key] == query[key]
```

An empty `query` object will return all users.

TODO: Decide if we want fuzzy matching etc

### GET `/api/thetis/users/me`

**UNIMPLEMENTED**

Requires a user authentication token.
Returns the canonical user object for the user whose token was used to make the request.

### GET `/api/thetis/users/user/<id>`

**UNIMPLEMENTED**

`id` is a base-10 integer corresponding to the `db_id` of the user you wish to access.
Requires an authentication token granting view permissions for the user.
Returns the canonical user object for the user with database id `<id>`.

### POST `/api/thetis/users/user/<id>`

**UNIMPLEMENTED**

`id` is a base-10 integer corresponding to the `db_id` of the user you wish to access.
Requires an authentication token granting the requisite `user.modify.<field>(db_id)` privileges.
Takes a parameter `update` containing a user object with one or more fields set. Updates those fields on the user with database id `<id>` and returns a canonical user object for that user.

### POST `/api/thetis/users/user/new`

**UNIMPLEMENTED**

Requires an authentication token granting `user.add`.
Takes a parameter `user` which is a user object with all fields but `db_id` set. Adds the user to the database and returns the canonical user object representing them. 

Other
-----

### GET `/api/thetis/ping`

#### Response

-	Always responds with status 204 and no body.
