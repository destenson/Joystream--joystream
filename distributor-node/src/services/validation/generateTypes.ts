import fs from 'fs'
import path from 'path'
import { compile } from 'json-schema-to-typescript'
import { schemas } from './schemas'

// eslint-disable-next-line @typescript-eslint/no-var-requires
const prettierConfig = require('@joystream/prettier-config')

Object.entries(schemas).forEach(([schemaKey, schema]) => {
  compile(schema, `${schemaKey}Json`, { style: prettierConfig }).then((output) =>
    fs.writeFileSync(path.resolve(__dirname, `../../types/generated/${schemaKey}Json.d.ts`), output)
  )
})