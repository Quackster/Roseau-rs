# Goal

Upgrade this project to Java 25.

The implementation must be consistent across the entire codebase. Keep formatting, naming, package structure, imports, indentation, and spacing uniform with the chosen project style. Do not leave mixed styles, unnecessary whitespace, inconsistent blank lines, or uneven spacing.

Entity persistence must use Entity4j, installed through JitPack. Configure the build so JitPack is available as a dependency repository and Entity4j is declared as the persistence dependency.

All database access must use Entity4j entity mapping. Define and use mapped entity classes for persistence operations. Do not use raw SQL anywhere in the application, including query strings, manual JDBC statements, hand-written SQL builders, migrations embedded in code, or repository methods that bypass Entity4j mapping.

Database engine selection must be configurable from `roseau.properties`. Use a generic database configuration block with one set of connection keys, not duplicated username/password/database fields per engine. The selected engine must drive the Entity4j dialect and JDBC URL for supported engines such as MySQL, PostgreSQL, MSSQL, and SQLite.

Before finishing, verify that:

- The project builds and runs on Java 25.
- Entity4j is resolved through JitPack.
- Persistence code uses Entity4j entity mapping only.
- No raw SQL remains anywhere in source or test code.
- Database engine selection is configurable through one generic database config shape.
- Formatting and spacing are consistent throughout the modified code.
