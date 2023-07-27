window.SIDEBAR_ITEMS = {"constant":[["CLIENT_CONFIG","Client configuration file"],["CLIENT_CONNECT_FILE","Client connection string file"],["CODE_VERSION","Version of the server code (should be the same among peers)"],["CONSENSUS_CONFIG","Server consensus-only configurable file"],["DB_FILE","Database file name"],["JSON_EXT",""],["LOCAL_CONFIG","Server locally configurable file"],["PLAINTEXT_PASSWORD","Plain-text stored password, used to restart the server without having to send a password in via the API"],["PRIVATE_CONFIG","Server encrypted private keys file"],["SALT_FILE","Salt backup for combining with the private key"]],"fn":[["read_server_config","Reads the server from the local, private, and consensus cfg files"],["write_server_config","Writes the server into configuration files (private keys encrypted)"]]};