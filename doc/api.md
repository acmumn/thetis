API Routes
==========

General
-------

~~All POST routes accept a body with a `Content-Type` of either `application/x-www-form-urlencoded` or `application/json`.~~

All POST routes accept a body with a `Content-Type` of `application/json` and corresponding contents. If the contents of the body are invalid, status 400 will be returned.

All routes return a body whose `Content-Type` is `application/json`.

All routes may respond with status 400 if the request body or query string are malformed or invalid. In this case, the response body will be an object describing the error, with a string in the `type` property.

All routes may respond with status 403 if an authentication token is:

-	required but not present, in which case the response body will be an object with the string `"auth_token_required"` in the `type` property.
-	expired, in which case the response body will be an object with the string `"expired_auth_token"` in the `type` property.
-	invalid, in which case the response body will be an object with the string `"invalid_auth_token"` in the `type` property.
-	insufficient to grant the required capabilities, in which case the response body will be an object with the string `"capabilities_required"` in the `type` property and an array of the required capabilities as strings in the `capabilities` property.

All routes may respond with status 500 if another error occurs. In this case, the response body will be an object describing the error with a string in the `type` property, or empty. In addition, clients should be able to handle 502, 503, or 504 statuses returned by intermediate servers.

If capabilities are specified to be required, an authentication token must be present as a cookie whose name is `auth`. Authentication tokens should be treated as opaque strings.

Authentication
--------------

### POST `/api/auth/issue`

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

### POST `/api/auth/login`

**UNIMPLEMENTED**

Causes the user to be mailed a "magic link."

#### Request Body

-	`redirect`: A URL as a string. Optional, defaults to `$BASE_URL`. The URL the magic link should redirect to after the user has logged in.
-	`student_id`: A seven-character string composed solely of ASCII decimal digits. The student ID of the user to send a link to.

#### Response

-	If all is successful, responds with status 204 and no body.
-	If the user does not have the `login` capability, responds with status 403 and a body containing an object with the string `"capabilities_required"` in the `type` property and an array of the required capabilities as strings in the `capabilities` property.
-	If the user does not exist, responds with status 404 and a body containing an object with the string `"no_such_user"` in the `type` property.

Authorization
-------------

### POST `/api/auth/check`

**UNIMPLEMENTED**

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

### POST `/api/mail/lists/:id/unsubscribe`

**TODO REVIEW AND FORMAT THIS**

**UNIMPLEMENTED**

Adds a row to the `mail_unsubscribes` table, preventing email form being sent to that address from the given mailing list. A request `Content-Type` of `application/x-www-form-urlencoded` is required. The body should contain the same `email` parameter as above.

### POST `/api/mail/enqueue`

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

### POST `/api/mail/templates/:id/render`

**TODO REVIEW AND FORMAT THIS**

**UNIMPLEMENTED**

Requires an authentication token granting admin privileges. A request `Content-Type` of `application/x-www-form-urlencoded` is required. Renders the template with the data in the body.

Other
-----

### GET `/api/ping`

#### Response

-	Always responds with status 204 and no body.
