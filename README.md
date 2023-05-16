# mpw-webapp
A web-frontend for the [masterpassword](https://masterpasswordapp.com) (now known as [spectre](https://spectre.app)) algorithm using browser local storage. Implemented in rust using the [leptos](https://github.com/leptos-rs/leptos) framework and compiled to WASM.

## Features
- Implement using the leptos framework for the rust programming language and compiled to WASM
- Stores usernames (cleartext) and site names, password types, and counters (encrypted) in the browser's local storage for convenience
- No communication outwards (in particular, there is no external webserver storing anything)
- Quick filter for site passwords
- Uses [magic crypt](https://github.com/magiclen/rust-magiccrypt) for encrypting the sites, password types, and counter locally
- Relies on [bootstrap](https://getbootstrap.com/) for styling and uses [fontawesome](https://fontawesome.com/) icons
    
## Disclaimer regarding
For anyone not already using the `masterpasswordapp` algorithm let me note that **I do not want to encourage you to use deterministic password generators** do so due to general security concerns, see https://tonyarcieri.com/4-fatal-flaws-in-deterministic-password-managers and https://www.reddit.com/r/privacy/comments/5loz6m/comment/dbxe7gg/. 

In particular, note that my particular implementation stores the username used for generating the password in clear text and the site-data for which passwords shall be generated as an encrypted string, increasing the general attack surface (for the benefit of convenience in using it as a password store).
