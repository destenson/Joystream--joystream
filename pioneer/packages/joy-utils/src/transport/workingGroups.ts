import { Option } from '@polkadot/types/';
import { Balance } from '@polkadot/types/interfaces';
import BaseTransport from './base';
import { ApiPromise } from '@polkadot/api';
import MembersTransport from './members';
import { SingleLinkedMapEntry } from '../index';
import { Worker, WorkerId, Opening as WGOpening, Application as WGApplication } from '@joystream/types/working-group';
import { apiModuleByGroup } from '../consts/workingGroups';
import { WorkingGroupKeys } from '@joystream/types/common';
import { WorkerData, OpeningData, ParsedApplication } from '../types/workingGroups';
import { OpeningId, ApplicationId, Opening, Application, ActiveOpeningStageKey } from '@joystream/types/hiring';
import { MultipleLinkedMapEntry } from '../LinkedMapEntry';
import { Stake, StakeId } from '@joystream/types/stake';
import { RewardRelationshipId, RewardRelationship } from '@joystream/types/recurring-rewards';

export default class WorkingGroupsTransport extends BaseTransport {
  private membersT: MembersTransport;

  constructor (api: ApiPromise, membersTransport: MembersTransport) {
    super(api);
    this.membersT = membersTransport;
  }

  protected queryByGroup (group: WorkingGroupKeys) {
    const module = apiModuleByGroup[group];
    return this.api.query[module];
  }

  public async groupMemberById (group: WorkingGroupKeys, workerId: number): Promise<WorkerData | null> {
    const workerLink = new SingleLinkedMapEntry(
      Worker,
      await this.queryByGroup(group).workerById(workerId)
    );
    const worker = workerLink.value;

    if (!worker.is_active) {
      return null;
    }

    const stake = worker.role_stake_profile.isSome
      ? (await this.stakeValue(worker.role_stake_profile.unwrap().stake_id)).toNumber()
      : undefined;

    const reward = worker.reward_relationship.isSome
      ? (await this.rewardRelationship(worker.reward_relationship.unwrap()))
      : undefined;

    const profile = await this.membersT.expectedMemberProfile(worker.member_id);

    return { group, workerId, worker, profile, stake, reward };
  }

  public async currentLead (group: WorkingGroupKeys): Promise<WorkerData | null> {
    const optLeadId = (await this.queryByGroup(group).currentLead()) as Option<WorkerId>;

    if (!optLeadId.isSome) {
      return null;
    }

    const leadWorkerId = optLeadId.unwrap().toNumber();

    return this.groupMemberById(group, leadWorkerId);
  }

  public async allOpenings (group: WorkingGroupKeys): Promise<OpeningData[]> {
    const nextId = (await this.queryByGroup(group).nextOpeningId()) as OpeningId;

    if (nextId.eq(0)) {
      return [];
    }

    const query = this.queryByGroup(group).openingById();
    const result = new MultipleLinkedMapEntry(OpeningId, WGOpening, await query);

    const openingsData: OpeningData[] = [];
    for (const [index, opening] of Object.entries(result.linked_values.toArray())) {
      const id = result.linked_keys[parseInt(index)];
      const hiringId = opening.hiring_opening_id;
      const hiringOpening = (new SingleLinkedMapEntry(Opening, await this.hiring.openingById(hiringId))).value;
      openingsData.push({ id, opening, hiringOpening });
    }

    return openingsData;
  }

  public async activeOpenings (group: WorkingGroupKeys, substage?: ActiveOpeningStageKey) {
    return (await this.allOpenings(group))
      .filter(od =>
        od.hiringOpening.stage.isOfType('Active') &&
        (!substage || od.hiringOpening.stage.asType('Active').stage.isOfType(substage))
      );
  }

  async wgApplicationById (group: WorkingGroupKeys, wgApplicationId: number | ApplicationId): Promise<WGApplication> {
    const nextAppId = await this.queryByGroup(group).nextApplicationId() as ApplicationId;

    if (wgApplicationId < 0 || wgApplicationId >= nextAppId.toNumber()) {
      throw new Error(`Invalid working group application ID (${wgApplicationId})!`);
    }

    return new SingleLinkedMapEntry(
      WGApplication,
      await this.queryByGroup(group).applicationById(wgApplicationId)
    ).value;
  }

  protected async hiringApplicationById (id: number | ApplicationId): Promise<Application> {
    return new SingleLinkedMapEntry(
      Application,
      await this.hiring.applicationById(id)
    ).value;
  }

  protected async stakeValue (stakeId: StakeId): Promise<Balance> {
    return new SingleLinkedMapEntry(
      Stake,
      await this.stake.stakes(stakeId)
    ).value.value;
  }

  protected async rewardRelationship (relationshipId: RewardRelationshipId): Promise<RewardRelationship> {
    return new SingleLinkedMapEntry(
      RewardRelationship,
      await this.recurringRewards.rewardRelationships(relationshipId)
    ).value;
  }

  protected async parseApplication (wgApplicationId: number, wgApplication: WGApplication): Promise<ParsedApplication> {
    const appId = wgApplication.application_id;
    const application = await this.hiringApplicationById(appId);

    const { active_role_staking_id: roleStakingId, active_application_staking_id: appStakingId } = application;

    return {
      wgApplicationId,
      applicationId: appId.toNumber(),
      member: await this.membersT.expectedMemberProfile(wgApplication.member_id),
      roleAccout: wgApplication.role_account_id,
      stakes: {
        application: appStakingId.isSome ? (await this.stakeValue(appStakingId.unwrap())).toNumber() : 0,
        role: roleStakingId.isSome ? (await this.stakeValue(roleStakingId.unwrap())).toNumber() : 0
      },
      humanReadableText: application.human_readable_text.toString(),
      stage: application.stage
    };
  }

  async parsedApplicationById (group: WorkingGroupKeys, wgApplicationId: number): Promise<ParsedApplication> {
    const wgApplication = await this.wgApplicationById(group, wgApplicationId);
    return await this.parseApplication(wgApplicationId, wgApplication);
  }

  async openingApplications (group: WorkingGroupKeys, wgOpeningId: number): Promise<ParsedApplication[]> {
    const applications: ParsedApplication[] = [];

    const nextAppId = await this.queryByGroup(group).nextApplicationId() as ApplicationId;
    for (let i = 0; i < nextAppId.toNumber(); i++) {
      const wgApplication = await this.wgApplicationById(group, i);
      if (wgApplication.opening_id.toNumber() !== wgOpeningId) {
        continue;
      }
      applications.push(await this.parseApplication(i, wgApplication));
    }

    return applications;
  }

  async openingActiveApplications (group: WorkingGroupKeys, wgOpeningId: number): Promise<ParsedApplication[]> {
    return (await this.openingApplications(group, wgOpeningId))
      .filter(a => a.stage.isOfType('Active'));
  }
}
