---
splat_schema: 1
splat_id: 2f057967-273c-532a-9bac-bb9ac8602109
splat_kind: item
splat_key: dnd2024:items/wand_of_paralysis
splat_name: Wand of Paralysis
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_paralysis
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Spellcaster
splat_rule:
  mechanic: dnd2024:items/wand_of_paralysis
  implementation: dnd2024:item_wand_of_paralysis
  parameters:
    equipment_state: held
    charges:
      maximum: 7
      initial: 7
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 1
    activation:
      action: magic
      cost: 1
      target:
        kind: creature
        seen: true
        range_ft: 60
      save:
        ability: constitution
        dc: 15
      failure:
        condition: paralyzed
        duration:
          value: 1
          unit: minute
      repeat_save: target_turn_end
      success: end_condition
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: destroy_item
    attunement:
      required: true
      eligibility:
        spellcaster: true
splat_contracts:
- id: dnd2024:item_wand_of_paralysis
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7d27be13385a4398c20c697ffe5ff08c03cac5aa6616e07fed362ae662d7eab0
  source_path: builtin/srd-5.2.1/magic_item/wand_of_paralysis.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_paralysis
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 453cedbb5944c76f5807cbb517d9e3557d0d5a04c819b3a81cf94a0a41f91b46
splat_clauses:
- key: source_rules
  text: '# Wand of Paralysis Wand, Rare (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can take a Magic action to expend 1 charge to cause a thin blue ray to streak
    from the tip toward a creature you can see within 60 feet of yourself. The target must succeed on
    a DC 15 Constitution saving throw or have the Paralyzed condition for 1 minute. At the end of each
    of the target’s turns, it repeats the save, ending the effect on itself on a success. Regaining Charges.
    The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll
    1d20. On a 1, the wand crumbles into ashes and is destroyed.'
  sha256: 64c575aad66fa144161285a353f5fe0636828d110f50944bcbecf4ff624ad2c1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Paralysis

<!-- SRD page 251 -->

Wand, Rare (Requires Attunement by a Spellcaster)\
This wand has 7 charges. While holding it, you can\
take a Magic action to expend 1 charge to cause a\
thin blue ray to streak from the tip toward a crea-\
ture you can see within 60 feet of yourself. The\
target must succeed on a DC 15 Constitution saving\
throw or have the Paralyzed condition for 1 minute.\
At the end of each of the target’s turns, it repeats\
the save, ending the effect on itself on a success.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
