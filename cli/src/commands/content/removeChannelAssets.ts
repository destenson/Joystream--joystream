import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import { flags } from '@oclif/command'
import { createType } from '@joystream/types'

export default class RemoveChannelAssetsCommand extends ContentDirectoryCommandBase {
  static description = 'Remove data objects associated with the channel or any of its videos.'

  static flags = {
    channelId: flags.integer({
      char: 'c',
      required: true,
      description: 'ID of the Channel',
    }),
    objectId: flags.integer({
      char: 'o',
      required: true,
      multiple: true,
      description: 'ID of an object to remove',
    }),
    context: ContentDirectoryCommandBase.channelManagementContextFlag,
  }

  async run(): Promise<void> {
    const {
      flags: { channelId, objectId: objectIds, context },
    } = this.parse(RemoveChannelAssetsCommand)
    // Context
    const account = await this.getRequiredSelectedAccount()
    const channel = await this.getApi().channelById(channelId)
    const actor = await this.getChannelManagementActor(channel, context)
    await this.requestAccountDecoding(account)

    this.jsonPrettyPrint(JSON.stringify({ channelId, assetsToRemove: objectIds }))
    await this.requireConfirmation('Do you confirm the provided input?', true)

    await this.sendAndFollowNamedTx(account, 'content', 'updateChannel', [
      actor,
      channelId,
      { assets_to_remove: createType('BTreeSet<DataObjectId>', objectIds) },
    ])
  }
}
