window.SIDEBAR_ITEMS = {"constant":[["MODULE_GLOBAL_PREFIX",""]],"enum":[["AutocommitError","Error returned when the autocommit function fails"],["DbKeyPrefix",""],["DecodingError",""],["TestDbKeyPrefix",""]],"fn":[["apply_migrations","`apply_migrations` iterates from the on disk database version for the module up to `target_db_version` and executes all of the migrations that exist in the `MigrationMap`. Each migration in `MigrationMap` updates the database to have the correct on-disk structures that the code is expecting. The entire migration process is atomic (i.e migration from 0->1 and 1->2 happen atomically). This function is called before the module is initialized and as long as the correct migrations are supplied in `MigrationMap`, the module will be able to read and write from the database successfully."],["expect_write_conflict",""],["find_by_prefix_sorted_descending",""],["future_returns_shortly",""],["verify_commit",""],["verify_find_by_prefix",""],["verify_insert_elements",""],["verify_module_db",""],["verify_module_prefix",""],["verify_phantom_entry",""],["verify_prevent_dirty_reads",""],["verify_prevent_nonrepeatable_reads",""],["verify_read_own_writes",""],["verify_remove_by_prefix",""],["verify_remove_existing",""],["verify_remove_nonexisting",""],["verify_rollback_to_savepoint",""],["verify_string_prefix",""]],"mod":[["mem_impl",""],["notifications",""]],"struct":[["CommitTracker","A handle to a type-erased database implementation"],["Database",""],["DatabaseTransaction","`DatabaseTransaction` is the parent-level database transaction that can modify the database. The owner of the `DatabaseTransaction` is responsible for managing the lifetime of the `DatabaseTransaction`, either by committing the modifications to the database or rolling back the transaction. From this parent-level `DatabaseTransaction`, a `ModuleDatabaseTransaction` can be created which operates like a child transaction where the child transaction only has access to the modules database namespace."],["DatabaseVersion",""],["DatabaseVersionKey",""],["DbKeyPrefixIter","An iterator over the variants of [Self]"],["ModuleDatabaseTransaction","`ModuleDatabaseTransaction` is the public wrapper structure that allows modules to modify the database. It takes a `ISingleUseDatabaseTransaction` that handles the details of interacting with the database. The APIs that the modules are allowed to interact with are a subset of `DatabaseTransaction`, since modules do not manage the lifetime of database transactions. Committing to the database or rolling back a transaction is not exposed."],["SingleUseDatabaseTransaction","Struct that implements `ISingleUseDatabaseTransaction` and can be wrapped easier in other structs since it does not consumed `self` by move."]],"trait":[["DatabaseKey","`DatabaseKey` that represents the lookup structure for retrieving key/value pairs from the database."],["DatabaseKeyPrefix",""],["DatabaseKeyWithNotify","Marker trait for `DatabaseKey`s where `NOTIFY` is true"],["DatabaseLookup","A key that can be used to query one or more `DatabaseRecord` Extends `DatabaseKeyPrefix` to prepend the key’s prefix."],["DatabaseRecord","A key + value pair in the database with a unique prefix Extends `DatabaseKeyPrefix` to prepend the key’s prefix."],["DatabaseValue","`DatabaseValue` that represents the value structure of database records."],["IDatabase",""],["IDatabaseTransaction","Fedimint requires that the database implementation implement Snapshot Isolation. Snapshot Isolation is a database isolation level that guarantees consistent reads from the time that the snapshot was created (at transaction creation time). Transactions with Snapshot Isolation level will only commit if there has been no write to the modified keys since the snapshot (i.e. write-write conflicts are prevented)."],["ISingleUseDatabaseTransaction","`ISingleUseDatabaseTransaction` re-defines the functions from `IDatabaseTransaction` but does not consumed `self` when committing to the database. This allows for wrapper structs to more easily borrow `ISingleUseDatabaseTransaction` without needing to make additional allocations."]],"type":[["MigrationMap","MigrationMap is a BTreeMap that maps DatabaseVersions to async functions. These functions are expected to “migrate” the database from the keyed DatabaseVersion to DatabaseVersion + 1."],["PrefixStream",""]]};