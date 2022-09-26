import { extendDebug } from 'src/Debugger'
import { FlowProps } from 'src/Flow'
import { Option } from '@polkadot/types'
import { assert } from 'chai'

export default async function forceEraIsNone({ api, query, env }: FlowProps): Promise<void> {
  const debug = extendDebug('flow: validator-set')
  debug('started')
  api.enableDebugTxLogs()

  assert(api.getForceEra().isForceNone)

  const currentEra = api.getCurrentEra()
  assert(currentEra.isNone)
}
