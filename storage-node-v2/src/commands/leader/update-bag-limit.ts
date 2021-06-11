import ApiCommandBase from '../../command-base/ApiCommandBase'
import { updateStorageBucketsPerBagLimit } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'

export default class LeaderUpdateBagLimit extends ApiCommandBase {
  static description =
    'Updates StorageBucketsPerBagLimit variable in the Joystream node storage.'

  static flags = {
    limit: flags.integer({
      char: 'l',
      required: true,
      description: 'New StorageBucketsPerBagLimit value',
    }),
    ...ApiCommandBase.keyflags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateBagLimit)

    this.log('Update "Storage buckets per bag" number limit....')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const limit = flags.limit ?? 0

    const api = await this.getApi()
    await updateStorageBucketsPerBagLimit(api, account, limit)
  }
}
