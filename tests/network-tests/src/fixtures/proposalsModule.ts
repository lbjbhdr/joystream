import { Api, WorkingGroups } from '../Api'
import { v4 as uuid } from 'uuid'
import BN from 'bn.js'
import { ProposalId } from '@joystream/types/proposals'
import { Fixture } from '../IFixture'
import { assert } from 'chai'
import { ApplicationId, OpeningId } from '@joystream/types/hiring'
import { WorkerId } from '@joystream/types/working-group'
import { Utils } from '../utils'

export class CreateWorkingGroupLeaderOpeningFixture implements Fixture {
  private api: Api
  private proposer: string
  private applicationStake: BN
  private roleStake: BN
  private workingGroup: string

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, applicationStake: BN, roleStake: BN, workingGroup: string) {
    this.api = api
    this.proposer = proposer
    this.applicationStake = applicationStake
    this.roleStake = roleStake
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing working group lead opening proposal ' + uuid().substring(0, 8)

    // Proposal stake calculation
    const proposalStake: BN = new BN(100000)
    const proposalFee: BN = this.api.estimateProposeCreateWorkingGroupLeaderOpeningFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeCreateWorkingGroupLeaderOpening({
      account: this.proposer,
      title: proposalTitle,
      description: description,
      proposalStake: proposalStake,
      actiavteAt: 'CurrentBlock',
      maxActiveApplicants: new BN(10),
      maxReviewPeriodLength: new BN(32),
      applicationStakingPolicyAmount: this.applicationStake,
      applicationCrowdedOutUnstakingPeriodLength: new BN(1),
      applicationReviewPeriodExpiredUnstakingPeriodLength: new BN(1),
      roleStakingPolicyAmount: this.roleStake,
      roleCrowdedOutUnstakingPeriodLength: new BN(1),
      roleReviewPeriodExpiredUnstakingPeriodLength: new BN(1),
      slashableMaxCount: new BN(1),
      slashableMaxPercentPtsPerTime: new BN(100),
      fillOpeningSuccessfulApplicantApplicationStakeUnstakingPeriod: new BN(1),
      fillOpeningFailedApplicantApplicationStakeUnstakingPeriod: new BN(1),
      fillOpeningFailedApplicantRoleStakeUnstakingPeriod: new BN(1),
      terminateApplicationStakeUnstakingPeriod: new BN(1),
      terminateRoleStakeUnstakingPeriod: new BN(1),
      exitRoleApplicationStakeUnstakingPeriod: new BN(1),
      exitRoleStakeUnstakingPeriod: new BN(1),
      text: uuid().substring(0, 8),
      workingGroup: this.workingGroup,
    })

    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class BeginWorkingGroupLeaderApplicationReviewFixture implements Fixture {
  private api: Api
  private proposer: string
  private openingId: OpeningId
  private workingGroup: string

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, openingId: OpeningId, workingGroup: string) {
    this.api = api
    this.proposer = proposer
    this.openingId = openingId
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing begin working group lead application review proposal ' + uuid().substring(0, 8)

    // Proposal stake calculation
    const proposalStake: BN = new BN(25000)
    const proposalFee: BN = this.api.estimateProposeBeginWorkingGroupLeaderApplicationReviewFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeBeginWorkingGroupLeaderApplicationReview(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      this.openingId,
      this.workingGroup
    )
    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class FillLeaderOpeningProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private applicationId: ApplicationId
  private firstRewardInterval: BN
  private rewardInterval: BN
  private payoutAmount: BN
  private openingId: OpeningId
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(
    api: Api,
    proposer: string,
    applicationId: ApplicationId,
    firstRewardInterval: BN,
    rewardInterval: BN,
    payoutAmount: BN,
    openingId: OpeningId,
    workingGroup: WorkingGroups
  ) {
    this.api = api
    this.proposer = proposer
    this.applicationId = applicationId
    this.firstRewardInterval = firstRewardInterval
    this.rewardInterval = rewardInterval
    this.payoutAmount = payoutAmount
    this.openingId = openingId
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing fill opening proposal ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)

    // Proposal stake calculation
    const proposalStake: BN = new BN(50000)
    const proposalFee: BN = this.api.estimateProposeFillLeaderOpeningFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    const now: BN = await this.api.getBestBlock()

    // Proposal creation
    const result = await this.api.proposeFillLeaderOpening({
      account: this.proposer,
      title: proposalTitle,
      description: description,
      proposalStake: proposalStake,
      openingId: this.openingId,
      successfulApplicationId: this.applicationId,
      amountPerPayout: this.payoutAmount,
      nextPaymentAtBlock: now.add(this.firstRewardInterval),
      payoutInterval: this.rewardInterval,
      workingGroup: workingGroupString,
    })

    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class TerminateLeaderRoleProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private slash: boolean
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, slash: boolean, workingGroup: WorkingGroups) {
    this.api = api
    this.proposer = proposer
    this.slash = slash
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing begin working group lead application review proposal ' + uuid().substring(0, 8)
    const rationale: string = 'Testing leader termination ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)
    // assert worker exists
    const workerId: WorkerId = (await this.api.getLeadWorkerId(this.workingGroup))!

    // Proposal stake calculation
    const proposalStake: BN = new BN(100000)
    const proposalFee: BN = this.api.estimateProposeTerminateLeaderRoleFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeTerminateLeaderRole(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      workerId,
      rationale,
      this.slash,
      workingGroupString
    )
    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class SetLeaderRewardProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private payoutAmount: BN
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, payoutAmount: BN, workingGroup: WorkingGroups) {
    this.api = api
    this.proposer = proposer
    this.payoutAmount = payoutAmount
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing set leader reward proposal ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)
    // assert worker exists?
    const workerId: WorkerId = (await this.api.getLeadWorkerId(this.workingGroup))!

    // Proposal stake calculation
    const proposalStake: BN = new BN(50000)
    const proposalFee: BN = this.api.estimateProposeLeaderRewardFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeLeaderReward(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      workerId,
      this.payoutAmount,
      workingGroupString
    )

    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class DecreaseLeaderStakeProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private stakeDecrement: BN
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, stakeDecrement: BN, workingGroup: WorkingGroups) {
    this.api = api
    this.proposer = proposer
    this.stakeDecrement = stakeDecrement
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing decrease leader stake proposal ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)
    // assert worker exists ?
    const workerId: WorkerId = (await this.api.getLeadWorkerId(this.workingGroup))!

    // Proposal stake calculation
    const proposalStake: BN = new BN(50000)
    const proposalFee: BN = this.api.estimateProposeDecreaseLeaderStakeFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeDecreaseLeaderStake(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      workerId,
      this.stakeDecrement,
      workingGroupString
    )

    this.result = this.api.expectProposalCreatedEvent(result.events)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class SlashLeaderProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private slashAmount: BN
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, slashAmount: BN, workingGroup: WorkingGroups) {
    this.api = api
    this.proposer = proposer
    this.slashAmount = slashAmount
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing slash leader stake proposal ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)
    const workerId: WorkerId = (await this.api.getLeadWorkerId(this.workingGroup))!

    // Proposal stake calculation
    const proposalStake: BN = new BN(50000)
    const proposalFee: BN = this.api.estimateProposeSlashLeaderStakeFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeSlashLeaderStake(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      workerId,
      this.slashAmount,
      workingGroupString
    )
    this.result = this.api.expectProposalCreatedEvent(result.events)
    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class WorkingGroupMintCapacityProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private mintCapacity: BN
  private workingGroup: WorkingGroups

  private result: ProposalId | undefined

  constructor(api: Api, proposer: string, mintCapacity: BN, workingGroup: WorkingGroups) {
    this.api = api
    this.proposer = proposer
    this.mintCapacity = mintCapacity
    this.workingGroup = workingGroup
  }

  public getCreatedProposalId(): ProposalId | undefined {
    return this.result
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing working group mint capacity proposal ' + uuid().substring(0, 8)
    const workingGroupString: string = this.api.getWorkingGroupString(this.workingGroup)

    // Proposal stake calculation
    const proposalStake: BN = new BN(50000)
    const proposalFee: BN = this.api.estimateProposeWorkingGroupMintCapacityFee()
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))

    // Proposal creation
    const result = await this.api.proposeWorkingGroupMintCapacity(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      this.mintCapacity,
      workingGroupString
    )
    this.result = this.api.expectProposalCreatedEvent(result.events)
    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class ElectionParametersProposalFixture implements Fixture {
  private api: Api
  private proposerAccount: string

  constructor(api: Api, proposerAccount: string) {
    this.api = api
    this.proposerAccount = proposerAccount
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing validator count proposal ' + uuid().substring(0, 8)

    // Council accounts enough balance to ensure they can vote
    const councilAccounts = await this.api.getCouncilAccounts()
    const runtimeVoteFee: BN = this.api.estimateVoteForProposalFee()
    this.api.treasuryTransferBalanceToAccounts(councilAccounts, runtimeVoteFee)

    const announcingPeriod: BN = new BN(28800)
    const votingPeriod: BN = new BN(14400)
    const revealingPeriod: BN = new BN(14400)
    const councilSize: BN = await this.api.getCouncilSize()
    const candidacyLimit: BN = await this.api.getCandidacyLimit()
    const newTermDuration: BN = new BN(144000)
    const minCouncilStake: BN = await this.api.getMinCouncilStake()
    const minVotingStake: BN = await this.api.getMinVotingStake()

    // Proposal stake calculation
    // Required stake is hardcoded in runtime-module (but not available as const)
    const proposalStake: BN = new BN(200000)
    const proposalFee: BN = this.api.estimateProposeElectionParametersFee(
      description,
      description,
      proposalStake,
      announcingPeriod,
      votingPeriod,
      revealingPeriod,
      councilSize,
      candidacyLimit,
      newTermDuration,
      minCouncilStake,
      minVotingStake
    )

    await this.api.treasuryTransferBalance(this.proposerAccount, proposalFee.add(proposalStake))

    // Proposal creation
    const proposedAnnouncingPeriod: BN = announcingPeriod.subn(1)
    const proposedVotingPeriod: BN = votingPeriod.addn(1)
    const proposedRevealingPeriod: BN = revealingPeriod.addn(1)
    const proposedCouncilSize: BN = councilSize.addn(1)
    const proposedCandidacyLimit: BN = candidacyLimit.addn(1)
    const proposedNewTermDuration: BN = newTermDuration.addn(1)
    const proposedMinCouncilStake: BN = minCouncilStake.addn(1)
    const proposedMinVotingStake: BN = minVotingStake.addn(1)

    const proposalCreationResult = await this.api.proposeElectionParameters(
      this.proposerAccount,
      proposalTitle,
      description,
      proposalStake,
      proposedAnnouncingPeriod,
      proposedVotingPeriod,
      proposedRevealingPeriod,
      proposedCouncilSize,
      proposedCandidacyLimit,
      proposedNewTermDuration,
      proposedMinCouncilStake,
      proposedMinVotingStake
    )
    const proposalNumber = this.api.expectProposalCreatedEvent(proposalCreationResult.events)

    // Approving the proposal
    this.api.batchApproveProposal(proposalNumber)
    await this.api.waitForProposalToFinalize(proposalNumber)

    // Assertions
    const newAnnouncingPeriod: BN = await this.api.getAnnouncingPeriod()
    const newVotingPeriod: BN = await this.api.getVotingPeriod()
    const newRevealingPeriod: BN = await this.api.getRevealingPeriod()
    const newCouncilSize: BN = await this.api.getCouncilSize()
    const newCandidacyLimit: BN = await this.api.getCandidacyLimit()
    const newNewTermDuration: BN = await this.api.getNewTermDuration()
    const newMinCouncilStake: BN = await this.api.getMinCouncilStake()
    const newMinVotingStake: BN = await this.api.getMinVotingStake()
    assert(
      proposedAnnouncingPeriod.eq(newAnnouncingPeriod),
      `Announcing period has unexpected value ${newAnnouncingPeriod}, expected ${proposedAnnouncingPeriod}`
    )
    assert(
      proposedVotingPeriod.eq(newVotingPeriod),
      `Voting period has unexpected value ${newVotingPeriod}, expected ${proposedVotingPeriod}`
    )
    assert(
      proposedRevealingPeriod.eq(newRevealingPeriod),
      `Revealing has unexpected value ${newRevealingPeriod}, expected ${proposedRevealingPeriod}`
    )
    assert(
      proposedCouncilSize.eq(newCouncilSize),
      `Council size has unexpected value ${newCouncilSize}, expected ${proposedCouncilSize}`
    )
    assert(
      proposedCandidacyLimit.eq(newCandidacyLimit),
      `Candidacy limit has unexpected value ${newCandidacyLimit}, expected ${proposedCandidacyLimit}`
    )
    assert(
      proposedNewTermDuration.eq(newNewTermDuration),
      `New term duration has unexpected value ${newNewTermDuration}, expected ${proposedNewTermDuration}`
    )
    assert(
      proposedMinCouncilStake.eq(newMinCouncilStake),
      `Min council stake has unexpected value ${newMinCouncilStake}, expected ${proposedMinCouncilStake}`
    )
    assert(
      proposedMinVotingStake.eq(newMinVotingStake),
      `Min voting stake has unexpected value ${newMinVotingStake}, expected ${proposedMinVotingStake}`
    )
    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class SpendingProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private spendingBalance: BN
  private mintCapacity: BN

  constructor(api: Api, proposer: string, spendingBalance: BN, mintCapacity: BN) {
    this.api = api
    this.proposer = proposer
    this.spendingBalance = spendingBalance
    this.mintCapacity = mintCapacity
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const description = 'spending proposal which is used for API network testing with some mock data'
    const runtimeVoteFee: BN = this.api.estimateVoteForProposalFee()

    // Topping the balances
    const proposalStake: BN = new BN(25000)
    const runtimeProposalFee: BN = this.api.estimateProposeSpendingFee(
      description,
      description,
      proposalStake,
      this.spendingBalance,
      this.proposer
    )
    await this.api.treasuryTransferBalance(this.proposer, runtimeProposalFee.add(proposalStake))
    const councilAccounts = await this.api.getCouncilAccounts()
    this.api.treasuryTransferBalanceToAccounts(councilAccounts, runtimeVoteFee)
    await this.api.sudoSetCouncilMintCapacity(this.mintCapacity)

    const fundingRecipient = this.api.createKeyPairs(1)[0].address

    // Proposal creation
    const result = await this.api.proposeSpending(
      this.proposer,
      'testing spending' + uuid().substring(0, 8),
      'spending to test proposal functionality' + uuid().substring(0, 8),
      proposalStake,
      this.spendingBalance,
      fundingRecipient
    )
    const proposalNumber: ProposalId = this.api.expectProposalCreatedEvent(result.events)

    // Approving spending proposal
    const balanceBeforeMinting: BN = await this.api.getBalance(fundingRecipient)
    this.api.batchApproveProposal(proposalNumber)
    await this.api.waitForProposalToFinalize(proposalNumber)

    const balanceAfterMinting: BN = await this.api.getBalance(fundingRecipient)
    assert(
      balanceAfterMinting.eq(balanceBeforeMinting.add(this.spendingBalance)),
      `member ${fundingRecipient} has unexpected balance ${balanceAfterMinting}, expected ${balanceBeforeMinting.add(
        this.spendingBalance
      )}`
    )
    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class TextProposalFixture implements Fixture {
  private api: Api
  private proposer: string

  constructor(api: Api, proposer: string) {
    this.api = api
    this.proposer = proposer
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing text proposal ' + uuid().substring(0, 8)
    const proposalText: string = 'Text of the testing proposal ' + uuid().substring(0, 8)
    const runtimeVoteFee: BN = this.api.estimateVoteForProposalFee()
    const councilAccounts = await this.api.getCouncilAccounts()
    await this.api.treasuryTransferBalanceToAccounts(councilAccounts, runtimeVoteFee)

    // Proposal stake calculation
    const proposalStake: BN = new BN(25000)
    const runtimeProposalFee: BN = this.api.estimateProposeTextFee(
      proposalStake,
      description,
      description,
      proposalText
    )
    await this.api.treasuryTransferBalance(this.proposer, runtimeProposalFee.add(proposalStake))

    // Proposal creation

    const result = await this.api.proposeText(this.proposer, proposalStake, proposalTitle, description, proposalText)
    const proposalNumber: ProposalId = this.api.expectProposalCreatedEvent(result.events)

    // Approving text proposal
    this.api.batchApproveProposal(proposalNumber)
    await this.api.waitForProposalToFinalize(proposalNumber)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class ValidatorCountProposalFixture implements Fixture {
  private api: Api
  private proposer: string
  private validatorCountIncrement: BN

  constructor(api: Api, proposer: string, validatorCountIncrement: BN) {
    this.api = api
    this.proposer = proposer
    this.validatorCountIncrement = validatorCountIncrement
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const proposalTitle: string = 'Testing proposal ' + uuid().substring(0, 8)
    const description: string = 'Testing validator count proposal ' + uuid().substring(0, 8)
    const runtimeVoteFee: BN = this.api.estimateVoteForProposalFee()
    const councilAccounts = await this.api.getCouncilAccounts()
    await this.api.treasuryTransferBalanceToAccounts(councilAccounts, runtimeVoteFee)

    // Proposal stake calculation
    const proposalStake: BN = new BN(100000)
    const proposalFee: BN = this.api.estimateProposeValidatorCountFee(description, description, proposalStake)
    await this.api.treasuryTransferBalance(this.proposer, proposalFee.add(proposalStake))
    const validatorCount: BN = await this.api.getValidatorCount()

    // Proposal creation
    const proposedValidatorCount: BN = validatorCount.add(this.validatorCountIncrement)
    const result = await this.api.proposeValidatorCount(
      this.proposer,
      proposalTitle,
      description,
      proposalStake,
      proposedValidatorCount
    )
    const proposalNumber: ProposalId = this.api.expectProposalCreatedEvent(result.events)

    // Approving the proposal
    this.api.batchApproveProposal(proposalNumber)
    await this.api.waitForProposalToFinalize(proposalNumber)

    const newValidatorCount: BN = await this.api.getValidatorCount()
    assert(
      proposedValidatorCount.eq(newValidatorCount),
      `Validator count has unexpeccted value ${newValidatorCount}, expected ${proposedValidatorCount}`
    )
    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class UpdateRuntimeFixture implements Fixture {
  private api: Api
  private proposer: string
  private runtimePath: string

  constructor(api: Api, proposer: string, runtimePath: string) {
    this.api = api
    this.proposer = proposer
    this.runtimePath = runtimePath
  }

  public async runner(expectFailure: boolean): Promise<void> {
    // Setup
    const runtime: string = Utils.readRuntimeFromFile(this.runtimePath)
    const description = 'runtime upgrade proposal which is used for API network testing'
    const runtimeVoteFee: BN = this.api.estimateVoteForProposalFee()

    // Topping the balances
    const proposalStake: BN = new BN(1000000)
    const runtimeProposalFee: BN = this.api.estimateProposeRuntimeUpgradeFee(
      proposalStake,
      description,
      description,
      runtime
    )
    this.api.treasuryTransferBalance(this.proposer, runtimeProposalFee.add(proposalStake))
    const councilAccounts = await this.api.getCouncilAccounts()
    this.api.treasuryTransferBalanceToAccounts(councilAccounts, runtimeVoteFee)

    // Proposal creation
    const result = await this.api.proposeRuntime(
      this.proposer,
      proposalStake,
      'testing runtime' + uuid().substring(0, 8),
      'runtime to test proposal functionality' + uuid().substring(0, 8),
      runtime
    )
    const proposalNumber: ProposalId = this.api.expectProposalCreatedEvent(result.events)

    // Approving runtime update proposal
    this.api.batchApproveProposal(proposalNumber)
    await this.api.waitForProposalToFinalize(proposalNumber)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}

export class VoteForProposalFixture implements Fixture {
  private api: Api
  private proposalNumber: ProposalId

  constructor(api: Api, proposalNumber: ProposalId) {
    this.api = api
    this.proposalNumber = proposalNumber
  }

  public async runner(expectFailure: boolean): Promise<void> {
    const proposalVoteFee: BN = this.api.estimateVoteForProposalFee()
    const councilAccounts = await this.api.getCouncilAccounts()
    this.api.treasuryTransferBalanceToAccounts(councilAccounts, proposalVoteFee)

    // Approving the proposal
    this.api.batchApproveProposal(this.proposalNumber)
    await this.api.waitForProposalToFinalize(this.proposalNumber)

    if (expectFailure) {
      throw new Error('Successful fixture run while expecting failure')
    }
  }
}
