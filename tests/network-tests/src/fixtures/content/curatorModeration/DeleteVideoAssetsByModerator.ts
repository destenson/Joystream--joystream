import { createType } from '@joystream/types'
import { ContentActor, CuratorGroupId } from '@joystream/types/content'
import { DataObjectId } from '@joystream/types/storage'
import { WorkerId } from '@joystream/types/working-group'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { BTreeSet } from '@polkadot/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { assert } from 'chai'
import { Api } from '../../../Api'
import { StandardizedFixture } from '../../../Fixture'
import { VideoAssetsDeletedByModeratorEventFieldsFragment } from '../../../graphql/generated/queries'
import { QueryNodeApi } from '../../../QueryNodeApi'
import { EventDetails, EventType } from '../../../types'

type VideoAssetsDeletedByModeratorEventDetails = EventDetails<EventType<'content', 'VideoAssetsDeletedByModerator'>>

export type DeleteVideoAssetsAsModeratorParams = {
  asCurator: [CuratorGroupId, WorkerId]
  videoId: number
  assetsToRemove: number[]
  rationale: string
}

export class DeleteVideoAssetsAsModeratorFixture extends StandardizedFixture {
  protected deleteVideoAssetsAsModeratorParams: DeleteVideoAssetsAsModeratorParams[]

  public constructor(
    api: Api,
    query: QueryNodeApi,
    deleteVideoAssetsAsModeratorParams: DeleteVideoAssetsAsModeratorParams[]
  ) {
    super(api, query)
    this.deleteVideoAssetsAsModeratorParams = deleteVideoAssetsAsModeratorParams
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<VideoAssetsDeletedByModeratorEventDetails> {
    return this.api.getEventDetails(result, 'content', 'VideoAssetsDeletedByModerator')
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.deleteVideoAssetsAsModeratorParams.map(async ({ asCurator }) =>
        (await this.api.query.contentWorkingGroup.workerById(asCurator[1])).role_account_id.toString()
      )
    )
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.deleteVideoAssetsAsModeratorParams.map(({ asCurator, videoId, assetsToRemove, rationale }) =>
      this.api.tx.content.deleteVideoAssetsAsModerator(
        createType<ContentActor, 'ContentActor'>('ContentActor', { Curator: asCurator }),
        videoId,
        createType<BTreeSet<DataObjectId>, 'BTreeSet<DataObjectId>'>('BTreeSet<DataObjectId>', assetsToRemove),
        rationale
      )
    )
  }

  protected assertQueryNodeEventIsValid(qEvent: VideoAssetsDeletedByModeratorEventFieldsFragment, i: number): void {
    const params = this.deleteVideoAssetsAsModeratorParams[i]
    assert.equal(qEvent.video.id, params.videoId.toString())
    assert.equal(qEvent.rationale, params.rationale)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getVideoDeletedByModeratorEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )
  }
}
