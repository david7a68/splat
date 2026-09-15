---
splat_schema: 1
splat_id: 7aaa5e38-3e00-595b-93f8-40c0e540003d
splat_kind: item
splat_key: dnd2024:items/dust_of_disappearance
splat_name: Dust of Disappearance
splat_origin:
  key: dnd2024:reference/magic_item/dust_of_disappearance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dust_of_disappearance
  implementation: dnd2024:item_dust_of_disappearance
  parameters:
    action: utilize
    consumed_on: effect_start
    area:
      shape: emanation
      radius_ft: 10
      origin: actor
      targets:
      - actor
      - creatures
      - objects
    effect:
      condition: invisible
      duration:
        amount:
          roll:
            count: 2
            sides: 4
        unit: minutes
      duration_roll_shared: true
      ends_for_each_creature_immediately_after:
      - attack_roll
      - damage_dealt
      - spell_cast
    attunement_required: false
splat_contracts:
- id: dnd2024:item_dust_of_disappearance
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
  - attack.rolled
  - damage.resolved
  - spell.cast_completed
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - effect_duration
  - spatial_targeting
  - damage
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8cfafab9b6c50ac8a70bff6e63ebe9acca7d6a9a42cde453a68835fd2437db07
  source_path: builtin/srd-5.2.1/magic_item/dust_of_disappearance.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dust_of_disappearance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 0fe77e7f71ebbca0fb088739a6699c73d82ec95ba1cd09a21a54f62d6ccf25e7
splat_clauses:
- key: source_rules
  text: '# Dust of Disappearance Wondrous Item, Uncommon This powder resembles fine sand. There is enough
    of it for one use. When you take a Utilize action to throw the dust into the air, you and each creature
    and object within a 10-foot Emanation originating from you have the Invisible condition for 2d4 minutes.
    The duration is the same for all subjects, and the dust is consumed when its magic takes effect. Immediately
    after an affected creature makes an attack roll, deals damage, or casts a spell, the Invisible condition
    ends for that creature.'
  sha256: baa2f83c4ad8bf7286fb3779c84b157957778a232ab535d905c38040ade39377
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dust of Disappearance

<!-- SRD page 219 -->

Wondrous Item, Uncommon\
This powder resembles fine sand. There is enough\
of it for one use. When you take a Utilize action to\
throw the dust into the air, you and each creature\
and object within a 10-foot Emanation originating\
from you have the Invisible condition for 2d4 min-\
utes. The duration is the same for all subjects, and\
the dust is consumed when its magic takes effect.\
Immediately after an affected creature makes an at-\
tack roll, deals damage, or casts a spell, the Invisible\
condition ends for that creature.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
