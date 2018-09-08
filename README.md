thetis
======

Configuration
-------------

Configured via either environment variables or a `.env` file. The following environment variables are used:

```
# Required
BASE_URL="https://acm.umn.edu" # Base URL for links
DATABASE_URL="mysql://root:password@localhost/acm" # MySQL database URL
JWT_SECRET="hunter2" # JWT HS512 secret
SMTP_FROM="example@gmail.com" # SMTP From header
SMTP_PASS="hunter2" # SMTP password
SMTP_USER="example@gmail.com" # SMTP username

# Optional, defaults to values here.
CAPABILITIES_FILE="capabilities.pl" # File to load capabilities from
HOST="::" # IP to bind to
PORT=8000 # Port to serve on
SMTP_ADDR="smtp.gmail.com" # SMTP server hostname
SMTP_REPLY_TO="example@gmail.com" # defaults to SMTP_FROM
SYSLOG_SERVER="" # If non-empty, the syslog server to send logs to
TEMPLATES_DIR="templates" # Directory to load web templates from
```
