import { extendDebug } from 'src/Debugger'
import { FixtureRunner } from 'src/Fixture'
import { FlowProps } from 'src/Flow'
import { assert } from 'chai'
import BN from 'bn.js'
import { BondingSucceedsFixture } from 'src/fixtures/staking/BondingSucceedsFixture'
import { ValidatingSucceedsFixture } from 'src/fixtures/staking/ValidatingSucceedsFixture'

export default async function validateSucceedsInPoA({ api, query, env }: FlowProps): Promise<void> {
  const debug = extendDebug('flow: validator-set')
  debug('started')
  api.enableDebugTxLogs()

  const bondAmount = new BN(100000)

  // we are in poa
  const currentEra = api.getCurrentEra()
  assert(currentEra.isNone)

  // create keys
  const account = (await api.createKeyPairs(1))[0].key.address

  // bond Tx
  const bondingSucceedsFixture = new BondingSucceedsFixture(api, {
    stash: account,
    controller: account,
    bondAmount: bondAmount,
  })
  const fixtureRunner = new FixtureRunner(bondingSucceedsFixture)
  await fixtureRunner.run()

  // candidate validator
  const validatorCandidatingSucceedsFixture = new ValidatingSucceedsFixture(api, {
    commission: 1,
    blocked: false,
  })
  const candidationFixture = new FixtureRunner(validatorCandidatingSucceedsFixture)
  await candidationFixture.run()
}
