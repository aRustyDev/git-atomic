---
paths:
  - *.md
---

- Only apply for markdown files whose 'remotes' are one of the following
  - aRustyDev/*
  - libsdk/*
  - civicbyte/*
- follow frontmatter schema (https://schemas.arusty.dev/markdown/frontmatter/latest.schema.json)
- `.project.id` must exist and must be UUIDv4
- `.id` must exist and must be UUIDv4; it is the document specific ID
  - prefer using `.id` to reference this document in other documents; ex relating documents/files
- `.status` key enum
- `.related` is a map of key value pairs using the `<relation-type>`:`[<uuid>]`
