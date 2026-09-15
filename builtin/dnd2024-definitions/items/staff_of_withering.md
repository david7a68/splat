---
splat_schema: 1
splat_id: 3aff3905-62ec-5e33-8fca-66e81225000a
splat_kind: item
splat_key: dnd2024:items/staff_of_withering
splat_name: Staff of Withering
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_withering
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/staff_of_withering
  implementation: dnd2024:item_staff_of_withering
  parameters:
    weapon:
      key: dnd2024:equipment/quarterstaff
    magical_weapon: true
    base_damage: normal_quarterstaff
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
    on_hit:
      optional_charge_cost: 1
      damage:
        amount:
          roll:
            count: 2
            sides: 10
        type: necrotic
      save:
        ability: constitution
        dc: 15
        failure:
          disadvantage:
            test_kinds:
            - ability_check
            - saving_throw
            abilities:
            - strength
            - constitution
          duration:
            amount: 1
            unit: hours
    attunement_required: true
splat_contracts:
- id: dnd2024:item_staff_of_withering
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
  - attack.hit
  - time.dawn
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_charges
  - effect_duration
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e9a9c9e0fc33e2495955e941fda54588cd0152aa03c898013a794a9a66c86bf6
  source_path: builtin/srd-5.2.1/magic_item/staff_of_withering.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_staff_of_withering
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 0f7ef06a242441ddb8469b68b2c656f590df879d25a49df76cebfb5b796454bf
splat_clauses:
- key: source_rules
  text: '# Staff of Withering Staff, Rare (Requires Attunement) This staff has 3 charges and regains 1d3
    expended charges daily at dawn. The staff can be wielded as a magic Quarterstaff. On a hit, it deals
    damage as a normal Quarterstaff, and you can expend 1 charge to deal an extra 2d10 Necrotic damage
    to the target and force it to make a DC 15 Constitution saving throw. On a failed save, the target
    has Disadvantage for 1 hour on any ability check or saving throw that uses Strength or Constitution.'
  sha256: 454b34878745ab4ad4ad03da60d1dacf3b9f1ca1e143c214dd33be63a658cb25
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Withering

<!-- SRD page 248 -->

Staff, Rare (Requires Attunement)\
This staff has 3 charges and regains 1d3 expended\
charges daily at dawn.\
The staff can be wielded as a magic Quarterstaff.\
On a hit, it deals damage as a normal Quarterstaff,\
and you can expend 1 charge to deal an extra 2d10\
Necrotic damage to the target and force it to make\
a DC 15 Constitution saving throw. On a failed\
save, the target has Disadvantage for 1 hour on any\
ability check or saving throw that uses Strength or\
Constitution.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
