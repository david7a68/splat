---
splat_schema: 1
splat_id: 754d424d-1ef9-597c-b4de-d820ef107624
splat_kind: item
splat_key: dnd2024:items/iron_bands
splat_name: Iron Bands
splat_origin:
  key: dnd2024:reference/magic_item/iron_bands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/iron_bands
  implementation: dnd2024:item_iron_bands
  parameters:
    dimensions:
      diameter_inches: 3
      weight_lb: 1
    activation:
      action: magic
      operation: throw
      target:
        maximum_size: huge
        seen: true
        range_ft: 60
      attack:
        kind: ranged
        bonus:
          sum:
          - value_from: dexterity_modifier
          - value_from: proficiency_bonus
      on_hit:
        condition: restrained
        duration: until_released_or_broken
      on_miss: revert_to_sphere
      resource:
        uses: 1
        recovery: dawn
    release:
      actor: user
      action: bonus_action
      command: true
      effects:
      - end_restraint
      - revert_to_sphere
    break_attempt:
      actor: any_creature_touching_bands
      action: action
      check:
        ability: strength
        skill: athletics
        dc: 20
      success:
      - destroy_item
      - free_creature
      failure:
        actor_further_attempts: automatic_failure
        duration:
          value: 24
          unit: hour
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_iron_bands
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7b3c34eac06e1e0b89b9a1e8bcba80b60cebf722fb4724db94be950144fa0c37
  source_path: builtin/srd-5.2.1/magic_item/iron_bands.md
  pages:
  - 228
  engine_gaps: &id001
  - dnd2024:item_iron_bands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: f38ee8a179422ac3e9a8def23f25ec3f3413d02ace1f457c7f2e5a3e0b9eb77d
splat_clauses:
- key: source_rules
  text: '# Iron Bands Wondrous Item, Rare This rusty iron sphere measures 3 inches in diameter and weighs
    1 pound. You can take a Magic action to throw the sphere at a Huge or smaller creature you can see
    within 60 feet of yourself. As the sphere moves through the air, it opens into a tangle of metal bands.
    Make a ranged attack roll with an attack bonus equal to your Dexterity modifier plus your Proficiency
    Bonus. On a hit, the target has the Restrained condition until you take a Bonus Action to issue a
    command that releases it. Doing so or missing with the attack causes the bands to contract and become
    a sphere once more. A creature that can touch the bands, including the one Restrained, can take an
    action to make a DC 20 Strength (Athletics) check to break the iron bands. On a successful check,
    the item is destroyed, and the Restrained creature is freed. On a failed check, any further attempts
    made by that creature automatically fail until 24 hours have elapsed. Once the bands are used, they
    can’t be used again until the next dawn.'
  sha256: 2ff387019b3fb8c471b65c5a5ee04e95dcf276885c1c9b670bee70dd9db15045
  status: contract_specified
splat_requires_mechanics: *id001
---

# Iron Bands

<!-- SRD page 228 -->

Wondrous Item, Rare\
This rusty iron sphere measures 3 inches in diame-\
ter and weighs 1 pound. You can take a Magic action\
to throw the sphere at a Huge or smaller creature\
you can see within 60 feet of yourself. As the sphere\
moves through the air, it opens into a tangle of\
metal bands.\
Make a ranged attack roll with an attack bonus\
equal to your Dexterity modifier plus your Profi-\
ciency Bonus. On a hit, the target has the Restrained\
condition until you take a Bonus Action to issue a\
command that releases it. Doing so or missing with\
the attack causes the bands to contract and become\
a sphere once more.\
A creature that can touch the bands, including the\
one Restrained, can take an action to make a DC 20\
Strength (Athletics) check to break the iron bands.\
On a successful check, the item is destroyed, and the\
Restrained creature is freed. On a failed check, any\
further attempts made by that creature automati-\
cally fail until 24 hours have elapsed.\
Once the bands are used, they can’t be used again\
until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]].
