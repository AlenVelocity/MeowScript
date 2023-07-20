import * as Comlink from 'comlink'

import type { MeowScriptWorker } from './MeowScriptWorker'

export const createMeowScriptWorker = async (): Promise<
  [wrapped: MeowScriptWorker, underlying: Worker]
> => {
  // @ts-ignore
  const worker = new Worker(new URL('./MeowScriptWorker.ts', import.meta.url))

  // @ts-ignore
  const WorkerClass: new (
    ..._args: ConstructorParameters<typeof MeowScriptWorker>
  ) => // @ts-ignore
  InstanceType<Promise<MeowScriptWorker>> = Comlink.wrap<MeowScriptWorker>(worker)

  const meowScriptWorker: MeowScriptWorker = await new WorkerClass()
  await meowScriptWorker.initWasm()

  return [meowScriptWorker, worker]
}
