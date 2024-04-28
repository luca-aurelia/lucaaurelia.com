import { sleep } from './Time'

export type AsyncCallback = () => Promise<void>

export interface SequentialQueue {
	push: (taskName: string, callback: AsyncCallback) => Promise<void>
}

/*
 * This gives you a queue that runs async tasks sequentially.
 *
 * const queue = createSequentialQueue(500)
 *
 * for (let i = 0; i < 10; i++) {
 *   queue.push(async () => {
 *     console.log(`Hi from task number ${i}!`)
 *   })
 * }
 *
 * Sequential queue
 *
 * ! = when await queue.push() resolves.
 *
 * task 1 --------
 * task 2         ------
 * task 3               ------------ !
 *
 */

export function create({ timeoutMilliseconds }: { timeoutMilliseconds: number }): SequentialQueue {
	let previousPromise: Promise<void> = Promise.resolve()

	async function timeout(): Promise<typeof timedOut> {
		await sleep(timeoutMilliseconds)
		return timedOut
	}

	// Adds the callback to the queue. If you await push,
	// it'll resolve when the callback has been processed.
	async function push(taskName: string, callback: AsyncCallback) {
		previousPromise = previousPromise
			.then(async () => {
				// See which resolves first, the timeout or the callback.
				const result = await Promise.race([timeout(), callback()])

				if (result === timedOut) {
					const error = new Error(
						`Task "${taskName}" timed out after ${timeoutMilliseconds} milliseconds.`
					)
					throw error
				}
			})
			.catch((error) => {
				console.error('Error in sequential queue:', error)

				previousPromise = Promise.resolve()
			})

		await previousPromise
	}

	return { push }
}

const timedOut = Symbol('timed out')
