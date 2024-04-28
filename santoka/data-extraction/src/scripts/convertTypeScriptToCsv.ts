import { database } from '../data/2023.06.26.14.52/standalone'
import type { TableName, Record } from '../data/2023.06.26.14.52/standalone'
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

function main() {
	const separator = ' | '
	const outputFolder = path.resolve(__dirname, '..', 'data', database.id)

	const tableNames = Object.keys(database.tables) as Array<TableName>
	for (const tableName of tableNames) {
		const records = database.tables[tableName]
		const tablePath = path.join(outputFolder, `${tableName}.csv`)
		const csv = getCsvRows(records, separator).join('\n')
		fs.writeFileSync(tablePath, csv)
	}
}

function getCsvRows<R extends Record>(records: R[], separator: string): string[] {
	const columnNames = Object.keys(records[0]) as Array<keyof R>
	const headingRow = columnNames.join(separator)
	const valueRows = records.map((record) => {
		const values = columnNames.map((columnName) => {
			const value = record[columnName]
			if (typeof value === 'string') {
				return value.replaceAll('\n', ' / ')
			}
			return value
		})
		return values.join(separator)
	})
	return [headingRow, ...valueRows]
}

main()
