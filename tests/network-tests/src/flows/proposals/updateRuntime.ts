import BN from 'bn.js'
import { Api } from '../../Api'
import { Utils } from '../../utils'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membershipModule'
import { UpdateRuntimeFixture } from '../../fixtures/proposalsModule'
import { PaidTermId } from '@joystream/types/members'
import { DbService } from '../../DbService'
import { assert } from 'chai'

export default async function updateRuntime(api: Api, env: NodeJS.ProcessEnv, db: DbService) {
  const paidTerms: PaidTermId = api.createPaidTermId(new BN(+env.MEMBERSHIP_PAID_TERMS!))
  const runtimePath: string = env.RUNTIME_WASM_PATH!

  // Pre-conditions: members and council
  const council = await api.getCouncil()
  assert(council.length)

  const proposer = council[0].member.toString()

  const updateRuntimeFixture: UpdateRuntimeFixture = new UpdateRuntimeFixture(api, proposer, runtimePath)
  await updateRuntimeFixture.runner(false)

  // Some tests after runtime update
  const thirdMemberSetFixture: BuyMembershipHappyCaseFixture = new BuyMembershipHappyCaseFixture(
    api,
    api.createKeyPairs(1).map((key) => key.address),
    paidTerms
  )
  await thirdMemberSetFixture.runner(false)
}
