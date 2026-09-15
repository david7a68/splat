---
splat_schema: 1
splat_id: fb287f89-1aaf-52e0-8b99-fc26dab375b6
splat_kind: item
splat_key: dnd2024:items/pipes_of_the_sewers
splat_name: Pipes of the Sewers
splat_origin:
  key: dnd2024:reference/magic_item/pipes_of_the_sewers
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/pipes_of_the_sewers
  implementation: dnd2024:item_pipes_of_the_sewers
  parameters:
    equipped: on_person
    rats:
      types:
      - ordinary_rat
      - giant_rat
      attitude: indifferent
      attacks_user_only_if:
      - user_threatens
      - user_harms
    charges:
      initial: 3
      maximum: 3
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 3
        cap_at_maximum: true
    play:
      action: magic
      maintain_each_round: true
    call:
      action: bonus_action
      requires: playing_pipes
      charges_min: 1
      charges_max: 3
      swarm_per_charge: 1
      creature:
        key: dnd2024:creatures/swarm_of_rats
      available_rats_radius_miles: 0.5
      availability_choice: gm
      insufficient_rats: charge_wasted
      movement: shortest_route_to_music
      automatically_controlled: false
    control:
      trigger: swarm_enters_30_ft_while_playing
      ineligible: controlled_by_another
      save:
        ability: wisdom
        dc: 15
        failure:
          attitude: friendly_to_user_and_allies
          obeys_commands: true
          no_command_behavior: defend_self_otherwise_no_actions
          requires_continued_magic_action_each_round: true
        success:
          behavior: normal
          immunity_to_pipes:
            amount: 24
            unit: hours
      turn_start_distance_over_ft: 30
      out_of_range_result:
        end_control: true
        behavior: normal
        immunity_to_pipes:
          amount: 24
          unit: hours
    attunement_required: true
splat_contracts:
- id: dnd2024:item_pipes_of_the_sewers
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - movement.completed
  - turn.started
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c17281f9f42180ce5da90d9be9f37c1a0986dd7aefaf36c6e94318965b07435a
  source_path: builtin/srd-5.2.1/magic_item/pipes_of_the_sewers.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_pipes_of_the_sewers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: e5edbdd155037cdde5a02a1e5f4233ad7bea620520310409b5b334c498f44470
splat_clauses:
- key: source_rules
  text: '# Pipes of the Sewers Wondrous Item, Uncommon (Requires Attunement) While these pipes are on
    your person, ordinary rats and giant rats are Indifferent toward you and won’t attack you unless you
    threaten or harm them. The pipes have 3 charges and regain 1d3 expended charges daily at dawn. If
    you play the pipes as a Magic action, you can take a Bonus Action to expend 1 to 3 charges, calling
    forth one Swarm of Rats with each expended charge if enough rats are within half a mile of you to
    be called in this fashion (as determined by the GM). If there aren’t enough rats to form a swarm,
    the charge is wasted. Called swarms move toward the music by the shortest available route but aren’t
    under your control otherwise. Whenever a Swarm of Rats that isn’t under another creature’s control
    comes within 30 feet of you while you are playing the pipes, the swarm makes a DC 15 Wisdom saving
    throw. On a successful save, the swarm behaves as it normally would and can’t be swayed by the pipes’
    music for the next 24 hours. On a failed save, the swarm is swayed by the pipes’ music and becomes
    Friendly to you and your allies for as long as you continue to play the pipes each round as a Magic
    action. A Friendly swarm obeys your commands. If you issue no commands to a Friendly swarm, it defends
    itself but otherwise takes no actions. If a Friendly swarm starts its turn more than 30 feet away
    from you, your control over that swarm ends, and the swarm behaves as it normally would and can’t
    be swayed by the pipes’ music for the next 24 hours.'
  sha256: 71641bcef375865e84c1ca8d459c614c75aef9db20b506b55d2013edd7d08052
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pipes of the Sewers

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement)\
While these pipes are on your person, ordinary rats\
and giant rats are Indifferent toward you and won’t\
attack you unless you threaten or harm them.\
The pipes have 3 charges and regain 1d3 ex-\
pended charges daily at dawn. If you play the pipes\
as a Magic action, you can take a Bonus Action to\
expend 1 to 3 charges, calling forth one Swarm\
of Rats with each expended charge if enough rats\
are within half a mile of you to be called in this\
fashion (as determined by the GM). If there aren’t\
enough rats to form a swarm, the charge is wasted.\
Called swarms move toward the music by the short-\
est available route but aren’t under your control\
otherwise.\
Whenever a Swarm of Rats that isn’t under an-\
other creature’s control comes within 30 feet of you\
while you are playing the pipes, the swarm makes\
a DC 15 Wisdom saving throw. On a successful\
save, the swarm behaves as it normally would and\
can’t be swayed by the pipes’ music for the next 24\
hours. On a failed save, the swarm is swayed by the\
pipes’ music and becomes Friendly to you and your\
allies for as long as you continue to play the pipes\
each round as a Magic action. A Friendly swarm\
obeys your commands. If you issue no commands\
to a Friendly swarm, it defends itself but otherwise\
takes no actions. If a Friendly swarm starts its turn\
more than 30 feet away from you, your control over\
that swarm ends, and the swarm behaves as it nor-\
mally would and can’t be swayed by the pipes’ mu-\
sic for the next 24 hours.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
