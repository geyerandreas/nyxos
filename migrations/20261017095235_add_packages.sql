-- Add migration script here
CREATE TABLE Packages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,          -- e.g. "My-Package"
    normalized_name TEXT UNIQUE NOT NULL -- e.g. "my-package" (PEP 503 normalization)
);

CREATE TABLE PackageFiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id INTEGER NOT NULL,
    filename TEXT NOT NULL,      -- e.g. "my_package-0.1.0-py3-none-any.whl"
    version TEXT NOT NULL,       -- e.g. "0.1.0"
    requires_python TEXT,        -- PEP 592 / PEP 503 data-requires-python
    sha256_hash TEXT NOT NULL,
    file_path TEXT NOT NULL,     -- Local path or Storage Key
    uploaded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(package_id) REFERENCES packages(id)
);
